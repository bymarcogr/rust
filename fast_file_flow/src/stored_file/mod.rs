pub mod column_stored;
pub mod file_type;
pub mod row_stored;

use crate::{
    ai::{
        dbscan::DensityBaseClustering, k_means::KMeansClustering, pca::PrincipalComponentsAnalisys,
    },
    constants::path::CSV,
    correlation_analysis::CorrelationAnalysis,
    dynamictable::{iced_column::IcedColumn, iced_row::IcedRow, simple_column::SimpleColumn},
    save_options::SaveOptions,
    stadistics::Stadistics,
};
use chardet::detect;
use column_stored::ColumnStored;
use csv_async::AsyncReaderBuilder;
use file_type::FileType;
use futures::stream::StreamExt;
use rayon::prelude::*;
use row_stored::RowStored;
use serde_json::Value;
use std::{fs::metadata, io::Cursor, path::Path, time::Instant};
use std::{
    io::Error,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
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
    pub k_means: KMeansClustering,
    pub principal_components_analisys: PrincipalComponentsAnalisys,
    pub density_base_clustering: DensityBaseClustering,
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
            k_means: KMeansClustering::new(),
            principal_components_analisys: PrincipalComponentsAnalisys::new(),
            density_base_clustering: DensityBaseClustering::new(),
        }
    }

    pub async fn new(file_path: String) -> Self {
        let format = Self::get_file_extension(&file_path);
        let sintaxis = Self::detect_file_type(&file_path).await;
        let k_means = KMeansClustering::new();
        let pca = PrincipalComponentsAnalisys::new();
        let db_scan = DensityBaseClustering::new();

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
                k_means,
                principal_components_analisys: pca,
                density_base_clustering: db_scan,
            }
        } else {
            Self {
                file_path: file_path.clone(),
                file_name: Self::get_file_name(&file_path),
                encoding: Self::get_encoding(&file_path).await,
                size: Self::get_size_kb(&file_path),
                format: format,
                rows: Self::get_rows(&file_path).await.unwrap(),
                columns: Self::get_columns(&file_path).await.unwrap(),
                sintaxis: sintaxis,
                k_means,
                principal_components_analisys: pca,
                density_base_clustering: db_scan,
            }
        }
    }

    pub async fn reload(&mut self) -> Result<(), Error> {
        self.rows = match StoredFile::get_rows(&self.file_path.to_string()).await {
            Ok(it) => it,
            Err(err) => return Err(err),
        };

        self.columns = match StoredFile::get_columns(&self.file_path.to_string()).await {
            Ok(it) => it,
            Err(err) => return Err(err),
        };

        self.k_means = KMeansClustering::new();
        self.principal_components_analisys = PrincipalComponentsAnalisys::new();
        self.density_base_clustering = DensityBaseClustering::new();
        Ok(())
    }

    pub fn get_simple_columns(&self) -> Vec<SimpleColumn> {
        if self.columns.total > 0 {
            let simple_column: Vec<SimpleColumn> = self
                .columns
                .headers
                .iter()
                .enumerate()
                .map(|(index, item)| SimpleColumn {
                    index: index,
                    header: item.column_header.clone(),
                    classification: item.stadistics.classification.clone(),
                    save_options: SaveOptions::default(),
                })
                .collect();

            simple_column
        } else {
            vec![]
        }
    }

    fn get_size_kb(file_path: &str) -> f64 {
        metadata(&file_path).map(|m| m.len()).unwrap() as f64 / 1024.0
    }

    pub fn get_file_name(file_path: &str) -> String {
        let path = Path::new(file_path);
        let mut file_name = String::from("");

        if let Some(filename) = path.file_name() {
            file_name = filename.to_string_lossy().into_owned()
        }

        file_name.to_owned()
    }

    pub fn get_file_extension(file_path: &str) -> String {
        let path = Path::new(file_path);
        let mut file_extension = String::from("");

        if let Some(extension) = path.extension() {
            file_extension = extension.to_string_lossy().into_owned()
        }

        file_extension.to_uppercase().to_owned()
    }

    pub async fn get_columns(file_path: &str) -> Result<ColumnStored, Error> {
        let mut rdr = csv_async::AsyncReader::from_reader(File::open(file_path).await?);
        let counter = rdr.headers().await?.into_iter().count() as u64;
        let headers_vec: Vec<IcedColumn> = rdr
            .headers()
            .await
            .unwrap()
            .clone()
            .iter()
            .map(|s| IcedColumn::new(s.to_string()))
            .collect();
        Ok(ColumnStored::new(counter, headers_vec.clone()))
    }

    #[warn(unused_assignments)]
    pub async fn get_rows(file_path: &str) -> Result<RowStored, Error> {
        let start = Instant::now();

        let file = match File::open(file_path).await {
            Ok(it) => it,
            Err(err) => return Err(err),
        };
        let file_count = File::open(file_path).await.unwrap();

        let mut rdr = csv_async::AsyncReader::from_reader(file);
        let mut rdr_count = csv_async::AsyncReader::from_reader(file_count);

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

        let message = format!("Loading Rows {:?}", counter);
        crate::util::print_timer(&message, start);

        Ok(RowStored::new(counter, records_vec))
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
        let mut total_bytes_read = 0;

        // Read lines until we have enough content to determine file type or reach 100 lines
        while total_bytes_read < 8192 {
            let bytes_read = buf_reader.read_line(&mut buffer).await.unwrap();
            if bytes_read == 0 {
                break;
            }
            total_bytes_read += bytes_read;
        }

        if serde_json::from_str::<Value>(&buffer).is_ok() {
            return FileType::JSON;
        }

        let cursor = Cursor::new(buffer);
        let mut rdr = AsyncReaderBuilder::new().create_reader(cursor);
        if rdr.records().next().await.is_some() {
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

    pub async fn get_stadistics(&self, column_index: &usize) -> Stadistics {
        Stadistics::new(
            self.columns.headers.get(column_index.clone()).unwrap(),
            self.get_full_column(column_index).await,
        )
        .await
    }

    pub async fn get_correlation(
        &self,
        column_base: &SimpleColumn,
        column_compare: &SimpleColumn,
    ) -> Result<CorrelationAnalysis, &'static str> {
        let base = Self::convert_to_f64(&self.get_full_column(&column_base.index).await);
        let compare = Self::convert_to_f64(&self.get_full_column(&column_compare.index).await);
        Ok(CorrelationAnalysis::new(&base, &compare).await)
    }

    fn convert_to_f64(vec: &Vec<String>) -> Vec<f64> {
        vec.par_iter()
            .map(|s| s.parse::<f64>().unwrap_or(0.0))
            .collect()
    }

    pub fn get_export_path(&self) -> String {
        let path = Path::new(&self.file_path);
        let stem = path.file_stem().unwrap_or_default();
        let extension = path.extension().unwrap_or_default();
        let ticks = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut new_file_name = format!("{}_export_{}", stem.to_string_lossy(), ticks);
        if !extension.is_empty() {
            new_file_name.push_str(&format!(".{}", extension.to_string_lossy()));
        }

        let new_path = path.with_file_name(new_file_name);
        new_path.to_string_lossy().into_owned()
    }

    pub async fn get_kmeans(
        &mut self,
        column_base: &SimpleColumn,
        column_compare: &SimpleColumn,
        clusters: &usize,
        iteraciones: &u64,
    ) -> Result<String, &'static str> {
        let start = Instant::now();
        let (base, compare) = self.convert_columns_f64(column_base, column_compare).await;
        match self
            .k_means
            .get_prediction(base, compare, *clusters, *iteraciones)
            .await
        {
            Ok(s) => {
                crate::util::print_timer("K Means ", start);
                Ok(s)
            }
            Err(e) => {
                crate::util::print_timer("K Means Error ", start);
                let error_msg: &'static str = Box::leak(Box::new(String::from(e.to_string())));
                Err(error_msg)
            }
        }
    }
    pub async fn get_pca_analysis(
        &mut self,
        column_base: &SimpleColumn,
        column_compare: &SimpleColumn,
        embedding_size: usize,
    ) -> Result<String, &'static str> {
        let (base, compare) = self.convert_columns_f64(column_base, column_compare).await;
        let start = Instant::now();
        match self
            .principal_components_analisys
            .pca_analysis(base, compare, embedding_size)
            .await
        {
            Ok(s) => {
                crate::util::print_timer("Principal Component Analisys -", start);
                Ok(s)
            }
            Err(e) => {
                crate::util::print_timer("Principal Component Analisys Error -", start);
                let error_msg: &'static str = Box::leak(Box::new(String::from(e.to_string())));
                Err(error_msg)
            }
        }
    }

    async fn convert_columns_f64(
        &self,
        column_base: &SimpleColumn,
        column_compare: &SimpleColumn,
    ) -> (Vec<f64>, Vec<f64>) {
        let base = Self::convert_to_f64(&self.get_full_column(&column_base.index).await);
        let compare = Self::convert_to_f64(&self.get_full_column(&column_compare.index).await);
        (base, compare)
    }

    pub async fn get_dbscan_analysis(
        &mut self,
        column_base: &SimpleColumn,
        column_compare: &SimpleColumn,
        tolerance: f64,
        min_points: usize,
    ) -> Result<String, &'static str> {
        let (base, compare) = self.convert_columns_f64(column_base, column_compare).await;
        let start = Instant::now();
        match self
            .density_base_clustering
            .dbscan_analysis(base, compare, tolerance, min_points)
            .await
        {
            Ok(s) => {
                crate::util::print_timer(
                    "Density-Based Spatial Clustering of Applications with Noise Analisys -",
                    start,
                );
                Ok(s)
            }
            Err(e) => {
                crate::util::print_timer(
                    "Density-Based Spatial Clustering of Applications with Noise Error -",
                    start,
                );
                let error_msg: &'static str = Box::leak(Box::new(String::from(e.to_string())));
                Err(error_msg)
            }
        }
    }
}
