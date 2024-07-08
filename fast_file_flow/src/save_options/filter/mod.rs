use crate::option::Option;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FilterOption {
    pub ignore_if_empty: bool,
    pub ignore_column: bool,
}
impl FilterOption {
    pub fn default() -> Self {
        Self {
            ignore_if_empty: bool::default(),
            ignore_column: bool::default(),
        }
    }
    pub fn new(ignore_if_empty: bool, ignore_column: bool) -> Self {
        Self {
            ignore_if_empty,
            ignore_column,
        }
    }
}

impl Option for FilterOption {
    fn is_dirty(&self) -> bool {
        self.ignore_column != bool::default() || self.ignore_if_empty != bool::default()
    }
}
