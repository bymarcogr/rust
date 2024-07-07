use crate::stadistics::data_classification::DataClassification;

#[derive(Debug, Hash, Clone)]
pub struct SimpleColumn {
    pub index: usize,
    pub header: String,
    pub classification: DataClassification,
}

impl SimpleColumn {}
