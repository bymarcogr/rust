#[derive(Debug, Clone)]
pub struct IcedRow {
    pub values: Vec<String>,
    pub is_enabled: bool,
    pub row_index: usize,
}

impl IcedRow {
    pub fn empty() -> Self {
        Self {
            is_enabled: true,
            values: vec![],
            row_index: 0,
        }
    }
    pub fn new(values: Vec<String>, row: usize) -> Self {
        Self {
            is_enabled: true,
            values,
            row_index: row,
        }
    }
}
