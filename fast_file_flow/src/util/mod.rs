use iced::font::Weight;
use iced::widget::image::{Handle, Image};
use iced::widget::{button, text, tooltip, Text};
use iced::window::{icon::Error, Icon};
use iced::Length::{self, Fixed};
use iced::{Element, Font};

use crate::app::FastFileFlowMessage;
use crate::constants::english::{ERROR_GET_FOLDER, ERROR_LOAD_ICON};
use crate::constants::path::{LOGO_PRIMARY_PATH, LOGO_SECONDARY_PATH};
use crate::constants::sizes::{FONT_ICON_SIZE, FONT_NAME, MENU_BUTTON_HEIGHT, MENU_BUTTON_WIDTH};

fn get_icon_from_file() -> Result<Icon, Error> {
    let mut icon = image::io::Reader::new(std::io::Cursor::new(include_bytes!(
        "../resources/images/icon.png"
    )));

    icon.set_format(image::ImageFormat::Png);

    let icon_with_format: image::io::Reader<std::io::Cursor<&[u8; 1060]>> = icon;

    let pixels = icon_with_format.decode().expect(ERROR_LOAD_ICON).to_rgba8();

    Ok(iced::window::icon::from_rgba(
        pixels.to_vec(),
        pixels.width(),
        pixels.height(),
    )?)
}

pub fn get_icon_image() -> Icon {
    get_icon_from_file().expect(ERROR_GET_FOLDER)
}

pub fn get_logo(is_primary: bool) -> Image<Handle> {
    let path = get_full_directory();
    let logo = if is_primary {
        LOGO_PRIMARY_PATH
    } else {
        LOGO_SECONDARY_PATH
    };
    let image = Image::new(format!("{path}{logo}"))
        .width(Fixed(75.0))
        .height(Fixed(69.5));
    return image;
}

pub fn get_full_directory() -> String {
    let folder = std::env::current_dir().expect(ERROR_GET_FOLDER);
    return folder.display().to_string();
}

pub fn get_icon_font<'a>(codepoint: char) -> Element<'a, FastFileFlowMessage> {
    const ICON_FONT: Font = Font::with_name(FONT_NAME);
    text(codepoint)
        .font(ICON_FONT)
        .horizontal_alignment(iced::alignment::Horizontal::Center)
        .vertical_alignment(iced::alignment::Vertical::Center)
        .size(FONT_ICON_SIZE)
        .into()
}

pub fn get_menu_button(
    _codepoint: char,
    on_press_event: FastFileFlowMessage,
    tooltip_text: &'static str,
) -> Element<'static, FastFileFlowMessage> {
    wrap_tooltip(
        button(get_icon_font(_codepoint))
            .width(MENU_BUTTON_WIDTH)
            .height(MENU_BUTTON_HEIGHT)
            .on_press(on_press_event)
            .into(),
        tooltip_text,
    )
    .into()
}

pub fn wrap_tooltip(
    element: Element<'static, FastFileFlowMessage>,
    tooltip_text: &'static str,
) -> Element<'static, FastFileFlowMessage> {
    tooltip(element, tooltip_text, tooltip::Position::Bottom).into()
}

pub fn get_menu_button_by_text(
    label: &'static str,
    on_press_event: FastFileFlowMessage,
) -> Element<'static, FastFileFlowMessage> {
    wrap_tooltip(
        button(text(label))
            //.width(MENU_BUTTON_WIDTH)
            .height(MENU_BUTTON_HEIGHT)
            .on_press(on_press_event)
            .into(),
        label,
    )
    .into()
}

pub fn get_text(input: &str, is_bold: bool) -> Text {
    text(input)
        .size(14)
        .font(Font {
            weight: if is_bold {
                Weight::Bold
            } else {
                Weight::Normal
            },
            ..Default::default()
        })
        .width(Length::Fill)
}
