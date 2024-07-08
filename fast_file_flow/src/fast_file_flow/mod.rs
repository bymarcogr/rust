use crate::constants::english::*;
use crate::constants::icons::*;
use crate::constants::sizes::{
    FONT_NAME, PANEL_FONT_SIZE, PANEL_HEIGHT, PANEL_WIDTH, SEARCH_TEXTBOX_WIDTH,
};
use crate::correlation_analysis::CorrelationAnalysis;
use crate::dynamictable::iced_column::IcedColumn;
use crate::dynamictable::iced_row::IcedRow;
use crate::dynamictable::simple_column::SimpleColumn;
use crate::stadistics::data_classification::DataClassification;
use crate::stadistics::Stadistics;
use crate::stored_file::StoredFile;
use crate::util::{
    get_full_directory, get_logo, get_menu_button, get_text, get_text_size, wrap_tooltip,
};
use iced::widget::{
    column, container, horizontal_space, responsive, row, scrollable, text, text_input, tooltip,
    Button, Column, Container, Row, Text, TextInput,
};
use iced::Length::Fixed;
use iced::{Alignment, Border, Color, Command, Font, Length, Padding, Pixels, Theme};
use iced_table::table;
use iced_widget::checkbox;
use iced_widget::combo_box;
use iced_widget::core::Element;
use iced_widget::vertical_space;
use linear::Linear;
use num_format::{Locale, ToFormattedString};
use std::time::Duration;
mod easing;
mod iced_app;
mod linear;

pub struct FastFileFlow {
    page: Page,
    is_primary_logo: bool,
    clicked_button: String,
    selected_file: StoredFile,
    column_stadistics: Stadistics,
    correlation_file: CorrelationAnalysis,
    header: scrollable::Id,
    body: scrollable::Id,
    footer: scrollable::Id,
    columns: Vec<IcedColumn>,
    rows: Vec<IcedRow>,
    file_loaded: String,
    progress: f32,
    running: bool,
    header_checked: Vec<SimpleColumn>,
    error_message: String,

    column_options: Vec<SimpleColumn>,
    column_option_selected: Option<SimpleColumn>,
    column_options_state: combo_box::State<SimpleColumn>,

    theme: Theme,
    input_value: String,
}

// Mensajes para la actualización de la GUI
#[derive(Debug, Clone)]
pub enum FastFileFlowMessage {
    Router(Page),
    TextBoxChange(String),
    UserButtonClick(),
    MenuButtonClick(),
    LoadFileButtonClick(bool),
    Tick(f32),
    SetSelectedFile(StoredFile),
    SetStadisticsFile(usize, Stadistics, bool),
    HeaderClicked(usize),
    HeaderCheckBoxToggled(usize, bool),
    SetCorrelationFile(CorrelationAnalysis),
    FilterButtonClick(),
    ProcessButtonClick(),
    AddButtonClick(),
    ScriptButtonClick(),
    PipelineButtonClick(),
    AnalysisButtonClick(),
    AnalysisCompleted(String),
    AIButtonClick(),
    PreviewButtonClick(),
    SaveButtonClick(),
    ExportButtonClick(),
    SearchOnSubmit(),
    SyncHeader(scrollable::AbsoluteOffset),
    Resizing(usize, f32),
    Resized,
    ColumnOptionSelected(SimpleColumn),
    ColumnOptionSelectedClosed,
    FilterIgnoreIfEmpty(usize, bool),
    FilterIgnoreColumn(usize, bool),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Page {
    Main,
    Filter,
}

impl FastFileFlow {
    fn show_main_screen(&self) -> Element<'_, FastFileFlowMessage, Theme, iced::Renderer> {
        let (clicked_button, header) = self.build_header();
        let panels = self.build_panels().padding([10.0, 0.0, 0.0, 0.0]);
        let action_menu = self.build_action_menu();
        let table = self.build_table();

        let loader = self.build_status();

