use crate::stadistics::Stadistics;

#[derive(Debug, Clone)]
pub struct IcedColumn {
    pub column_header: String,
    pub width: f32,
    pub resize_offset: Option<f32>,
    pub is_checked: bool,
    pub stadistics: Stadistics,
}

impl IcedColumn {
    pub fn new(column_header: String) -> Self {
        let width = 100.0;

        Self {
            column_header,
            width,
            resize_offset: None,
            is_checked: false,
            stadistics: Stadistics::default(),
        }
    }
}
