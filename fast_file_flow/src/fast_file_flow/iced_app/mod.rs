use crate::ai::AiModel;
use crate::constants::english;
use crate::constants::english::*;
use crate::constants::path::CSV;
use crate::constants::path::DBSCAN_IMAGE_RESULT;
use crate::constants::path::FFFLOW;
use crate::constants::path::KMEANS_IMAGE_RESULT;
use crate::constants::path::LR_IMAGE_RESULT;
use crate::constants::path::PCA_IMAGE_RESULT;
use crate::correlation_analysis::CorrelationAnalysis;
use crate::dynamictable::simple_column::SimpleColumn;
use crate::export::Export;
use crate::save_options::option_type::OptionType;
use crate::save_options::SaveOptions;
use crate::stadistics::data_classification::DataClassification;
use crate::stadistics::Stadistics;
use crate::stored_file::StoredFile;
use crate::util::print_timer;
use iced::widget::combo_box;
use iced::widget::scrollable;
use iced::Subscription;
use iced::{Command, Element};
use iced_futures::subscription;
use iced_widget::text_editor::Content;
use native_dialog::FileDialog;
use std::time::Duration;
use std::time::Instant;

use super::FastFileFlow;
use super::FastFileFlowMessage;
use super::Page;

impl iced::Application for FastFileFlow {
    type Executor = iced::executor::Default;
    type Flags = ();
    type Message = FastFileFlowMessage;
    type Theme = iced::Theme;

