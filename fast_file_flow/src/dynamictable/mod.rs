pub struct RowTable {
    pub notes: String,
    pub category: Category,
    pub is_enabled: bool,
}

impl RowTable {
    pub fn generate(index: usize) -> Self {
        let category = match index % 5 {
            0 => Category::A,
            1 => Category::B,
            2 => Category::C,
            3 => Category::D,
            4 => Category::E,
            _ => unreachable!(),
        };
        let is_enabled = true;

        Self {
            notes: String::new(),
            category,
            is_enabled,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    A,
    B,
    C,
    D,
    E,
}

pub struct ColumnTable {
    pub column_header: String,
    pub width: f32,
    pub resize_offset: Option<f32>,
}

impl ColumnTable {
    pub fn new(column_header: String) -> Self {
        let width = 100.0;

        Self {
            column_header,
            width,
            resize_offset: None,
        }
    }
}

enum ColumnKind {
    Index,
    Category,
    Enabled,
    Notes,
    Delete,
}
