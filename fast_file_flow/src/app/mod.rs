use crate::constants::english::*;
use crate::constants::icons::*;
use crate::constants::sizes::{FONT_NAME, PANEL_HEIGHT, PANEL_WIDTH, SEARCH_TEXTBOX_WIDTH};
use crate::dynamictable::{ColumnTable, RowTable};
use crate::util::{
    get_full_directory, get_logo, get_menu_button, get_menu_button_by_text, get_text, wrap_tooltip,
};

use iced_table::table;

use iced::widget::{
    column, container, horizontal_space, responsive, row, scrollable, text, text_input, tooltip,
    Button, Column, Container, Row, Text, TextInput,
};

use iced::Length::Fixed;
use iced::{Alignment, Border, Color, Command, Element, Font, Length, Padding, Pixels, Theme};

pub struct FastFileFlow {
    page: Page,
    theme: Theme,
    input_value: String,
    is_primary_logo: bool,
    clicked_button: String,
    header: scrollable::Id,
    body: scrollable::Id,
    footer: scrollable::Id,
    columns: Vec<ColumnTable>,
    rows: Vec<RowTable>,
    file_loaded: String,
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
    FilterButtonClick(),
    ProcessButtonClick(),
    AddButtonClick(),
    ScriptButtonClick(),
    PipelineButtonClick(),
    AnalysisButtonClick(),
    AIButtonClick(),
    PreviewButtonClick(),
    SaveButtonClick(),
    ExportButtonClick(),
    SearchOnSubmit(),
    SyncHeader(scrollable::AbsoluteOffset),
    Resizing(usize, f32),
    Resized,
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
                header: scrollable::Id::unique(),
                body: scrollable::Id::unique(),
                footer: scrollable::Id::unique(),
                columns: vec![
                    ColumnTable::new("Index".to_string()),
                    ColumnTable::new("Column 2".to_string()),
                    ColumnTable::new("Column 3".to_string()),
                    ColumnTable::new("Column 4".to_string()),
                    ColumnTable::new("Column 5".to_string()),
                    ColumnTable::new("Column 6".to_string()),
                    ColumnTable::new("Column 7".to_string()),
                ],
                rows: (0..50).map(RowTable::generate).collect(),
                file_loaded: String::from(""),
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
                if self.file_loaded != "" {
                    self.clicked_button = String::from(self.file_loaded.clone());
                    // Load File
                }

