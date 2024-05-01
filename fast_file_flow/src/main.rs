mod general;
use crate::constants::sizes::PANEL_HEIGHT;
use crate::constants::sizes::PANEL_WIDTH;
pub use crate::general::constants;
pub use crate::general::util;
use crate::util::get_menu_button_by_text;
use general::constants::english::{APP_TITLE, SEARCH_PLACEHOLDER};
use general::constants::sizes::{APP_HEIGHT, APP_WIDTH};
use general::util::get_menu_button;

use iced::widget::{
    column, container, horizontal_space, row, text, text_input, Button, Column, Container, Row,
    Text, TextInput,
};
use iced::Border;
use iced::Color;
use iced::Length::Fixed;

use iced::{
    window, Alignment, Application, Command, Element, Font, Length, Padding, Pixels, Theme,
};

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
            level: window::Level::AlwaysOnTop,
            icon: Some(util::get_icon_image()),
            position: window::Position::Specific(iced::Point::new(0.0, 0.0)),
            ..iced::window::Settings::default()
        },
        ..iced::Settings::default()
    };

    FastFileFlow::run(settings)
}

// Define tu aplicación
struct FastFileFlow {
    page: Page,
    theme: Theme,
    input_value: String,
    is_primary_logo: bool,
    clicked_button: String,
}

// Mensajes para la actualización de la GUI
#[derive(Debug, Clone)]
pub enum FastFileFlowMessage {
    Router(Page),
    TextBoxChange(String),
    UserButtonClick(),
    MenuButtonClick(),
    RefreshButtonClick(),
    LoadFileButtonClick(),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Page {
    Main,
    Configuration,
}

impl iced::Application for FastFileFlow {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = FastFileFlowMessage;
    type Theme = iced::Theme;

    // Inicializar el estado de la aplicación aquí
    fn new(_flags: ()) -> (FastFileFlow, Command<Self::Message>) {
        (
            FastFileFlow {
                page: Page::Main,
                theme: Theme::Dark,
                input_value: String::from(""),
                is_primary_logo: false,
                clicked_button: String::from(""),
            },
            Command::none(),
        )
    }

    // El título de la ventana de la aplicación
    fn title(&self) -> String {
        String::from(APP_TITLE)
    }

    // Actualizaciones basadas en los mensajes aquí
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            FastFileFlowMessage::TextBoxChange(string) => {
                self.input_value = string;
                Command::none()
            }

            FastFileFlowMessage::Router(page) => {
                match page {
                    Page::Main => {
                        self.page = Page::Main;
                        self.theme = Theme::Dark;
                        self.is_primary_logo = false;
                    }
                    _ => {
                        self.page = Page::Configuration;
                        self.theme = Theme::CatppuccinLatte;
                        self.is_primary_logo = true;
                    }
                }
                Command::none()
            }

            FastFileFlowMessage::UserButtonClick() => {
                self.clicked_button = String::from("User Button Clicked");
                Command::none()
            }
            FastFileFlowMessage::MenuButtonClick() => {
                self.clicked_button = String::from("Menu Button Clicked");
                Command::none()
            }
            FastFileFlowMessage::RefreshButtonClick() => {
                self.clicked_button = String::from("Refresh Button Clicked");
                Command::none()
            }
            FastFileFlowMessage::LoadFileButtonClick() => {
                self.clicked_button = String::from("Load File Button Clicked");
                Command::none()
            }
        }
    }

    // Define el layout de tu GUI aquí
    fn view(&self) -> Element<Self::Message> {
        let pages = match self.page {
            Page::Main => main_page(
                self.input_value.as_str(),
                FastFileFlowMessage::Router(Page::Configuration),
            ),
            Page::Configuration => config_page(FastFileFlowMessage::Router(Page::Main)),
        };

        let (clicked_button, header) = self.build_header();
        let panels = self.build_panels().padding([10.0, 0.0, 0.0, 0.0]);

        let content = column![header, panels, clicked_button, pages];

        let border = Border {
            color: Color::from_rgb(0.315, 0.315, 0.315).into(),
            width: 1.0,
            radius: 40.0.into(),
            ..Default::default()
        };
        container(content)
            .align_x(iced::alignment::Horizontal::Left)
            .align_y(iced::alignment::Vertical::Top)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .padding(Padding::from(40))
            .style(container::Appearance {
                border,
                ..Default::default()
            })
            .into()
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
}

impl FastFileFlow {
    fn build_header(&self) -> (Text, Row<FastFileFlowMessage, Theme, iced::Renderer>) {
        let image = util::get_logo(self.is_primary_logo);
        let text_input: TextInput<'_, FastFileFlowMessage> =
            text_input(SEARCH_PLACEHOLDER, self.input_value.as_str())
                .on_input(FastFileFlowMessage::TextBoxChange)
                .width(Fixed(300.0))
                .padding(10)
                .size(15)
                .icon(text_input::Icon {
                    font: Font::with_name("iced-fff"),
                    code_point: '\u{E800}',
                    size: Some(Pixels(20.0)),
                    spacing: 10.0,
                    side: text_input::Side::Left,
                });
        let button_user = get_menu_button(
            crate::general::constants::icons::USER,
            FastFileFlowMessage::UserButtonClick(),
        );
        let button_menu = get_menu_button(
            crate::general::constants::icons::MENU,
            FastFileFlowMessage::MenuButtonClick(),
        );
        let space = horizontal_space();
        let clicked_button = Text::new(self.clicked_button.as_str());
        let header = row![image, space, text_input, button_user, button_menu];
        (clicked_button, header)
    }

