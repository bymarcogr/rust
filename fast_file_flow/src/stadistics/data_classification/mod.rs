use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum DataClassification {
    Unknown,
    Qualitative,
    Quantitative,
}

pub fn get_column_classification(column: &Vec<String>) -> DataClassification {
    let mut counts: HashMap<DataClassification, usize> = HashMap::new();
    for value in column {
        let data_type = if value.parse::<f64>().is_ok() {
            DataClassification::Quantitative
        } else {
            DataClassification::Qualitative
        };

        *counts.entry(data_type).or_insert(0) += 1;
    }

    if counts.get(&DataClassification::Quantitative).unwrap_or(&0)
        >= counts.get(&DataClassification::Qualitative).unwrap_or(&0)
    {
        DataClassification::Quantitative
    } else {
        DataClassification::Qualitative
    }
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