                Command::none()
            }
            FastFileFlowMessage::LoadFileButtonClick() => {
                self.clicked_button = String::from("Load File Button Clicked");

                let path = crate::dialog::load_csv();

                if path != "" {
                    self.file_loaded = path;
                    // Load File
                }

                Command::none()
            }
            FastFileFlowMessage::FilterButtonClick() => {
                self.clicked_button = String::from("Filter Button Clicked");
                Command::none()
            }
            FastFileFlowMessage::ProcessButtonClick() => {
                self.clicked_button = String::from("Process Button Clicked");
                Command::none()
            }
            FastFileFlowMessage::AddButtonClick() => {
                self.clicked_button = String::from("Add Button Clicked");
                Command::none()
            }
            FastFileFlowMessage::ScriptButtonClick() => {
                self.clicked_button = String::from("Script Button Clicked");
                Command::none()
            }
            FastFileFlowMessage::PipelineButtonClick() => {
                self.clicked_button = String::from("Pipeline Button Clicked");
                Command::none()
            }
            FastFileFlowMessage::AnalysisButtonClick() => {
                self.clicked_button = String::from("Analysis Button Clicked");
                Command::none()
            }
            FastFileFlowMessage::AIButtonClick() => {
                self.clicked_button = String::from("AI Button Clicked");
                Command::none()
            }
            FastFileFlowMessage::PreviewButtonClick() => {
                self.clicked_button = String::from("Preview Button Clicked");
                Command::none()
            }
            FastFileFlowMessage::SaveButtonClick() => {
                self.clicked_button = String::from("Save Button Clicked");
                Command::none()
            }
            FastFileFlowMessage::ExportButtonClick() => {
                self.clicked_button = String::from("Export Button Clicked");
                Command::none()
            }
            FastFileFlowMessage::SearchOnSubmit() => {
                self.clicked_button = String::from("Search On Submit");
                Command::none()
            }
            FastFileFlowMessage::SyncHeader(offset) => {
                return Command::batch(vec![
                    scrollable::scroll_to(self.header.clone(), offset),
                    scrollable::scroll_to(self.footer.clone(), offset),
                ])
            }
            FastFileFlowMessage::Resizing(index, offset) => {
                if let Some(column) = self.columns.get_mut(index) {
                    column.resize_offset = Some(offset);
                }
                Command::none()
            }
            FastFileFlowMessage::Resized => {
                self.columns.iter_mut().for_each(|column| {
                    if let Some(offset) = column.resize_offset.take() {
                        column.width += offset;
                    }
                });
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
        let action_menu = self.build_action_menu();
        let table = self.build_table();

        let content = column![header, panels, action_menu, clicked_button, table];

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
        let image = tooltip(
            get_logo(self.is_primary_logo),
            APP_TOOLTIP,
            tooltip::Position::Right,
        );

        let text_input: TextInput<'_, FastFileFlowMessage> =
            text_input(SEARCH_PLACEHOLDER, self.input_value.as_str())
                .on_input(FastFileFlowMessage::TextBoxChange)
                .on_submit(FastFileFlowMessage::SearchOnSubmit())
                .width(Fixed(SEARCH_TEXTBOX_WIDTH))
                .padding(10)
                .size(15)
                .icon(text_input::Icon {
                    font: Font::with_name(FONT_NAME),
                    code_point: '\u{E800}',
                    size: Some(Pixels(15.0)),
                    spacing: 10.0,
                    side: text_input::Side::Left,
                });
        let button_user = get_menu_button(USER, FastFileFlowMessage::UserButtonClick(), USER_ICON);

        let button_menu = get_menu_button(MENU, FastFileFlowMessage::MenuButtonClick(), MENU_ICON);

        let clicked_button = Text::new(self.clicked_button.as_str());
        let header = row![
            image,
            horizontal_space(),
            text_input,
            button_user,
            TAB_SPACE,
            button_menu
        ];
        (clicked_button, header)
    }

    fn build_panels(&self) -> Row<FastFileFlowMessage, Theme, iced::Renderer> {
        let button_refresh = get_menu_button(
            REFRESH,
            FastFileFlowMessage::RefreshButtonClick(),
            REFRESH_ICON,
        );
        let button_open =
            get_menu_button(OPEN, FastFileFlowMessage::LoadFileButtonClick(), OPEN_ICON);
        let selected_file = Text::new(self.file_loaded.as_str())
            .width(PANEL_WIDTH)
            .size(Pixels(10.0));
        let panel_file = column![
            row![
                "File",
                horizontal_space(),
                button_open,
                TAB_SPACE,
                button_refresh
            ],
            row![TAB_SPACE],
            row![selected_file],
        ];

        let container_load_file = create_section_container(panel_file);

        let panel_file_details = column![
            row![
                get_text("Filename", false),
                TAB_SPACE,
                get_text("Value", true)
            ],
            row![
                get_text("Encoding ", false),
                TAB_SPACE,
                get_text("Valor", true)
            ],
            row![
                get_text("Size     ", false),
                TAB_SPACE,
                get_text("Valor", true)
            ],
            row![
                get_text("Format   ", false),
                TAB_SPACE,
                get_text("Valor", true)
            ],
            row![
                get_text("Sintaxis ", false),
                TAB_SPACE,
                get_text("Valor", true)
            ]
        ];
        let container_file_details = create_section_container(panel_file_details);

        let panel_column_analysis = column![
            row![
                get_text("Datatype", false),
                get_text("Valor", true),
                TAB_SPACE,
                get_text("Mean", false),
                get_text("Valor", true)
            ],
            row![
                get_text("Distinct", false),
                get_text("Valor", true),
                TAB_SPACE,
                get_text("Median", false),
                get_text("Valor", true)
            ],
            row![
                get_text("Repeated", false),
                get_text("Valor", true),
                TAB_SPACE,
                get_text("Mode", false),
                get_text("Valor", true)
            ],
            row![
                get_text("Minimum", false),
                get_text("Valor", true),
                TAB_SPACE,
                get_text("Range", false),
                get_text("Valor", true)
            ],
            row![
                get_text("Maximum", false),
                get_text("Valor", true),
                TAB_SPACE,
                get_text("Variance", false),
                get_text("Valor", true)
            ],
            row![
                horizontal_space(),
                horizontal_space(),
                TAB_SPACE,
                get_text("Quatril", false),
                get_text("Valor", true)
            ],
            row![
                horizontal_space(),
                horizontal_space(),
                TAB_SPACE,
                get_text("Std Dev.", false),
                get_text("Valor", true)
            ],
        ];
        let container_analysis = create_section_container(panel_column_analysis);

        let panel_coefficient = column![
            row![
                wrap_tooltip(
                    get_text("Pearson CC", false).into(),
                    "Pearson correlation coefficient"
                ),
                get_text("Valor", true)
            ],
            row![
                wrap_tooltip(
                    get_text("Spearman CC", false).into(),
                    "Spearman correlation coefficient"
                ),
                get_text("Valor", true)
            ],
            row![get_text("Covariance", false), get_text("Valor", true)],
            row!["Graph", TAB_SPACE, "Valor"]
        ];
        let container_correlation = create_section_container(panel_coefficient);

        row![
            container_load_file,
            horizontal_space(),
            container_file_details,
            horizontal_space(),
            container_analysis,
            horizontal_space(),
            container_correlation
        ]
    }

    fn build_action_menu(&self) -> Row<FastFileFlowMessage, Theme, iced::Renderer> {
        let button_filter = get_menu_button(
            FILTER,
            FastFileFlowMessage::FilterButtonClick(),
            FILTER_ICON,
        );

        let button_process = get_menu_button(
            PROCESS,
            FastFileFlowMessage::ProcessButtonClick(),
            PROCESS_ICON,
        );

        let button_add = get_menu_button(ADD, FastFileFlowMessage::AddButtonClick(), ADD_ICON);

        let button_script = get_menu_button(
            SCRIPT,
            FastFileFlowMessage::ScriptButtonClick(),
            SCRIPT_ICON,
        );

        let button_pipeline = get_menu_button(
            PIPELINE,
            FastFileFlowMessage::PipelineButtonClick(),
            PIPELINE_ICON,
        );

        let button_analysis = get_menu_button(
            ANALYSIS,
            FastFileFlowMessage::AnalysisButtonClick(),
            ANALYSIS_ICON,
        );

        let button_ai = get_menu_button(AI, FastFileFlowMessage::AIButtonClick(), AI_ICON);

        let button_preview = get_menu_button(
            PREVIEW,
            FastFileFlowMessage::PreviewButtonClick(),
            PREVIEW_ICON,
        );

        let button_save = get_menu_button(SAVE, FastFileFlowMessage::SaveButtonClick(), SAVE_ICON);

        let button_export = get_menu_button(
            EXPORT,
            FastFileFlowMessage::ExportButtonClick(),
            EXPORT_ICON,
        );
        row![
            button_filter,
            TAB_SPACE,
            button_process,
            TAB_SPACE,
            button_add,
            TAB_SPACE,
            button_script,
            TAB_SPACE,
            button_pipeline,
            TAB_SPACE,
            button_analysis,
            TAB_SPACE,
            button_ai,
            TAB_SPACE,
            button_preview,
            TAB_SPACE,
            button_save,
            TAB_SPACE,
            button_export,
            TAB_SPACE,
        ]
        .padding([10.0, 50.0, 10.0, 0.0])
        .into()
    }

    fn build_table(&self) -> Row<FastFileFlowMessage, Theme, iced::Renderer> {
        let table = responsive(|size| {
            let mut table = table(
                self.header.clone(),
                self.body.clone(),
                &self.columns,
                &self.rows,
                FastFileFlowMessage::SyncHeader,
            );

            // if self.resize_columns_enabled {
            table =
                table.on_column_resize(FastFileFlowMessage::Resizing, FastFileFlowMessage::Resized);
            // }
            // if self.footer_enabled {
            table = table.footer(self.footer.clone());
            // }
            // if self.min_width_enabled {
            table = table.min_width(size.width);
            // }

            table.into()
        });

        row![table]
    }
}

