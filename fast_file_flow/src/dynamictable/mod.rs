use crate::app::FastFileFlowMessage;
use iced::widget::{container, text};
use iced::{Element, Length, Theme};
use iced_table::table;

pub struct IcedRow {
    pub notes: String,
    pub category: Category,
    pub is_enabled: bool,
}

impl IcedRow {
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
pub enum Category {
    A,
    B,
    C,
    D,
    E,
}

pub struct IcedColumn {
    pub column_header: String,
    pub width: f32,
    pub resize_offset: Option<f32>,
}

impl IcedColumn {
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

impl<'a> table::Column<'a, FastFileFlowMessage, Theme, iced::Renderer> for IcedColumn {
    type Row = IcedRow;

    fn header(&'a self, _col_index: usize) -> Element<'a, FastFileFlowMessage> {
        let content = self.column_header.clone();

        container(text(content)).height(24).center_y().into()
    }

    fn cell(
        &'a self,
        _col_index: usize,
        _row_index: usize,
        _row: &'a Self::Row,
    ) -> Element<'a, FastFileFlowMessage> {
        let content: Element<_> = text(_row_index).into();

        container(content)
            .width(Length::Fill)
            .height(20)
            .center_y()
            .into()
    }

    fn footer(
        &'a self,
        _col_index: usize,
        rows: &'a [Self::Row],
    ) -> Option<Element<'a, FastFileFlowMessage>> {
        // if matches!(self.kind, ColumnKind::Enabled) {
        let total_enabled = rows.iter().filter(|row| row.is_enabled).count();

        let content = Element::from(text(format!("Total Enabled: {total_enabled}")));
        // } else {
        //     horizontal_space().into()
        // };

        Some(container(content).height(24).center_y().into())
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}