    fn new(_flags: ()) -> (FastFileFlow, Command<Self::Message>) {
        (FastFileFlow::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from(APP_TITLE)
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.notification_message = String::from("");

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
                self.router(Page::UserAboutIt);

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

                if !path.is_empty() {
                    let extension = StoredFile::get_file_extension(&path);

                    if extension == CSV {
                        self.file_loaded = path.clone();
                        Command::perform(
                            async move {
                                let start = Instant::now();
                                let file = StoredFile::new(path.clone()).await;
                                print_timer("File Loading", start);
                                file
                            },
                            |stored_file| FastFileFlowMessage::SetSelectedFile(stored_file),
                        )
                    } else if extension == FFFLOW {
                        match self.load_from_file(path.as_str()) {
                            Ok(_) => Command::perform(async move {}, |_| {
                                FastFileFlowMessage::SetLoadedProject()
                            }),
                            Err(_) => {
                                self.enable_loading(false);
                                self.set_error(&ERROR_PROJECT_INVALID.to_owned());
                                self.reset_state();
                                Command::none()
                            }
                        }
                    } else {
                        Command::none()
                    }
                } else {
                    self.enable_loading(false);
                    self.set_file_not_found_error();
                    Command::none()
                }
            }

            FastFileFlowMessage::Tick(progress) => {
                if self.running {
                    self.progress = progress;
                }
                Command::none()
            }

            FastFileFlowMessage::SetSelectedFile(selected_file) => {
                if selected_file.sintaxis.clone() != crate::stored_file::file_type::FileType::CSV {
                    self.notification_message = format!(
                        "File sintaxis {}, it is not supported yet, please use a valid csv",
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
            FastFileFlowMessage::SetLoadedProject() => {
                self.selected_file.file_name = StoredFile::get_file_name(&self.file_loaded);
                let future = self.selected_file.reload();

                match futures::executor::block_on(future) {
                    Ok(_) => {
                        self.reset_state();

                        self.rows = self.selected_file.rows.sample.clone();
                        self.columns = self.selected_file.columns.headers.clone();
                        self.column_options_state =
                            combo_box::State::new(self.column_options.clone());
                        self.enable_loading(false);
                    }
                    Err(e) => {
                        self.selected_file = StoredFile::default();
                        self.file_loaded = String::default();
                        self.reset_state();
                        self.set_error(&e.to_string());
                        self.enable_loading(false);
                    }
                }
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
                    let item_deselect = self.header_checked.pop().unwrap().clone();
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

            FastFileFlowMessage::ShowFilterButtonClick() => {
                if !self.is_file_loaded() {
                    self.set_file_not_found_error();
                } else {
                    self.router(Page::Filter);
                }
                Command::none()
            }
            FastFileFlowMessage::ShowProcessButtonClick() => {
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
                if !self.is_file_loaded() {
                    self.set_file_not_found_error();
                    return Command::none();
                }

                if self.header_checked.len() == 2_usize {
                    self.enable_loading(true);

                    let selected_file = self.selected_file.clone();
                    let (is_quantitative, column_base, column_compare) =
                        FastFileFlow::is_quantitative_by_header(self.header_checked.clone());

                    if is_quantitative {
                        Command::perform(
                            async move {
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
                        self.notification_message = ERROR_QUANTITATIVE_COLUMNS.to_string();
                        self.enable_loading(false);
                        Command::none()
                    }
                } else {
                    self.notification_message = ERROR_QUANTITATIVE_COLUMNS.to_string();
                    self.enable_loading(false);
                    Command::none()
                }
            }

            FastFileFlowMessage::AnalysisCompleted(error) => {
                if !&error.is_empty() {
                    self.notification_message = error;
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
            FastFileFlowMessage::ColumnOptionSelectedClosed() => Command::none(),
            FastFileFlowMessage::FilterEvent(index, checked, option_type) => {
                if self.column_option_selected != None {
                    match option_type {
                        OptionType::FilterIgnoreIfEmpty => {
                            self.column_option_selected
                                .as_mut()
                                .unwrap()
                                .save_options
                                .filter
                                .ignore_row_if_empty = checked;

                            self.column_options
                                .get_mut(index)
                                .unwrap()
                                .save_options
                                .filter
                                .ignore_row_if_empty = checked;
                        }
                        OptionType::FilterIgnoreIf => {
                            self.column_option_selected
                                .as_mut()
                                .unwrap()
                                .save_options
                                .filter
                                .ignore_row_if = checked;

                            self.column_options
                                .get_mut(index)
                                .unwrap()
                                .save_options
                                .filter
                                .ignore_row_if = checked;
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
            FastFileFlowMessage::FilterTextEvent(index, value, option_type) => {
                if self.column_option_selected != None {
                    match option_type {
                        OptionType::FilterIgnoreIf => {
                            self.column_option_selected
                                .as_mut()
                                .unwrap()
                                .save_options
                                .filter
                                .ignore_row_if_text = value.clone();

                            self.column_options
                                .get_mut(index)
                                .unwrap()
                                .save_options
                                .filter
                                .ignore_row_if_text = value.clone();
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
            FastFileFlowMessage::ShowPreviewButtonClick() => {
                if self.is_file_loaded() {
                    let mut export_file =
                        Export::new(self.selected_file.clone(), self.column_options.clone());
                    Command::perform(
                        async move { export_file.get_preview().await },
                        |(preview_headers, preview_rows)| {
                            FastFileFlowMessage::PreviewCompleted(preview_headers, preview_rows)
                        },
                    )
                } else {
                    self.set_file_not_found_error();
                    self.enable_loading(false);
                    Command::none()
                }
            }
            FastFileFlowMessage::SaveProjectButtonClick() => {
                if self.is_file_loaded() {
                    if let Some(path) = FileDialog::new()
                        .add_filter(
                            english::DIALOG_LOAD_PROJECT_TITLE,
                            &[DIALOG_PROJECT_EXTENSION],
                        )
                        .set_filename(format!("project.{}", DIALOG_PROJECT_EXTENSION).as_str())
                        .show_save_single_file()
                        .ok()
                        .flatten()
                    {
                        let start = Instant::now();
                        let _ = self.save_to_file(path.to_str().unwrap());
                        crate::util::print_timer("Save Project ", start);
                    }
                } else {
                    self.set_file_not_found_error();
                    self.enable_loading(false);
                }
                Command::none()
            }
            FastFileFlowMessage::ExportButtonClick() => {
                self.enable_loading(true);
                if self.is_file_loaded() {
                    if let Some(path) = FileDialog::new()
                        .add_filter(
                            english::DIALOG_FILE_EXTENSION_CSV,
                            &[DIALOG_FILE_EXTENSION_CSV],
                        )
                        .set_filename(format!(".{}", DIALOG_FILE_EXTENSION_CSV).as_str())
                        .show_save_single_file()
                        .ok()
                        .flatten()
                    {
                        let mut export_file =
                            Export::new(self.selected_file.clone(), self.column_options.clone());
                        Command::perform(
                            async move { export_file.save_file(path.to_str().unwrap()).await },
                            |saved_file| FastFileFlowMessage::ExportCompletedEvent(saved_file),
                        )
                    } else {
                        self.enable_loading(false);
                        Command::none()
                    }
                } else {
                    self.set_file_not_found_error();
                    self.enable_loading(false);
                    Command::none()
                }
            }
            FastFileFlowMessage::ExportCompletedEvent(file_saved) => {
                self.enable_loading(false);
                self.notification_message = format!("File Saved: {file_saved}");
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
            FastFileFlowMessage::PreviewCompleted(headers, rows) => {
                self.columns_backup = self.columns.clone();
                self.header_checked_backup = self.header_checked.clone();
                self.columns = headers;
                self.rows = rows;
                self.router(Page::Preview);
                Command::none()
            }
            FastFileFlowMessage::ShowAIButtonClick() => {
                if !self.is_file_loaded() {
                    self.set_file_not_found_error();
                    return Command::none();
                }

                if self.header_checked.len() != 2_usize {
                    self.notification_message = ERROR_QUANTITATIVE_COLUMNS.to_string();
                    self.enable_loading(false);
                    return Command::none();
                }

                let (is_quantitative, _, _) =
                    FastFileFlow::is_quantitative_by_header(self.header_checked.clone());

                if is_quantitative {
                    self.router(Page::AI);
                    self.ai_result = "".to_string();
                    self.ai_image = "".to_string();
                    self.result_content = Content::new();
                } else {
                    self.notification_message = ERROR_QUANTITATIVE_COLUMNS.to_string();
                    self.enable_loading(false);
                }

                Command::none()
            }
            FastFileFlowMessage::AIAnalysisEvent(model) => {
                self.enable_loading(true);
                let mut header = self.header_checked.clone();

                let column_compare = header.pop().unwrap();
                let column_base = header.pop().unwrap();
                let mut selected_file = self.selected_file.clone();
                self.ai_image = String::default();
                self.ai_result = String::default();

                Command::perform(
                    async move {
                        match model {
                            AiModel::KMeans => {
                                selected_file
                                    .get_kmeans(&column_base, &column_compare, &3, &500)
                                    .await
                            }
                            AiModel::PCA => {
                                selected_file
                                    .get_pca_analysis(&column_base, &column_compare, 2)
                                    .await
                            }
                            AiModel::DbScan => {
                                selected_file
                                    .get_dbscan_analysis(&column_base, &column_compare, 1.0, 4)
                                    .await
                            }
                            AiModel::LRegression => {
                                selected_file
                                    .get_ln_rg_prediction(&column_base, &column_compare)
                                    .await
                            }
                        }
                    },
                    move |item| match item {
                        Ok(value) => FastFileFlowMessage::AICompleted(
                            model.to_owned(),
                            value.to_string(),
                            false,
                        ),
                        Err(e) => {
                            FastFileFlowMessage::AICompleted(model.to_owned(), e.to_string(), true)
                        }
                    },
                )
            }
            FastFileFlowMessage::AICompleted(model, result, is_error) => {
                if is_error {
                    self.set_error(result.as_str());
                    self.ai_image = "".to_string();
                    self.ai_result = "".to_string();
                } else {
                    self.ai_image = match model {
                        AiModel::KMeans => KMEANS_IMAGE_RESULT.to_owned(),
                        AiModel::PCA => PCA_IMAGE_RESULT.to_owned(),
                        AiModel::DbScan => DBSCAN_IMAGE_RESULT.to_owned(),
                        AiModel::LRegression => LR_IMAGE_RESULT.to_owned(),
                    };

                    self.ai_result = result;
                    self.router(Page::AI);
                }
                self.result_content = Content::with_text(&self.ai_result);
                self.enable_loading(false);

                Command::none()
            }
            FastFileFlowMessage::PreviewButtonCloseClick() => {
                self.columns = self.columns_backup.clone();
                self.columns_backup = vec![];
                self.header_checked = self.header_checked_backup.clone();
                self.header_checked_backup = vec![];
                self.rows = self.selected_file.rows.sample.to_owned();
                self.router(Page::Main);
                Command::none()
            }
            FastFileFlowMessage::ActionPerformed(action) => {
                self.result_content.perform(action);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        match self.page {
            Page::Main => self.show_main_screen(),
            Page::Filter => self.show_filter_screen(),
            Page::Process => self.show_process_screen(),
            Page::AI => self.show_ai_screen(),
            Page::Preview => self.show_preview_screen(),
            Page::UserAboutIt => self.show_user_screen(),
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
        } else {
            Subscription::none()
        }
    }
}
