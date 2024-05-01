mod general;
pub use crate::general::util;
use iced::widget::{
    container, text, text_input, Button, Column, Container, Row, Text, TextInput,
};
use iced::{
    window,Command, Alignment, Application,  Element, Length, Padding, Theme,
};

fn main() -> iced::Result {
    let settings = iced::Settings {
        window: iced::window::Settings {
            size: iced::Size::new(1366.0, 768.0),
            resizable: false,
            decorations: true,
            transparent: true,
            visible: true,
            level: window::Level::AlwaysOnTop,
            icon: Some(util::get_icon()),
            position: window::Position::Specific(iced::Point::new(0.0,0.0)),
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
}

// Mensajes para la actualización de la GUI
#[derive(Debug, Clone)]
enum FastFileFlowMessage {
    Router(Page),
    TextBoxChange(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Page {
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
                input_value: String::from("world"),
            },
            Command::none(),
        )
    }

    // El título de la ventana de la aplicación
    fn title(&self) -> String {
        String::from("Fast File Flow - Unir")
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
                    }
                    _ => {
                        self.page = Page::Configuration;
                        self.theme = Theme::CatppuccinLatte;
                    }
                }
                Command::none()
            }
        }
    }

    // Define el layout de tu GUI aquí
    fn view(&self) -> Element<Self::Message> {
        let content = match self.page {
            Page::Main => main_page(
                self.input_value.as_str(),
                FastFileFlowMessage::Router(Page::Configuration),
            ),
            Page::Configuration => config_page(FastFileFlowMessage::Router(Page::Main)),
        };     

        let nt = Text::new("ICON CANNOT BE DISPLAYED");
        let header = Column::new()
            .spacing(50)
            .width(Length::Fill)
            .align_items(Alignment::Start)
            .push(nt)
            .push(content);

        container(header)
            .center_x()
            .center_y()
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .padding(Padding::from(20))
            .into()
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }
}

fn main_page(_value: &str, event: FastFileFlowMessage) -> Container<FastFileFlowMessage> {
    // //"Hello, world!".into()-
    let button = submit_btn("Cambiar", event);
    let image = util::get_logo();
    let nt = Text::new("Hello iced");
    let folder = util::get_full_directory();
    let text = Text::new(format!("Hello, {0}.", _value))
        .width(Length::Fill)
        .horizontal_alignment(iced::alignment::Horizontal::Center);
    let row1 = Row::new().push(text) .push(Text::new(folder));
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

fn config_page(event: FastFileFlowMessage) -> Container<'static, FastFileFlowMessage> {
    let button = submit_btn("Back", event);
    let image = util::get_logo();
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
