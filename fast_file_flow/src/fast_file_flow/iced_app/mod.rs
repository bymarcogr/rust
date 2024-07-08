use crate::constants::english::*;
use crate::correlation_analysis::CorrelationAnalysis;
use crate::dynamictable::simple_column::SimpleColumn;
use crate::export::Export;
use crate::save_options::option_type::OptionType;
use crate::save_options::SaveOptions;
use crate::stadistics::data_classification::DataClassification;
use crate::stadistics::Stadistics;
use crate::stored_file::StoredFile;
use iced::widget::combo_box;
use iced::widget::scrollable;
use iced::Subscription;
use iced::{Command, Element, Theme};
use iced_futures::subscription;
use std::time::Duration;

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
                search_value: String::from(""),
                is_primary_logo: true,
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
                column_options: vec![],
                column_option_selected: None,
                column_options_state: combo_box::State::new(vec![]),
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
            FastFileFlowMessage::TextBoxChange(search_value) => {
                self.search_value = search_value;
                Command::none()
            }

            FastFileFlowMessage::Router(page) => {
                self.router(page);
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

                #[allow(unused_assignments)]
                let mut path = String::from("");
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
                    self.set_file_not_found_error();
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
                self.column_options = selected_file.get_simple_columns();
                self.column_options_state = combo_box::State::new(self.column_options.clone());

                self.selected_file = selected_file;
                self.enable_loading(false);
                Command::none()
            }

            FastFileFlowMessage::HeaderClicked(column_index) => {
                self.column_stadistics = Stadistics::default();
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
                self.column_stadistics = Stadistics::default();
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
                            save_options: SaveOptions::default(),
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
                if !self.is_file_loaded() {
                    self.set_file_not_found_error();
                } else {
                    self.router(Page::Filter);
                }
                Command::none()
            }
            FastFileFlowMessage::ProcessButtonClick() => {
                if !self.is_file_loaded() {
                    self.set_file_not_found_error();
                } else {
                    self.router(Page::Process);
                }
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
                    self.enable_loading(true);

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
                    self.enable_loading(false);
                    Command::none()
                }
            }

            FastFileFlowMessage::AnalysisCompleted(error) => {
                if !&error.is_empty() {
                    self.error_message = error;
                }
                self.enable_loading(false);
                Command::none()
            }
            FastFileFlowMessage::ColumnOptionSelected(option) => {
                self.column_option_selected = Some(option.clone());
                if option.classification == DataClassification::Unknown {
                    self.get_column_stadistics_message(option.index.clone(), false)
                } else {
                    Command::none()
                }
            }
            FastFileFlowMessage::ColumnOptionSelectedClosed() => {
                if let Some(column) = &self.column_option_selected {
                    println!("No hay Header {}", column.header);
                } else {
                    println!("No hay valor");
                }

                Command::none()
            }
            FastFileFlowMessage::FilterEvent(index, checked, option_type) => {
                if self.column_option_selected != None {
                    match option_type {
                        OptionType::FilterIgnoreIfEmpty => {
                            self.column_option_selected
                                .as_mut()
                                .unwrap()
                                .save_options
                                .filter
                                .ignore_if_empty = checked;

                            self.column_options
                                .get_mut(index)
                                .unwrap()
                                .save_options
                                .filter
                                .ignore_if_empty = checked;
                        }
                        OptionType::FilterIgnoreColumn => {
                            self.column_option_selected
                                .as_mut()
                                .unwrap()
                                .save_options
                                .filter
                                .ignore_column = checked;

                            self.column_options
                                .get_mut(index)
                                .unwrap()
                                .save_options
                                .filter
                                .ignore_column = checked;
                        }
                        _ => {}
                    };

                    self.column_options_state = combo_box::State::new(self.column_options.clone());
                }
                Command::none()
            }
            FastFileFlowMessage::ProcessEvent(index, checked, option_type) => {
                if self.column_option_selected != None {
                    match option_type {
                        OptionType::ProcessTrim => {
                            self.column_option_selected
                                .as_mut()
                                .unwrap()
                                .save_options
                                .process
                                .trim = checked;

                            self.column_options
                                .get_mut(index)
                                .unwrap()
                                .save_options
                                .process
                                .trim = checked;
                        }
                        OptionType::ProcessReplaceIfEmpty => {
                            self.column_option_selected
                                .as_mut()
                                .unwrap()
                                .save_options
                                .process
                                .replace_if_empty = checked;

                            self.column_options
                                .get_mut(index)
                                .unwrap()
                                .save_options
                                .process
                                .replace_if_empty = checked;
                        }
                        OptionType::ProcessReplaceIf => {
                            self.column_option_selected
                                .as_mut()
                                .unwrap()
                                .save_options
                                .process
                                .replace_if = checked;

                            self.column_options
                                .get_mut(index)
                                .unwrap()
                                .save_options
                                .process
                                .replace_if = checked;
                        }
                        OptionType::ProcessReplaceWith => {
                            self.column_option_selected
                                .as_mut()
                                .unwrap()
                                .save_options
                                .process
                                .replace_with = checked;

                            self.column_options
                                .get_mut(index)
                                .unwrap()
                                .save_options
                                .process
                                .replace_with = checked;
                        }
                        _ => {}
                    };
                    self.column_options_state = combo_box::State::new(self.column_options.clone());
                }
                Command::none()
            }
            FastFileFlowMessage::ProcessTextEvent(index, value, option_type) => {
                if self.column_option_selected != None {
                    match option_type {
                        OptionType::ProcessReplaceIfEmpty => {
                            self.column_option_selected
                                .as_mut()
                                .unwrap()
                                .save_options
                                .process
                                .replace_if_empty_value = value.clone();

                            self.column_options
                                .get_mut(index)
                                .unwrap()
                                .save_options
                                .process
                                .replace_if_empty_value = value.clone();
                        }
                        OptionType::ProcessReplaceIf => {
                            self.column_option_selected
                                .as_mut()
                                .unwrap()
                                .save_options
                                .process
                                .replace_if_value = value.clone();

                            self.column_options
                                .get_mut(index)
                                .unwrap()
                                .save_options
                                .process
                                .replace_if_value = value.clone();
                        }
                        OptionType::ProcessReplaceIfThen => {
                            self.column_option_selected
                                .as_mut()
                                .unwrap()
                                .save_options
                                .process
                                .replace_then_value = value.clone();

                            self.column_options
                                .get_mut(index)
                                .unwrap()
                                .save_options
                                .process
                                .replace_then_value = value.clone();
                        }
                        OptionType::ProcessReplaceWith => {
                            self.column_option_selected
                                .as_mut()
                                .unwrap()
                                .save_options
                                .process
                                .replace_with_value = value.clone();

                            self.column_options
                                .get_mut(index)
                                .unwrap()
                                .save_options
                                .process
                                .replace_with_value = value.clone();
                        }
                        _ => {}
                    };
                    self.column_options_state = combo_box::State::new(self.column_options.clone());
                }
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
                self.enable_loading(true);
                if self.is_file_loaded() {
                    println!("Inicia Export");
                    let mut export_file =
                        Export::new(self.selected_file.clone(), self.column_options.clone());
                    Command::perform(async move { export_file.save().await }, |saved_file| {
                        FastFileFlowMessage::ExportCompletedEvent(saved_file)
                    })
                } else {
                    self.set_file_not_found_error();
                    self.enable_loading(false);
                    Command::none()
                }
            }
            FastFileFlowMessage::ExportCompletedEvent(file_saved) => {
                println!("{file_saved}");
                self.enable_loading(false);
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

    fn view(&self) -> Element<Self::Message> {
        match self.page {
            Page::Main => self.show_main_screen(),
            Page::Filter => self.show_filter_screen(),
            Page::Process => self.show_process_screen(),
        }
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
