use core::mem::discriminant as tag;

use crate::constants::english::{
    DIALOG_FILE_EXTENSION_CSV, DIALOG_FILE_EXTENSION_JSON, UNKNOWN_LABEL,
};

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
            FileType::CSV => DIALOG_FILE_EXTENSION_CSV,
            FileType::JSON => DIALOG_FILE_EXTENSION_JSON,
            FileType::Unknown => UNKNOWN_LABEL,
        }
    }

    pub fn from_string(unwrap: &str) -> FileType {
        match unwrap {
            DIALOG_FILE_EXTENSION_CSV => FileType::CSV,
            DIALOG_FILE_EXTENSION_JSON => FileType::JSON,
            _ => FileType::Unknown, // Maneja todos los demás casos
        }
    }
}
impl PartialEq<Self> for FileType {
    fn eq(&self, rhs: &Self) -> bool {
        tag(self) == tag(rhs)
    }
}
