use crate::option::Option;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FilterOption {
    pub ignore_if_empty: bool,
    pub ignore_column: bool,
    //pub ignore_fn: Fn(usize, bool, OptionType) -> bool,
}
impl FilterOption {
    pub fn default() -> Self {
        Self {
            ignore_if_empty: bool::default(),
            ignore_column: bool::default(),
            //ignore_fn: Box::new(|_,_| false),
        }
    }
    pub fn new(ignore_if_empty: bool, ignore_column: bool) -> Self {
        Self {
            ignore_if_empty,
            ignore_column,
            // ignore_fn: todo!(),
        }
    }
    // ignore_fn: Box::new(|record: &StringRecord, index: &usize| record.get(index).unwrap_or("").contains("ignore")),
}

impl Option for FilterOption {
    fn is_dirty(&self) -> bool {
        self.ignore_column != bool::default() || self.ignore_if_empty != bool::default()
    }
}