        let content = column![header, panels, action_menu, clicked_button, table, loader];

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
            .padding([40.0, 40.0, 10.0, 40.0])
            .style(container::Appearance {
                border,
                ..Default::default()
            })
            .into()
    }

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
        let panel_file_details = column![
            row![
                get_text("Filename:", false),
                get_text_size(self.selected_file.file_name.as_str(), true, Pixels(9.0))
            ],
            row![
                get_text("Encoding:", false),
                get_text(self.selected_file.encoding.as_str(), true)
            ],
            row![
                get_text("Size:", false),
                get_text(self.selected_file.size_mb_as_str(), true)
            ],
            row![
                get_text("Format:", false),
                get_text(self.selected_file.format.as_str(), true)
            ],
            row![
                get_text("Sintaxis:", false),
                get_text(
                    format!("{}", self.selected_file.sintaxis.to_string()).as_str(),
                    true
                )
            ],
            row![
                get_text("Rows:", false),
                get_text(
                    format!(
                        "{}",
                        self.selected_file
                            .rows
                            .total
                            .to_formatted_string(&Locale::en)
                    )
                    .as_str(),
                    true
                )
            ],
            row![
                get_text("Columns:", false),
                get_text(
                    format!(
                        "{}",
                        self.selected_file
                            .columns
                            .total
                            .to_formatted_string(&Locale::en)
                    )
                    .as_str(),
                    true
                )
            ]
        ];

        let container_file_details = create_section_container(panel_file_details);

        let panel_column_analysis = column![
            row![
                get_text(self.column_stadistics.header.to_string(), true)
                    .height(Length::Fixed(24.0))
                    .width(Length::Fixed(PANEL_WIDTH - 12.0)),
                get_text("Mean:", false),
                get_text_size(
                    self.column_stadistics.mean.as_str(),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                )
            ],
            row![
                get_text("Datatype:", false),
                get_text_size(
                    self.column_stadistics.data_type.to_string(),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                ),
                TAB_SPACE,
                get_text("Median:", false),
                get_text_size(
                    self.column_stadistics.median.as_str(),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                )
            ],
            row![
                get_text("Class:", false),
                get_text_size(
                    self.column_stadistics.classification.to_string(),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                ),
                TAB_SPACE,
                get_text("Range:", false),
                get_text_size(
                    self.column_stadistics.range.as_str(),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                ),
            ],
            row![
                get_text("Distinct:", false),
                get_text_size(
                    self.column_stadistics.distinct.as_str(),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                ),
                TAB_SPACE,
                get_text("Variance:", false),
                get_text_size(
                    self.column_stadistics.variance.as_str(),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                ),
            ],
            row![
                get_text("Minimum:", false),
                get_text_size(
                    self.column_stadistics.minimum.as_str(),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                ),
                TAB_SPACE,
                get_text("Std Dev:", false),
                get_text_size(
                    self.column_stadistics.std_dev.as_str(),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                ),
            ],
            row![
                get_text("Maximum:", false),
                get_text_size(
                    self.column_stadistics.maximum.as_str(),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                ),
                TAB_SPACE,
                get_text("Percentil:", false),
                get_text_size(
                    self.column_stadistics.percentil.as_str(),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                )
            ],
            row![
                get_text("Mode:", false),
                get_text_size(
                    self.column_stadistics.mode.as_str(),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                ),
                TAB_SPACE,
                get_text("Quatril:", false),
                get_text_size(
                    self.column_stadistics.quartil.as_str(),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                )
            ],
        ];
        let container_analysis =
            create_section_container_width(panel_column_analysis, PANEL_WIDTH * 2.0);

        let panel_coefficient = column![
            row![
                get_text_size(
                    self.header_checked
                        .get(0)
                        .map(|h| h.header.to_string())
                        .unwrap_or_else(|| "".to_string()),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                ),
                get_text_size(
                    self.header_checked
                        .get(1)
                        .map(|h| h.header.to_string())
                        .unwrap_or_else(|| "".to_string()),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                )
            ],
            row![TAB_SPACE, horizontal_space()],
            row![
                wrap_tooltip(
                    get_text("Pearson CC", false).into(),
                    "Pearson correlation coefficient"
                ),
                get_text_size(
                    self.correlation_file.pearson_correlation.to_string(),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                )
            ],
            row![
                wrap_tooltip(
                    get_text("Spearman CC", false).into(),
                    "Spearman correlation coefficient"
                ),
                get_text_size(
                    self.correlation_file.spearman_correlation.to_string(),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                )
            ],
            row![
                get_text("Covariance", false),
                get_text_size(
                    self.correlation_file.covariance.to_string(),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                )
            ],
            row!["Graph", TAB_SPACE, "Valor"]
        ];
        let container_correlation = create_section_container(panel_coefficient);

