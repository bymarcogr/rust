use crate::dynamictable::IcedColumn;
use data_classification::{get_column_classification, DataClassification};
use data_type::{get_column_type, DataType};

pub mod data_classification;
pub mod data_type;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stadistics {
    pub index: u32,
    pub header: String,
    pub classification: DataClassification,
    pub data_type: DataType,
    pub distinct: String,      // numero diferentes
    pub most_repeated: String, // el mas repetetido
    pub minimum: String,       // Minimo si aplica
    pub maximum: String,       // Maximo si aplica
    pub mean: String,
    pub median: String,
    pub mode: String,
    pub range: String,
    pub variance: String,
    pub quartil: String,
    pub std_dev: String,
}

impl Stadistics {
    pub fn default() -> Self {
        Self {
            classification: DataClassification::Unknown,
            data_type: DataType::Unknown,
            distinct: String::default(),
            most_repeated: String::default(),
            minimum: String::default(),
            maximum: String::default(),
            mean: String::default(),
            median: String::default(),
            mode: String::default(),
            range: String::default(),
            variance: String::default(),
            quartil: String::default(),
            std_dev: String::default(),
            index: u32::default(),
            header: String::default(),
        }
    }
    pub async fn new(selected_column: &IcedColumn, full_column: Vec<String>) -> Self {
        Self {
            classification: get_column_classification(&full_column),
            data_type: get_column_type(&full_column),
            distinct: String::default(),
            most_repeated: String::default(),
            minimum: String::default(),
            maximum: String::default(),
            mean: String::default(),
            median: String::default(),
            mode: String::default(),
            range: String::default(),
            variance: String::default(),
            quartil: String::default(),
            std_dev: String::default(),
            index: 0,
            header: selected_column.column_header.clone(),
        }
    }
}
