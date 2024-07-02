use chardet::detect;
use csv::{ReaderBuilder, Result};
use futures::{StreamExt, TryFutureExt};
use serde_json::Value;
use std::error::Error;
use std::{fs::metadata, path::Path};
use tokio::io;
use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
};

pub struct StoredFile {
    pub file_path: String,
    pub file_name: String,
    pub encoding: String,
    pub size: f64,
    pub format: String,
    pub sintaxis: FileType,
    pub rows_counter: u64,
    pub columns_counter: u64,
}

pub enum FileType {
    CSV,
    JSON,
    Unknown,
}

impl FileType {
    pub fn to_string(&self) -> &str {
        match self {
            FileType::CSV => "CSV",
            FileType::JSON => "JSON",
            FileType::Unknown => "Unknown",
        }
    }
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
            rows_counter: 0,
            columns_counter: 0,
        }
    }

    pub async fn new(file_path: String) -> Self {
        Self {
            file_path: file_path.clone(),
            file_name: Self::get_file_name(&file_path),
            encoding: Self::get_encoding(&file_path).await,
            size: Self::get_size_kb(&file_path),
            format: Self::get_file_extension(&file_path),
            sintaxis: Self::detect_file_type(&file_path).await,
            rows_counter: Self::get_rows_number(&file_path).await,
            columns_counter: Self::get_columns_number(&file_path).await,
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

    async fn get_columns_number(file_path: &str) -> u64 {
        let mut rdr =
            csv_async::AsyncReader::from_reader(tokio::fs::File::open(file_path).await.unwrap());
        rdr.headers().await.unwrap().into_iter().count() as u64
    }

    async fn get_rows_number(file_path: &str) -> u64 {
        let mut counter: u64 = 0;
        //let file = File::open(file_path).await.unwrap();

        let mut rdr =
            csv_async::AsyncReader::from_reader(tokio::fs::File::open(file_path).await.unwrap());

        let handle_spawn = tokio::spawn(async move {
            // let mut buffer = [0; 1024];
            // let mut reader = BufReader::new(file);

            // loop {
            //     let n = reader.read(&mut buffer).await.unwrap();
            //     println!("{}", n);
            //     if n == 0 {
            //         break;
            //     }
            // }
            let it = rdr.records().enumerate().count().await;
            it as u64
            // while rdr.records().next().await.is_some() {
            //     counter += 1;
            //     println!("{}", counter);
            // }
            // counter
        });
        counter = handle_spawn.await.unwrap();
        println!("Rows {}", counter);
        counter
    }

    pub fn size_mb_as_str(&self) -> String {
        format!("{:.2} MB", self.size / 1024.0)
    }

    async fn get_read_async_reader(file_path: &str) -> csv_async::AsyncReader<tokio::fs::File> {
        let mut rdr =
            csv_async::AsyncReader::from_reader(tokio::fs::File::open(file_path).await.unwrap());
        rdr
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
        let mut file = File::open(file_path).await.unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).await.unwrap();

        if serde_json::from_slice::<Value>(&buffer).is_ok() {
            return FileType::JSON;
        }

        let mut rdr: csv_async::AsyncReader<File> =
            csv_async::AsyncReader::from_reader(tokio::fs::File::open(file_path).await.unwrap());

        if rdr.records().next().await.is_some() {
            return FileType::CSV;
        }

        FileType::Unknown
    }
}
