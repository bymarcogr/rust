use crate::constants::english::*;
use crate::correlation_analysis::CorrelationAnalysis;
use crate::dynamictable::simple_column::SimpleColumn;
use crate::stadistics::Stadistics;
use crate::stored_file::StoredFile;
use iced::Subscription;
use iced::{Command, Element, Theme};
use iced_futures::subscription;
use std::time::Duration;

use iced::widget::scrollable;

use super::config_page;
use super::main_page;
use super::FastFileFlow;
use super::FastFileFlowMessage;
use super::Page;

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
                theme: Theme::GruvboxLight,
                input_value: String::from(""),
                is_primary_logo: false,
                clicked_button: String::from(""),
                selected_file: StoredFile::default(),
                column_stadistics: Stadistics::default(),
                correlation_file: CorrelationAnalysis::default(),
                header: scrollable::Id::unique(),
                body: scrollable::Id::unique(),
                footer: scrollable::Id::unique(),
                columns: vec![],
                rows: vec![],
                file_loaded: String::from(""),

                progress: 0.0,
                running: false,
                header_checked: vec![],
                error_message: String::from(""),
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
        self.error_message = String::from("");

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
            FastFileFlowMessage::LoadFileButtonClick(is_refresh) => {
                self.enable_loading(true);

                let mut path = String::default();
                if is_refresh {
                    path = String::from(self.file_loaded.clone());
                } else {
                    path = crate::dialog::load_csv();
                }

                if path != "" {
                    self.file_loaded = path.clone();

                    Command::perform(StoredFile::new(String::from(path)), |stored_file| {
                        FastFileFlowMessage::SetSelectedFile(stored_file)
                    })
                } else {
                    self.enable_loading(false);
                    self.error_message =
                        "Selecciona un archivo CSV para utilizar esta funcion".to_string();
                    Command::none()
                }
            }

            FastFileFlowMessage::Tick(progress) => {
                if self.running {
                    self.progress = progress;
                    //println!("Tick - {}", progress)
                }
                Command::none()
            }
            FastFileFlowMessage::SetSelectedFile(selected_file) => {
                if selected_file.sintaxis.clone() != crate::stored_file::file_type::FileType::CSV {
                    self.error_message = format!(
                        "Sintaxis {} en el archivo no es compatible, seleccione un archivo CSV válido",
                        &selected_file.sintaxis.to_string()
                    )
                    .to_string();
                }
                self.reset_state();
                self.rows = selected_file.rows.sample.clone();
                self.columns = selected_file.columns.headers.clone();
                self.selected_file = selected_file;
                self.enable_loading(false);
                Command::none()
            }

            FastFileFlowMessage::HeaderClicked(column_index) => {
                self.get_column_stadistics_message(column_index, false)
            }
            FastFileFlowMessage::SetStadisticsFile(index, stadistics_file, is_header_checkbox) => {
                if is_header_checkbox {
                    self.header_checked
                        .iter_mut()
                        .find(|s| s.index == index)
                        .unwrap()
                        .classification = stadistics_file.classification.clone();
                }
                self.column_stadistics = stadistics_file.clone();
                self.columns.get_mut(index).unwrap().stadistics = stadistics_file;
                self.enable_loading(false);
                Command::none()
            }
            FastFileFlowMessage::HeaderCheckBoxToggled(index, toggle) => {
                self.correlation_file = CorrelationAnalysis::default();
                if toggle {
                    if self.header_checked.len() == 2_usize {
                        let item_deselect = self.header_checked.pop();
                        self.columns
                            .get_mut(item_deselect.unwrap().index)
                            .unwrap()
                            .is_checked = false;
                    }

                    if self.header_checked.len() <= 1_usize {
                        self.enable_loading(true);
                        let column = self.columns.get_mut(index).unwrap();

                        self.header_checked.push(SimpleColumn {
                            index,
                            header: column.column_header.clone(),
                            classification: column.stadistics.classification.clone(),
                        });

                        column.is_checked = toggle;
                    }

                    let com1: iced::Command<FastFileFlowMessage> =
                        self.get_column_stadistics_message(index, true);

                    iced::Command::batch(vec![com1])
                } else {
                    let item_deselect = self.header_checked.pop().unwrap();
                    if item_deselect.index == index {
                        self.columns
                            .get_mut(item_deselect.index)
                            .unwrap()
                            .is_checked = false;
                    } else {
                        let remove = self.header_checked.pop().unwrap();
                        self.header_checked.push(item_deselect);
                        self.columns.get_mut(remove.index).unwrap().is_checked = false;
                    }

                    Command::none()
                }
            }
            FastFileFlowMessage::SetCorrelationFile(correlation_file) => {
                self.correlation_file = correlation_file;
                self.enable_loading(false);
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
                if self.header_checked.len() == 2_usize {
                    println!("Inicia Analisis");
                    self.running = true;

                    let mut header = self.header_checked.clone();
                    let column_compare = header.pop().unwrap();
                    let column_base = header.pop().unwrap();
                    let selected_file = self.selected_file.clone();
                    Command::perform(
                        async move {
                            println!("get_correlation");
                            selected_file
                                .get_correlation(&column_base.clone(), &column_compare.clone())
                                .await
                        },
                        |correlation_file| match correlation_file {
                            Ok(value) => FastFileFlowMessage::SetCorrelationFile(value),
                            Err(e) => FastFileFlowMessage::AnalysisCompleted(e.to_string()),
                        },
                    )
                } else {
                    self.error_message =
                        "Selecciona dos columnas del tipo Cuantitativo".to_string();
                    self.running = false;
                    Command::none()
                }
            }
            FastFileFlowMessage::AnalysisCompleted(error) => {
                if !&error.is_empty() {
                    self.error_message = error;
                }
                self.running = false;
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

        self.build_main_screen()
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        if self.running {
            subscription::unfold("progress", self.progress, move |progress| async move {
                tokio::time::sleep(Duration::from_millis(50)).await;
                let mut new_progress = progress + 1.0 as f32;
                if new_progress == 100.0 {
                    new_progress = 1.0 as f32;
                }
                (FastFileFlowMessage::Tick(new_progress), new_progress)
            })

            // todo() -> Agregar lectura de estado global de la aplicacion
        } else {
            println!("exit");
            Subscription::none()
        }
    }
}
