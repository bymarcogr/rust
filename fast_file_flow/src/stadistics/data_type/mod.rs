use core::mem::discriminant as tag;
use regex::Regex;
use std::collections::HashMap;

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

pub fn get_column_type(column: &Vec<String>) -> DataType {
    let date_re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    let time_re = Regex::new(r"^\d{2}:\d{2}(:\d{2})?$").unwrap();
    let datetime_re = Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}(:\d{2})?$").unwrap();
    let coordinates_re = Regex::new(r"^-?\d+\.\d+,\s*-?\d+\.\d+$").unwrap();

    let mut type_counts: HashMap<DataType, usize> = HashMap::new();

    for value in column {
        let data_type = if value.parse::<i64>().is_ok() {
            DataType::Integer
        } else if value.parse::<f64>().is_ok() {
            DataType::Float
        } else if date_re.is_match(&value) {
            DataType::Date
        } else if time_re.is_match(&value) {
            DataType::Time
        } else if datetime_re.is_match(&value) {
            DataType::DateTime
        } else if coordinates_re.is_match(&value) {
            DataType::Coordinates
        } else {
            DataType::Text
        };

        *type_counts.entry(data_type).or_insert(0) += 1;
    }

    type_counts
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(data_type, _)| data_type)
        .unwrap_or(DataType::Unknown)
}
