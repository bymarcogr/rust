use crate::app::FastFileFlowMessage;
use crate::util::wrap_tooltip_with_position;
use iced::widget::{container, text};
use iced::{Element, Length, Padding, Pixels, Theme};
use iced_table::table;
use iced_widget::{checkbox, column, row, tooltip, Button, Text};

#[derive(Debug, Clone)]
pub struct IcedRow {
    pub values: Vec<String>,
    pub is_enabled: bool,
    pub row_index: u32,
}

impl IcedRow {
    pub fn empty() -> Self {
        Self {
            is_enabled: true,
            values: vec![],
            row_index: 0,
        }
    }
    pub fn new(values: Vec<String>, row: u32) -> Self {
        Self {
            is_enabled: true,
            values,
            row_index: row,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IcedColumn {
    pub column_header: String,
    pub width: f32,
    pub resize_offset: Option<f32>,
    pub is_checked: bool,
}

impl IcedColumn {
    pub fn new(column_header: String) -> Self {
        let width = 100.0;

        Self {
            column_header,
            width,
            resize_offset: None,
            is_checked: false,
        }
    }
}

impl<'a> table::Column<'a, FastFileFlowMessage, Theme, iced::Renderer> for IcedColumn {
    type Row = IcedRow;

    fn header(&'a self, _col_index: usize) -> Element<'a, FastFileFlowMessage> {
        let checkbox = checkbox("", self.is_checked)
            .size(Pixels(14.0))
            .spacing(Pixels(1.0))
            .on_toggle(move |is_checked| {
                FastFileFlowMessage::HeaderCheckBoxToggled(_col_index, is_checked)
            });

        let button = Button::new(Text::new(self.column_header.clone()).size(Pixels(11.0)))
            .on_press(FastFileFlowMessage::HeaderClicked(_col_index));

        let header = row![
            (column![checkbox]).padding(Padding::from([3, 0, 0, 0])),
            column![button]
        ];

        let tooltip: &'static str = Box::leak(Box::new(String::from(self.column_header.clone())));
        let wrapped_button = wrap_tooltip_with_position(
            header.align_items(iced::Alignment::Start).into(),
            tooltip,
            tooltip::Position::Top,
        );

        container(wrapped_button)
            .height(32)
            .width(Length::Fill)
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
        let values = rows.get(_col_index).unwrap().clone();
        let total_enabled = values
            .values
            .iter()
            .filter(|s| !s.is_empty())
            .into_iter()
            .count();

        let content = Element::from(text(format!("{total_enabled}")).size(Pixels(10.0)));
        Some(container(content).height(24).center_y().into())
    }

    fn width(&self) -> f32 {
        self.width
    }

    fn resize_offset(&self) -> Option<f32> {
        self.resize_offset
    }
}
