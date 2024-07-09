use crate::option::Option;
use filter::FilterOption;
use process::ProcessOption;
pub mod filter;
pub mod option_type;
pub mod process;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct SaveOptions {
    pub filter: FilterOption,
    pub process: ProcessOption,
}

impl SaveOptions {
    pub fn default() -> Self {
        Self {
            filter: FilterOption::default(),
            process: ProcessOption::default(),
        }
    }
    pub fn new() -> Self {
        Self {
            filter: FilterOption::default(),
            process: ProcessOption::default(),
        }
    }
}

impl Option for SaveOptions {
    fn is_dirty(&self) -> bool {
        self.filter.is_dirty() || self.process.is_dirty()
    }
}
