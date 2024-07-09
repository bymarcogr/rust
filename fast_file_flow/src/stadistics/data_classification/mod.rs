#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum DataClassification {
    Unknown = 0,
    Qualitative = 1,
    Quantitative = 2,
}

impl DataClassification {
    pub fn to_string(&self) -> &str {
        match self {
            DataClassification::Qualitative => "Qualitative",
            DataClassification::Quantitative => "Quantitative",
            DataClassification::Unknown => "Unknown",
        }
    }

    pub fn default() -> Self {
        DataClassification::Unknown
    }

    pub fn from_string(unwrap: &str) -> DataClassification {
        match unwrap {
            "Qualitative" => DataClassification::Qualitative,
            "Quantitative" => DataClassification::Quantitative,
            _ => DataClassification::Unknown,
        }
    }
}
