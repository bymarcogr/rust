use crate::constants::english::{
    DATA_CLASSIFICATION_QUALITATIVE, DATA_CLASSIFICATION_QUANTITATIVE, UNKNOWN_LABEL,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum DataClassification {
    Unknown = 0,
    Qualitative = 1,
    Quantitative = 2,
}

impl DataClassification {
    pub fn to_string(&self) -> &str {
        match self {
            DataClassification::Qualitative => DATA_CLASSIFICATION_QUALITATIVE,
            DataClassification::Quantitative => DATA_CLASSIFICATION_QUANTITATIVE,
            DataClassification::Unknown => UNKNOWN_LABEL,
        }
    }

    pub fn default() -> Self {
        DataClassification::Unknown
    }

    pub fn from_string(unwrap: &str) -> DataClassification {
        match unwrap {
            DATA_CLASSIFICATION_QUALITATIVE => DataClassification::Qualitative,
            DATA_CLASSIFICATION_QUANTITATIVE => DataClassification::Quantitative,
            _ => DataClassification::Unknown,
        }
    }
}
