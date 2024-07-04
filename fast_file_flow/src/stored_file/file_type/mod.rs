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
