use crate::stadistics::data_classification::DataClassification;

pub struct SimpleColumn {
    pub index: usize,
    pub header: String,
    pub classification: DataClassification,
}

impl SimpleColumn {}
