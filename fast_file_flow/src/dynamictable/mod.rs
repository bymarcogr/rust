use crate::app::FastFileFlowMessage;
use iced::widget::{container, text};
use iced::{Element, Length, Theme};
use iced_table::table;

#[derive(Debug, Clone)]
pub struct IcedRow {
    pub values: Vec<String>,
    pub is_enabled: bool,
    pub column_index: u32,
    pub row_index: u32,
}

impl IcedRow {
    pub fn empty() -> Self {
        Self {
            is_enabled: true,
            values: vec![],
            column_index: 0,
            row_index: 0,
        }
    }
    pub fn new(values: Vec<String>, column: u32, row: u32) -> Self {
        Self {
            is_enabled: true,
            values,
            column_index: column,
            row_index: row,
        }
    }
}

#[derive(Debug, Clone)]
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

impl<'a> table::Column<'a, FastFileFlowMessage, Theme, iced::Renderer> for IcedColumn {
    type Row = IcedRow;

    fn header(&'a self, _col_index: usize) -> Element<'a, FastFileFlowMessage> {
        let content = self.column_header.clone();

        container(text(content).size(10.0))
            .height(24)
            .center_y()
            .into()
    }

    fn cell(
        &'a self,
        _col_index: usize,
        _row_index: usize,
        _row: &'a Self::Row,
    ) -> Element<'a, FastFileFlowMessage> {
        let value = _row
            .values
            .get(_col_index)
            .unwrap_or(&String::new())
            .clone();
        let content: Element<_> = text(value.clone()).size(10.0).into();

        container(content)
            .width(Length::Fill)
            .height(24.0)
            .center_y()
            .into()
    }

    fn footer(
        &'a self,
        _col_index: usize,
        rows: &'a [Self::Row],
    ) -> Option<Element<'a, FastFileFlowMessage>> {
        let total_enabled = rows.iter().filter(|row| row.is_enabled).count();
        let content = Element::from(text(format!("Total Enabled: {total_enabled}")));
        Some(container(content).height(24).center_y().into())
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}
