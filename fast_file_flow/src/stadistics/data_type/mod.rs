use core::mem::discriminant as tag;

#[derive(Debug, Eq, Hash, Clone)]
pub enum DataType {
    Integer,
    Float,
    Text,
    Date,
    Time,
    DateTime,
    Coordinates,
    Unknown,
}

impl DataType {
    pub fn to_string(&self) -> &str {
        match self {
            DataType::Integer => "Integer",
            DataType::Float => "Float",
            DataType::Text => "Text",
            DataType::Unknown => "Unknown",
            DataType::Date => "Date",
            DataType::Time => "Time",
            DataType::DateTime => "DateTime",
            DataType::Coordinates => "Coordinates",
        }
    }
}

impl PartialEq for DataType {
    fn eq(&self, other: &Self) -> bool {
        tag(self) == tag(other)
    }
}