        row![
            container_file_details,
            horizontal_space(),
            container_analysis,
            horizontal_space(),
            container_correlation
        ]
    }

    fn build_action_menu(&self) -> Row<FastFileFlowMessage, Theme, iced::Renderer> {
        let button_open = get_menu_button(
            OPEN,
            FastFileFlowMessage::LoadFileButtonClick(false),
            OPEN_ICON,
        );

        let button_refresh = get_menu_button(
            REFRESH,
            FastFileFlowMessage::LoadFileButtonClick(true),
            REFRESH_ICON,
        );

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

        let error_label =
            row![get_text(&self.error_message, true)].padding(Padding::from([10.0, 0.0, 0.0, 0.0]));

        row![
            button_open,
            TAB_SPACE,
            button_refresh,
            TAB_SPACE,
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
            error_label
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

            table =
                table.on_column_resize(FastFileFlowMessage::Resizing, FastFileFlowMessage::Resized);

            table = table.min_width(size.width);
            //table = table.footer(self.footer.clone());

            table.into()
        });

        row![table].padding([0.0, 0.0, 5.0, 0.0])
    }

    fn build_status(&self) -> Row<FastFileFlowMessage, Theme, iced::Renderer> {
        let selected_file = Text::new(self.file_loaded.as_str())
            .width(Length::Fill)
            .size(Pixels(PANEL_FONT_SIZE));

        let loader = row![
            selected_file,
            horizontal_space(),
            if self.running {
                Linear::new(340.0, 15.0)
                    .easing(&easing::EMPHASIZED_ACCELERATE)
                    .cycle_duration(Duration::from_secs_f32(2_f32))
            } else {
                Linear::default()
            }
        ];

        loader
    }

    fn show_filter_screen(&self) -> Element<'_, FastFileFlowMessage, Theme, iced::Renderer> {
        let container_correlation = self.build_filter_panel();
        let container_analysis = self.build_filter_statistics();

        let render = row![
            container_correlation,
            TAB_SPACE,
            container_analysis,
            horizontal_space(),
            column![
                vertical_space(),
                if self.running {
                    Linear::new(340.0, 15.0)
                        .easing(&easing::EMPHASIZED_ACCELERATE)
                        .cycle_duration(Duration::from_secs_f32(2_f32))
                } else {
                    Linear::default()
                }
            ]
        ];
        let border = Border {
            color: Color::from_rgb(0.315, 0.315, 0.315).into(),
            width: 1.0,
            radius: 40.0.into(),
            ..Default::default()
        };

        container(render)
            .align_x(iced::alignment::Horizontal::Left)
            .align_y(iced::alignment::Vertical::Top)
            .padding(40.0)
            .style(container::Appearance {
                border,
                ..Default::default()
            })
            .into()
    }

    fn build_filter_statistics(&self) -> Container<FastFileFlowMessage, Theme, iced::Renderer> {
        let option_selected = self.column_option_selected.clone().unwrap_or_default();

        let default_statistics: IcedColumn = IcedColumn::new(option_selected.header.clone());
        let column_stadistics = self
            .columns
            .get(option_selected.index)
            .unwrap_or(&default_statistics)
            .stadistics
            .clone();

        let panel_column_analysis = column![
            row![get_text(option_selected.header.to_string(), true)
                .height(Length::Fixed(24.0))
                .width(Length::Fixed(PANEL_WIDTH - 13.0)),],
            row![
                get_text("Datatype:", false),
                get_text_size(
                    column_stadistics.data_type.to_string(),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                )
            ],
            row![
                get_text("Class:", false),
                get_text_size(
                    column_stadistics.classification.to_string(),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                )
            ],
            row![
                get_text("Minimum:", false),
                get_text_size(
                    column_stadistics.minimum.as_str(),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                )
            ],
            row![
                get_text("Maximum:", false),
                get_text_size(
                    column_stadistics.maximum.as_str(),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                )
            ],
            row![
                get_text("Mode:", false),
                get_text_size(
                    column_stadistics.mode.as_str(),
                    true,
                    Pixels(PANEL_FONT_SIZE)
                )
            ],
        ];
        let container_analysis = create_section_container_width(panel_column_analysis, PANEL_WIDTH);
        container_analysis
    }

    fn build_filter_panel(&self) -> Container<FastFileFlowMessage, Theme, iced::Renderer> {
        // let save_button =
        //     Button::new(Text::new("Save")).on_press(FastFileFlowMessage::Router(Page::Main));

        let close_button =
            Button::new(Text::new("Close")).on_press(FastFileFlowMessage::Router(Page::Main));

        let combo_box = combo_box(
            &self.column_options_state,
            "Choose a column",
            self.column_option_selected.as_ref(),
            FastFileFlowMessage::ColumnOptionSelected,
        )
        //.on_close(FastFileFlowMessage::ColumnOptionSelectedClosed)
        .size(12.0)
        .width(Length::Fill);

        let index = self
            .column_option_selected
            .clone()
            .unwrap_or_default()
            .index;

        let default_simple_column = SimpleColumn::default();
        let checkbox_ignore_if_empty = checkbox(
            "",
            self.column_options
                .get(index)
                .unwrap_or(&default_simple_column)
                .save_options
                .filter
                .ignore_if_empty,
        )
        .size(Pixels(14.0))
        .spacing(Pixels(1.0))
        .on_toggle(move |is_checked| FastFileFlowMessage::FilterIgnoreIfEmpty(index, is_checked));

        let checkbox_ignore_column = checkbox(
            "",
            self.column_options
                .get(index)
                .unwrap_or(&default_simple_column)
                .save_options
                .filter
                .ignore_column,
        )
        .size(Pixels(14.0))
        .spacing(Pixels(1.0))
        .on_toggle(move |is_checked| FastFileFlowMessage::FilterIgnoreColumn(index, is_checked));

        let panel_dropdown = column![
            row![combo_box],
            row![TAB_SPACE, horizontal_space()],
            row![
                get_text("Ignore Row if Empty", false),
                TAB_SPACE,
                (column![checkbox_ignore_if_empty]).padding(Padding::from([3, 0, 0, 0])),
            ],
            row![
                get_text("Ignore Column", false),
                TAB_SPACE,
                (column![checkbox_ignore_column]).padding(Padding::from([3, 0, 0, 0])),
            ],
            row![TAB_SPACE, horizontal_space()],
            row![
                TAB_SPACE,
                horizontal_space(), // save_button,
                close_button
            ],
        ];
        create_section_container(panel_dropdown)
    }

    fn get_column_stadistics_message(
        &mut self,
        column_index: usize,
        is_header_check: bool,
    ) -> Command<FastFileFlowMessage> {
        self.progress = 0.0;
        self.enable_loading(true);

        let current_stadistics = self
            .columns
            .get_mut(column_index)
            .unwrap()
            .stadistics
            .clone();

        if current_stadistics.classification == DataClassification::Unknown {
            let selected_file = self.selected_file.clone();
            Command::perform(
                async move { selected_file.get_stadistics(&column_index).await },
                move |stadistics_file| {
                    FastFileFlowMessage::SetStadisticsFile(
                        column_index,
                        stadistics_file,
                        is_header_check,
                    )
                },
            )
        } else {
            println!("Analisis estadistico realizado previamente");
            self.column_stadistics = current_stadistics.clone();
            self.enable_loading(false);
            Command::none()
        }
    }

    fn enable_loading(&mut self, activate: bool) {
        self.running = activate;
        if !self.running {
            self.progress = 0.0;
        }
    }

    fn reset_state(&mut self) {
        self.column_stadistics = Stadistics::default();
        self.correlation_file = CorrelationAnalysis::default();
        self.progress = 0.0;
        self.running = false;
        self.header_checked = vec![];
    }

    fn is_file_loaded(&self) -> bool {
        !self.file_loaded.is_empty()
    }

    fn router(&mut self, page: Page) {
        match page {
            Page::Main => {
                //self.theme = Theme::Dark;
                //self.is_primary_logo = false;
            }
            _ => {
                //     self.page = Page::Filter;
                //self.theme = Theme::CatppuccinLatte;
                //self.is_primary_logo = true;
            }
        }
        self.page = page;
    }
}

fn create_section_container(
    section: Column<FastFileFlowMessage, Theme, iced::Renderer>,
) -> Container<FastFileFlowMessage, Theme, iced::Renderer> {
    create_section_container_width(section, PANEL_WIDTH)
}

fn create_section_container_width(
    section: Column<FastFileFlowMessage, Theme, iced::Renderer>,
    width: f32,
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
        .width(width)
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
