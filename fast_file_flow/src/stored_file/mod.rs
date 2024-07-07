pub mod column_stored;
pub mod file_type;
pub mod row_stored;

use crate::{
    constants::path::CSV,
    correlation_analysis::CorrelationAnalysis,
    dynamictable::{iced_column::IcedColumn, iced_row::IcedRow},
    stadistics::{data_classification::DataClassification, Stadistics},
};
use chardet::detect;
use column_stored::ColumnStored;
use csv_async::AsyncReaderBuilder;
use file_type::FileType;
use futures::stream::StreamExt;
use rayon::prelude::*;
use row_stored::RowStored;
use serde_json::Value;
use std::fmt::{Debug, Error};
use std::{fs::metadata, io::Cursor, path::Path};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
    time::Instant,
};
#[derive(Debug, Clone)]
pub struct StoredFile {
    pub file_path: String,
    pub file_name: String,
    pub encoding: String,
    pub size: f64,
    pub format: String,
    pub sintaxis: FileType,
    pub rows: RowStored,
    pub columns: ColumnStored,
}

impl StoredFile {
    pub fn default() -> Self {
        Self {
            file_path: String::new(),
            file_name: String::new(),
            encoding: String::new(),
            size: 0.0,
            format: String::new(),
            sintaxis: FileType::Unknown,
            rows: RowStored::empty(),
            columns: ColumnStored::empty(),
        }
    }

    pub async fn new(file_path: String) -> Self {
        let format = Self::get_file_extension(&file_path);
        let sintaxis = Self::detect_file_type(&file_path).await;
        if format != CSV || sintaxis != FileType::CSV {
            Self {
                file_path: file_path.clone(),
                file_name: Self::get_file_name(&file_path),
                encoding: Self::get_encoding(&file_path).await,
                size: Self::get_size_kb(&file_path),
                format,
                rows: RowStored::empty(),
                columns: ColumnStored::empty(),
                sintaxis,
            }
        } else {
            Self {
                file_path: file_path.clone(),
                file_name: Self::get_file_name(&file_path),
                encoding: Self::get_encoding(&file_path).await,
                size: Self::get_size_kb(&file_path),
                format: format,
                rows: Self::get_rows(&file_path).await,
                columns: Self::get_columns(&file_path).await,
                sintaxis: sintaxis,
            }
        }
    }

    fn get_size_kb(file_path: &str) -> f64 {
        metadata(&file_path).map(|m| m.len()).unwrap() as f64 / 1024.0
    }

    fn get_file_name(file_path: &str) -> String {
        let path = Path::new(file_path);
        let mut file_name = String::from("");

        if let Some(filename) = path.file_name() {
            file_name = filename.to_string_lossy().into_owned()
        }

        file_name.to_owned()
    }

    fn get_file_extension(file_path: &str) -> String {
        let path = Path::new(file_path);
        let mut file_extension = String::from("");

        if let Some(extension) = path.extension() {
            file_extension = extension.to_string_lossy().into_owned()
        }

        file_extension.to_uppercase().to_owned()
    }

    async fn get_columns(file_path: &str) -> ColumnStored {
        let mut rdr = csv_async::AsyncReader::from_reader(File::open(file_path).await.unwrap());
        let counter = rdr.headers().await.unwrap().into_iter().count() as u64;
        let headers_vec: Vec<IcedColumn> = rdr
            .headers()
            .await
            .unwrap()
            .clone()
            .iter()
            .map(|s| IcedColumn::new(s.to_string()))
            .collect();

        ColumnStored::new(counter, headers_vec.clone())
    }

    #[warn(unused_assignments)]
    async fn get_rows(file_path: &str) -> RowStored {
        let start = Instant::now();
        let mut rdr = csv_async::AsyncReader::from_reader(File::open(file_path).await.unwrap());
        let mut rdr_count =
            csv_async::AsyncReader::from_reader(File::open(file_path).await.unwrap());

        let handle_count = tokio::spawn(async move {
            let count = rdr_count.records().count().await;
            count as u64
        });

        let handle_records = tokio::spawn(async move {
            let mut records_vec = Vec::new();
            let mut row_index = 0;
            let mut records = rdr.records();

            while let Some(record) = records.next().await {
                if row_index >= 50 {
                    break;
                }
                let record = record.unwrap();
                let values: Vec<String> = record.iter().map(|s| s.to_string()).collect();
                records_vec.push(IcedRow::new(values, row_index));
                row_index += 1;
            }
            records_vec
        });

        let counter = handle_count.await.unwrap();
        let records_vec = handle_records.await.unwrap();
        let duration = start.elapsed();
        println!(
            "Rows {} Execution time: {:?}",
            counter,
            duration.as_secs_f64()
        );
        RowStored::new(counter, records_vec)
    }

