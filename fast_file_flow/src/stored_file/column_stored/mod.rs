use crate::dynamictable::IcedColumn;

#[derive(Debug, Clone)]
pub struct ColumnStored {
    pub total: u64,
    pub headers: Vec<IcedColumn>,
}

impl ColumnStored {
    pub fn empty() -> Self {
        Self {
            total: 0,
            headers: vec![],
        }
    }
    pub fn new(total: u64, headers: Vec<IcedColumn>) -> Self {
        Self { total, headers }
    }
}
