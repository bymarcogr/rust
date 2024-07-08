use crate::option::Option;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct FilterOption {
    pub ignore_row_if_empty: bool,
    pub ignore_column: bool,
    pub ignore_row_if: bool,
    pub ignore_row_if_text: String,
}
impl FilterOption {
    pub fn default() -> Self {
        Self {
            ignore_row_if_empty: bool::default(),
            ignore_column: bool::default(),
            ignore_row_if: bool::default(),
            ignore_row_if_text: String::default(),
        }
    }
    pub fn new(
        ignore_row_if_empty: bool,
        ignore_column: bool,
        ignore_row_if: bool,
        ignore_row_if_text: String,
    ) -> Self {
        Self {
            ignore_row_if_empty,
            ignore_column,
            ignore_row_if,
            ignore_row_if_text,
        }
    }
}

impl Option for FilterOption {
    fn is_dirty(&self) -> bool {
        self.ignore_column != bool::default()
            || self.ignore_row_if_empty != bool::default()
            || self.ignore_row_if != bool::default()
            || self.ignore_row_if_text != String::default()
    }
}