    fn build_panels(&self) -> Row<FastFileFlowMessage, Theme, iced::Renderer> {
        let border = Border {
            color: Color::from_rgb(0.315, 0.315, 0.315).into(),
            width: 1.0,
            radius: 20.0.into(),
            ..Default::default()
        };

        let button_refresh = get_menu_button(
            crate::general::constants::icons::REFRESH,
            FastFileFlowMessage::RefreshButtonClick(),
        );
        let section_1 = column![
            row!["File", horizontal_space(), button_refresh],
            row![
                "File",
                get_menu_button_by_text("Load", FastFileFlowMessage::LoadFileButtonClick())
            ]
        ];

        let contenedor = container(section_1)
            .align_x(iced::alignment::Horizontal::Left)
            .align_y(iced::alignment::Vertical::Top)
            .width(PANEL_WIDTH)
            .height(PANEL_HEIGHT)
            .padding([10.0, 20.0, 0.0, 20.0])
            .style(container::Appearance {
                border,
                ..Default::default()
            });

        let section_2 = column![
            row!["Filename", "Valor"],
            row!["Filename", "Valor"],
            row!["Filename", "Valor"],
            row!["Filename", "Valor"],
            row!["Filename", "Valor"]
        ];
        let contenedor2 = container(section_2)
            .align_x(iced::alignment::Horizontal::Left)
            .align_y(iced::alignment::Vertical::Top)
            .width(PANEL_WIDTH)
            .height(PANEL_HEIGHT)
            .padding([10.0, 20.0, 0.0, 20.0])
            .style(container::Appearance {
                border,
                ..Default::default()
            });
        let section_3 = column![
            row!["Filename", "Valor"],
            row!["Filename", "Valor"],
            row!["Filename", "Valor"],
            row!["Filename", "Valor"],
            row!["Filename", "Valor"]
        ];
        let contenedor3 = container(section_3)
            .align_x(iced::alignment::Horizontal::Left)
            .align_y(iced::alignment::Vertical::Top)
            .height(PANEL_HEIGHT)
            .width(PANEL_WIDTH)
            .padding([10.0, 20.0, 0.0, 20.0])
            .style(container::Appearance {
                border,
                ..Default::default()
            });

        let section_4 = column![
            row!["Filename", "Valor"],
            row!["Filename", "Valor"],
            row!["Filename", "Valor"],
            row!["Filename", "Valor"],
            row!["Filename", "Valor"]
        ];
        let contenedor4 = container(section_4)
            .align_x(iced::alignment::Horizontal::Left)
            .align_y(iced::alignment::Vertical::Top)
            .height(PANEL_HEIGHT)
            .width(PANEL_WIDTH)
            .padding([10.0, 20.0, 0.0, 20.0])
            .style(container::Appearance {
                border,
                ..Default::default()
            });

        row![
            contenedor,
            horizontal_space(),
            contenedor2,
            horizontal_space(),
            contenedor3,
            horizontal_space(),
            contenedor4
        ]
    }
}

fn main_page(
    _value: &str,
    page_change_event: FastFileFlowMessage,
) -> Container<FastFileFlowMessage> {
    // //"Hello, world!".into()-
    // FastFileFlowMessage::ChangeLogo(false);
    let button = submit_btn("Cambiar", page_change_event);
    let image = util::get_logo(false);
    let nt = Text::new("Hello iced");
    let folder = util::get_full_directory();
    let text = Text::new(format!("Hello, {0}.", _value))
        .width(Length::Fill)
        .horizontal_alignment(iced::alignment::Horizontal::Center);
    let row1 = Row::new().push(text).push(Text::new(folder));
    let text_input: TextInput<'_, FastFileFlowMessage> =
        text_input("world", _value).on_input(FastFileFlowMessage::TextBoxChange);
    let row2 = Row::new().push(image).push(nt);
    let col = Column::new().push(row1).push(row2).push(text_input);
    let column = Column::new()
        .push(col)
        .push(Text::new("Hello iced 2"))
        .push(button)
        .padding(Padding::from([50, 20]))
        .align_items(Alignment::Start)
        .spacing(40);

    container(column).padding(Padding::from(20))
}

fn config_page(page_change_event: FastFileFlowMessage) -> Container<'static, FastFileFlowMessage> {
    // FastFileFlowMessage::ChangeLogo(true);
    let button = submit_btn("Back", page_change_event);
    let image = util::get_logo(true);
    let column = Column::new()
        .push(Text::new("Hello iced 2"))
        .push(image)
        .push(button)
        .padding(Padding::from([50, 20]))
        .align_items(Alignment::Start)
        .spacing(40);

    container(column).padding(Padding::from(20))
}

fn submit_btn(name: &str, event: FastFileFlowMessage) -> Button<FastFileFlowMessage> {
    Button::new(
        text(name)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
            .vertical_alignment(iced::alignment::Vertical::Top)
            .size(18),
    )
    .on_press(event)
    .width(Length::Fixed(100.0))
    .height(Length::Fixed(60.0))
    .style(iced::theme::Button::Primary)
}