    pub fn size_mb_as_str(&self) -> String {
        format!("{:.2} MB", self.size / 1024.0)
    }

    async fn get_encoding(file_path: &str) -> String {
        let mut file = File::open(file_path).await.unwrap();
        let mut buffer = vec![0; 4096];
        file.read(&mut buffer).await.unwrap();

        let result = detect(&buffer);
        let encoding = result.0;

        encoding.to_uppercase()
    }

    async fn detect_file_type(file_path: &str) -> FileType {
        let file = File::open(file_path).await.unwrap();
        let mut buf_reader = BufReader::new(file);

        let mut buffer = String::new();
        for _ in 0..100 {
            let bytes_read = buf_reader.read_line(&mut buffer).await.unwrap();
            if bytes_read == 0 {
                break;
            }
        }

        if serde_json::from_str::<Value>(&buffer).is_ok() {
            return FileType::JSON;
        }
        let cursor = Cursor::new(buffer);
        let mut rdr = AsyncReaderBuilder::new().create_reader(cursor);
        let mut records = rdr.records();

        if records.next().await.is_some() {
            return FileType::CSV;
        }

        FileType::Unknown
    }

    pub async fn get_full_column(&self, column_index: &usize) -> Vec<String> {
        let mut rdr =
            csv_async::AsyncReader::from_reader(File::open(&self.file_path).await.unwrap());
        let index: usize = *column_index;
        let handle_records = tokio::spawn(async move {
            let mut records_vec = Vec::new();
            let mut records = rdr.records();

            while let Some(record) = records.next().await {
                let value = record.unwrap().get(index).unwrap().to_string();
                records_vec.push(value);
            }
            records_vec
        });

        let records_vec = handle_records.await.unwrap();

        records_vec
    }

    // pub async fn get_full_column_generic<T>(&self, column_index: &usize) -> Vec<T>
    // where
    //     T: FromStr + Send + 'static,
    //     <T as FromStr>::Err: Debug,
    // {
    //     let mut rdr =
    //         csv_async::AsyncReader::from_reader(File::open(&self.file_path).await.unwrap());
    //     let index: usize = *column_index;

    //     let handle_records = spawn(async move {
    //         let mut records_vec = Vec::new();
    //         let mut records = rdr.records();

    //         while let Some(record) = records.next().await {
    //             let value = record.unwrap().get(index).unwrap().to_string();

    //             if value.is_empty() {
    //                 records_vec.push(0);
    //             } else {
    //                 let parsed_value: T = value.parse().unwrap();
    //                 records_vec.push(parsed_value);
    //             }
    //         }
    //         records_vec
    //     });

    //     handle_records.await.unwrap()
    // }

    pub async fn get_stadistics(&self, column_index: &usize) -> Stadistics {
        Stadistics::new(
            self.columns.headers.get(column_index.clone()).unwrap(),
            self.get_full_column(column_index).await,
        )
        .await
    }

    pub async fn get_correlation(
        &self,
        column_base: &usize,
        column_compare: &usize,
    ) -> Result<CorrelationAnalysis, &'static str> {
        let column_base_class = self.get_column_class(column_base).await;
        let column_compare_class = self.get_column_class(column_compare).await;
        print!(
            "{} y {}",
            column_base_class.to_string(),
            column_compare_class.to_string()
        );
        let base = Self::convert_to_f64(&self.get_full_column(column_base).await);
        let compare = Self::convert_to_f64(&self.get_full_column(column_compare).await);
        if column_base_class == column_compare_class
            && column_compare_class == DataClassification::Quantitative
        {
            Ok(
                CorrelationAnalysis::new(&base, column_base_class, &compare, column_compare_class)
                    .await,
            )
        } else {
            Err("Ambas columnas deben ser de tipo { DataClassification::Quantitative}")
        }
    }

    async fn get_column_class(&self, column_base: &usize) -> DataClassification {
        let current_stadistics = self
            .columns
            .headers
            .get(column_base.clone())
            .unwrap()
            .stadistics
            .clone();
        let current_class = current_stadistics.classification;
        if current_class == DataClassification::Unknown {
            let (current_class, _) =
                Stadistics::get_column_analysis(&self.get_full_column(column_base).await);
            current_class
        } else {
            current_class
        }
    }

    fn convert_to_f64(vec: &Vec<String>) -> Vec<f64> {
        vec.par_iter()
            .map(|s| s.parse::<f64>().unwrap_or(0.0))
            .collect()
    }
}
