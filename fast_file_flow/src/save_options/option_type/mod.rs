use core::mem::discriminant as tag;
#[derive(Debug, Eq, Hash, Clone)]
pub enum OptionType {
    None = 0,
    FilterIgnoreIfEmpty = 1,
    FilterIgnoreColumn = 2,
    ProcessTrim = 3,
    ProcessReplaceIfEmpty = 4,
    ProcessReplaceWith = 5,
    ProcessReplaceIf = 6,
}

impl PartialEq for OptionType {
    fn eq(&self, other: &Self) -> bool {
        tag(self) == tag(other)
    }
}
