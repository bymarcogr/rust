pub mod file_type;

use chardet::detect;
use csv_async::AsyncReaderBuilder;
use file_type::FileType;
use futures::stream::StreamExt;
use serde_json::Value;
use std::{fs::metadata, io::Cursor, path::Path};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
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
            rows_counter: Self::get_rows_number(&file_path).await,
            columns_counter: Self::get_columns_number(&file_path).await,
            sintaxis: Self::detect_file_type(&file_path).await,
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

    #[warn(unused_assignments)]
    async fn get_rows_number(file_path: &str) -> u64 {
        let mut counter: u64 = 0;
        let mut rdr =
            csv_async::AsyncReader::from_reader(tokio::fs::File::open(file_path).await.unwrap());

        let handle_spawn = tokio::spawn(async move {
            let it = rdr.records().enumerate().count().await;
            it as u64
        });
        counter = handle_spawn.await.unwrap();
        println!("Rows {}", counter);
        counter
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
}
