use core::mem::discriminant as tag;
#[derive(Debug, Eq, Hash, Clone)]
pub enum OptionType {
    None = 0,
    FilterIgnoreIfEmpty = 1,
    FilterIgnoreColumn = 2,
    FilterIgnoreIf = 3,
    ProcessTrim = 4,
    ProcessReplaceIfEmpty = 5,
    ProcessReplaceWith = 6,
    ProcessReplaceIf = 7,
    ProcessReplaceIfThen = 8,
}

impl PartialEq for OptionType {
    fn eq(&self, other: &Self) -> bool {
        tag(self) == tag(other)
    }
}
