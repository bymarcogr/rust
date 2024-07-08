use crate::option::Option;
use filter::FilterOption;
pub mod filter;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SaveOptions {
    pub filter: FilterOption,
}

impl SaveOptions {
    pub fn default() -> Self {
        Self {
            filter: FilterOption::default(),
        }
    }
    pub fn new() -> Self {
        Self {
            filter: FilterOption::default(),
        }
    }
}

impl Option for SaveOptions {
    fn is_dirty(&self) -> bool {
        self.filter.is_dirty()
    }
}
