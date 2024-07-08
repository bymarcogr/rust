use crate::option::Option;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ProccessOption {
    pub trim: bool,
    pub replace_if_empty: bool,
    pub replace_with: bool,
    pub replace_if: bool,
}
impl ProccessOption {
    pub fn default() -> Self {
        Self {
            trim: bool::default(),
            replace_if_empty: bool::default(),
            replace_with: bool::default(),
            replace_if: bool::default(),
        }
    }
    pub fn new(trim: bool, replace_if_empty: bool, replace_with: bool, replace_if: bool) -> Self {
        Self {
            trim,
            replace_if_empty,
            replace_with,
            replace_if,
        }
    }
}

impl Option for ProccessOption {
    fn is_dirty(&self) -> bool {
        self.replace_if_empty != bool::default()
            || self.trim != bool::default()
            || self.replace_with != bool::default()
            || self.replace_if != bool::default()
    }
}
