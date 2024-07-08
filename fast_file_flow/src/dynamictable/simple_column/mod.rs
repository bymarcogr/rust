use crate::{save_options::SaveOptions, stadistics::data_classification::DataClassification};

#[derive(Debug, Hash, Clone)]
pub struct SimpleColumn {
    pub index: usize,
    pub header: String,
    pub classification: DataClassification,
    pub save_options: SaveOptions,
}

impl SimpleColumn {
    pub fn default() -> Self {
        Self {
            index: usize::default(),
            header: String::from(""),
            classification: DataClassification::default(),
            save_options: SaveOptions::default(),
        }
    }
}

impl std::fmt::Display for SimpleColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.header)
    }
}
impl Default for SimpleColumn {
    fn default() -> Self {
        SimpleColumn::default()
    }
}
