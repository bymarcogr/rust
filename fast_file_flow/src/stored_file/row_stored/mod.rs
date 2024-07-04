use crate::dynamictable::IcedRow;

#[derive(Debug, Clone)]
pub struct RowStored {
    pub total: u64,
    pub sample: Vec<IcedRow>,
}

impl RowStored {
    pub fn empty() -> Self {
        Self {
            total: 0,
            sample: vec![],
        }
    }
    pub fn new(total: u64, sample: Vec<IcedRow>) -> Self {
        Self { total, sample }
    }
}
