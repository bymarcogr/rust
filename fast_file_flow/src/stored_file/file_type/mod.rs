use core::mem::discriminant as tag;

#[derive(Debug, Clone)]
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
impl PartialEq<Self> for FileType {
    fn eq(&self, rhs: &Self) -> bool {
        tag(self) == tag(rhs)
    }
}
