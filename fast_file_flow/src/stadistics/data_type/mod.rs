use core::mem::discriminant as tag;

#[derive(Debug, Clone, Eq)]
pub enum DataType {
    Integer,
    Float,
    Text,
    Unknown,
}

impl DataType {
    pub fn to_string(&self) -> &str {
        match self {
            DataType::Integer => "Integer",
            DataType::Float => "Float",
            DataType::Text => "Text",
            DataType::Unknown => "Unknown",
        }
    }
}

impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        tag(self) == tag(other)
    }
}
