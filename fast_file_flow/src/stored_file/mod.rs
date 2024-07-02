use std::{
    borrow::Borrow,
    fs::metadata,
    path::{Path, PathBuf},
};

pub struct StoredFile {
    pub file_path: String,
    pub file_name: String,
    pub encoding: String,
    pub size: u64,
    pub format: String,
    pub sintaxis: String,
    pub rows_counter: u64,
    pub columns_counter: u64,
}

impl StoredFile {
    pub fn default() -> Self {
        Self {
            file_path: String::new(),
            file_name: String::new(),
            encoding: String::new(),
            size: 0,
            format: String::new(),
            sintaxis: String::new(),
            rows_counter: 0,
            columns_counter: 0,
        }
    }

    pub fn new(file_path: String) -> Self {
        Self {
            file_path: file_path.clone(),
            file_name: Self::get_file_name(&file_path),
            encoding: String::new(),
            size: Self::get_size_kb(&file_path),
            format: String::new(),
            sintaxis: String::new(),
            rows_counter: 0,
            columns_counter: 0,
        }
    }

    fn get_size_kb(file_path: &str) -> u64 {
        metadata(&file_path).map(|m| m.len()).unwrap() / 1024
    }

    fn get_file_name(file_path: &str) -> String {
        let path = Path::new(file_path);
        let mut file_name = String::from("");

        if let Some(filename) = path.file_name() {
            file_name = filename.to_string_lossy().into_owned()
        }

        file_name.to_owned()
    }

    pub fn size_as_str(&self) -> String {
        self.size.to_string().to_owned()
    }
}
