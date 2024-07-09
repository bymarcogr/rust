use core::mem::discriminant as tag;

#[derive(Debug, Clone)]
pub enum FileType {
    CSV,
    JSON,
    Unknown,
}

impl Default for FileType {
    fn default() -> Self {
        FileType::Unknown
    }
}

impl FileType {
    pub fn to_string(&self) -> &str {
        match self {
            FileType::CSV => "CSV",
            FileType::JSON => "JSON",
            FileType::Unknown => "Unknown",
        }
    }

    pub fn from_string(unwrap: &str) -> FileType {
        match unwrap {
            "CSV" => FileType::CSV,
            "JSON" => FileType::JSON,
            _ => FileType::Unknown, // Maneja todos los demás casos
        }
    }
}
impl PartialEq<Self> for FileType {
    fn eq(&self, rhs: &Self) -> bool {
        tag(self) == tag(rhs)
    }
}
