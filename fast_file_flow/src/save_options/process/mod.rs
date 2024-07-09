use crate::option::Option;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ProccessOption {
    pub trim: bool,
    pub replace_if_empty: bool,
    pub replace_with: bool,
    pub replace_if: bool,
    pub replace_if_empty_value: String,
    pub replace_with_value: String,
    pub replace_if_value: String,
    pub replace_then_value: String,
}
impl ProccessOption {
    pub fn default() -> Self {
        Self {
            trim: bool::default(),
            replace_if_empty: bool::default(),
            replace_with: bool::default(),
            replace_if: bool::default(),
            replace_with_value: String::default(),
            replace_if_value: String::default(),
            replace_then_value: String::default(),
            replace_if_empty_value: String::default(),
        }
    }
    pub fn new(
        trim: bool,
        replace_if_empty: bool,
        replace_with: bool,
        replace_if: bool,
        replace_with_value: String,
        replace_if_value: String,
        replace_when_value: String,
        replace_if_empty_value: String,
    ) -> Self {
        Self {
            trim,
            replace_if_empty,
            replace_with,
            replace_if,
            replace_with_value,
            replace_if_value,
            replace_then_value: replace_when_value,
            replace_if_empty_value: replace_if_empty_value,
        }
    }
}

impl Option for ProccessOption {
    fn is_dirty(&self) -> bool {
        self.replace_if_empty != bool::default()
            || self.trim != bool::default()
            || self.replace_with != bool::default()
            || self.replace_if != bool::default()
            || self.replace_with_value != String::default()
            || self.replace_if_value != String::default()
            || self.replace_then_value != String::default()
            || self.replace_if_empty_value != String::default()
    }
}
