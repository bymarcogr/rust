#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum DataClassification {
    Unknown,
    Qualitative,
    Quantitative,
}

impl DataClassification {
    pub fn to_string(&self) -> &str {
        match self {
            DataClassification::Qualitative => "Qualitative",
            DataClassification::Quantitative => "Quantitative",
            DataClassification::Unknown => "Unknown",
        }
    }
}
