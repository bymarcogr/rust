use fast_file_flow::{
    app::FastFileFlow,
    constants::sizes::{APP_HEIGHT, APP_WIDTH},
    util,
};

use iced::{window, Application};

fn main() -> iced::Result {
    let settings = iced::Settings {
        fonts: vec![include_bytes!("resources/fonts/iced-fff.ttf")
            .as_slice()
            .into()],
        window: iced::window::Settings {
            size: iced::Size::new(APP_WIDTH, APP_HEIGHT),
            resizable: false,
            decorations: true,
            transparent: true,
            visible: true,
            level: window::Level::Normal,
            icon: Some(util::get_icon_image()),
            position: window::Position::Specific(iced::Point::new(0.0, 0.0)),
            ..iced::window::Settings::default()
        },
        ..iced::Settings::default()
    };

    FastFileFlow::run(settings)
}