fn create_section_container(
    section: Column<FastFileFlowMessage, Theme, iced::Renderer>,
) -> Container<FastFileFlowMessage, Theme, iced::Renderer> {
    let border = Border {
        color: Color::from_rgb(0.315, 0.315, 0.315).into(),
        width: 1.0,
        radius: 20.0.into(),
        ..Default::default()
    };

    container(section)
        .align_x(iced::alignment::Horizontal::Left)
        .align_y(iced::alignment::Vertical::Top)
        .width(PANEL_WIDTH)
        .height(PANEL_HEIGHT)
        .padding([10.0, 20.0, 0.0, 20.0])
        .style(container::Appearance {
            border,
            ..Default::default()
        })
}

fn main_page(
    _value: &str,
    page_change_event: FastFileFlowMessage,
) -> Container<FastFileFlowMessage> {
    // //"Hello, world!".into()-
    // FastFileFlowMessage::ChangeLogo(false);
    let button = submit_btn("Cambiar", page_change_event);
    let image = get_logo(false);
    let nt = Text::new("Hello iced");
    let folder = get_full_directory();
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
    let image = get_logo(true);
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

impl<'a> table::Column<'a, FastFileFlowMessage, Theme, iced::Renderer> for ColumnTable {
    type Row = RowTable;

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
            .height(32)
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
