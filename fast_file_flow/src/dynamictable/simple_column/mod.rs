use crate::{save_options::SaveOptions, stadistics::data_classification::DataClassification};

#[derive(Debug, Hash, Clone)]
pub struct SimpleColumn {
    pub index: usize,
    pub header: String,
    pub classification: DataClassification,
    pub save_options: SaveOptions,
}

impl SimpleColumn {}
