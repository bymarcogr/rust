use csv::WriterBuilder;
use futures::stream::StreamExt;
use rayon::prelude::*;
use std::{collections::HashMap, fs::remove_file, time::Instant};
use tokio::fs::File;

use crate::{
    constants::english::ERROR_FILE_SAVE,
    dynamictable::{iced_column::IcedColumn, iced_row::IcedRow, simple_column::SimpleColumn},
    stored_file::StoredFile,
};

pub struct Export {
    pub stored_file: StoredFile,
    pub simple_column: Vec<SimpleColumn>,
    pub preview: (Vec<IcedColumn>, Vec<IcedRow>),
    preview_enabled: bool,
    max_preview_rows: usize,
}

impl Export {
    pub fn new(stored_file: StoredFile, simple_column: Vec<SimpleColumn>) -> Self {
        Self {
            simple_column,
            stored_file,
            preview: (vec![], vec![]),
            preview_enabled: false,
            max_preview_rows: 70,
        }
    }

    pub fn default() -> Self {
        Self {
            simple_column: vec![],
            stored_file: StoredFile::default(),
            preview: (vec![], vec![]),
            preview_enabled: false,
            max_preview_rows: 0,
        }
    }

    pub async fn save_file(&mut self, file_path: &str) -> String {
        self.preview_enabled = false;

        match self
            .write_csv(file_path.to_string(), self.stored_file.file_path.clone())
            .await
        {
            Ok(_) => file_path.to_string(),
            Err(_) => ERROR_FILE_SAVE.to_owned(),
        }
    }

    async fn write_csv(
        &self,
        save_path: String,
        open_path: String,
    ) -> Result<(Vec<String>, Vec<Vec<String>>), std::io::Error> {
        let start = Instant::now();
        let mut wtr = WriterBuilder::new().from_path(save_path)?;

        let mut rdr = csv_async::AsyncReader::from_reader(File::open(&open_path).await.unwrap());

        let columns_ignore = self.get_ignore_column();
        let row_ignore_if_empty = self.get_ignored_row_if_empty_indexes();
        let row_ignore_if_value = self.get_ignored_row_if_value_indexes();
        let replace_with = self.get_replace_value_with();
        let replace_if_empty = self.get_replace_value_if_empty();
        let replace_with_trim = self.get_do_trim();
        let replace_if_value = self.get_replace_value_if_value();

        let headers: Vec<String> = self
            .simple_column
            .par_iter()
            .filter(|f| !columns_ignore.contains(&f.index))
            .map(|s| s.header.to_string())
            .collect();

        let mut counter = self.max_preview_rows;
        let preview_enabled = self.preview_enabled;
        let mut preview_rows: Vec<Vec<String>> = vec![];
        let headers_clone = headers.clone();

        let handle_records = tokio::spawn(async move {
            let _ = wtr.serialize(&headers_clone);

            let mut records = rdr.records();
            let mut row_buffer = Vec::new();

            while let Some(record) = records.next().await {
                let record = record.unwrap();

                let mut values: Vec<(usize, String)> = record
                    .iter()
                    .enumerate()
                    .map(|(i, v)| (i, v.to_string()))
                    .collect();

                if ignore_row_if_empty(&values, &row_ignore_if_empty)
                    || ignore_row_if_value(&values, &row_ignore_if_value)
                {
                    continue;
                }

                for value in values.iter_mut() {
                    let (index, ref mut val) = *value;

                    if columns_ignore.contains(&index) {
                        continue;
                    }

                    if let Some(_new_value) = replace_with_trim.get(index) {
                        *val = val.trim().to_string();
                    }

                    if let Some(new_value) = replace_if_empty.get(&index) {
                        if val.is_empty() {
                            *val = new_value.clone();
                        }
                    }

                    if let Some(new_value) = replace_with.get(&index) {
                        *val = new_value.clone();
                    }

                    if let Some((current_value, new_value)) = replace_if_value.get(&index) {
                        if val == current_value {
                            *val = new_value.clone();
                        }
                    }
                }

                let finals: Vec<String> = values.into_iter().map(|(_, v)| v).collect();
                if preview_enabled {
                    preview_rows.push(finals.clone());
                    counter -= 1;
                    if counter == 0 {
                        break;
                    }
                }

                row_buffer.push(finals);
                if row_buffer.len() >= 1000 {
                    for row in row_buffer.drain(..) {
                        let _ = wtr.serialize(&row);
                    }
                }
            }

            for row in row_buffer {
                let _ = wtr.serialize(&row);
            }

            _ = wtr.flush();
            Ok((headers, preview_rows))
        });

        let result = handle_records.await.unwrap();
        crate::util::print_timer("Export CSV", start);
        result
    }

    fn get_ignore_column(&self) -> Vec<usize> {
        self.simple_column
            .par_iter()
            .filter(|f| f.save_options.filter.ignore_column)
            .map(|item| item.index)
            .collect()
    }

    fn get_ignored_row_if_empty_indexes(&self) -> Vec<usize> {
        self.simple_column
            .par_iter()
            .filter(|f| f.save_options.filter.ignore_row_if_empty)
            .map(|item| item.index)
            .collect()
    }

    fn get_ignored_row_if_value_indexes(&self) -> HashMap<usize, String> {
        self.simple_column
            .par_iter()
            .filter(|f| f.save_options.filter.ignore_row_if)
            .map(|item| {
                (
                    item.index,
                    item.save_options.filter.ignore_row_if_text.clone(),
                )
            })
            .collect()
    }

    fn get_replace_value_with(&self) -> HashMap<usize, String> {
        self.simple_column
            .par_iter()
            .filter(|f| f.save_options.process.replace_with)
            .map(|item| {
                (
                    item.index,
                    item.save_options.process.replace_with_value.clone(),
                )
            })
            .collect()
    }

    fn get_replace_value_if_empty(&self) -> HashMap<usize, String> {
        self.simple_column
            .par_iter()
            .filter(|f| f.save_options.process.replace_if_empty)
            .map(|item| {
                (
                    item.index,
                    item.save_options.process.replace_if_empty_value.clone(),
                )
            })
            .collect()
    }

    fn get_do_trim(&self) -> Vec<usize> {
        self.simple_column
            .par_iter()
            .filter(|f| f.save_options.process.trim)
            .map(|item| item.index)
            .collect()
    }

    fn get_replace_value_if_value(&self) -> HashMap<usize, (String, String)> {
        self.simple_column
            .par_iter()
            .filter(|f| f.save_options.process.replace_if)
            .map(|item| {
                (
                    item.index,
                    (
                        item.save_options.process.replace_if_value.clone(),
                        item.save_options.process.replace_then_value.clone(),
                    ),
                )
            })
            .collect()
    }

    pub async fn get_preview(&mut self) -> (Vec<IcedColumn>, Vec<IcedRow>) {
        let start = Instant::now();
        self.preview_enabled = true;
        let save_path = self.stored_file.get_export_path();
        let (columns, rows) = self
            .write_csv(save_path.clone(), self.stored_file.file_path.clone())
            .await
            .unwrap();

        let _ = remove_file(save_path);

        let iced_preview_columns: Vec<IcedColumn> = columns
            .par_iter()
            .map(|s| IcedColumn::new(s.to_string()))
            .collect();

        let iced_preview_rows: Vec<IcedRow> = rows
            .into_iter()
            .enumerate()
            .map(|(i, val)| IcedRow::new(val, i))
            .collect();

        crate::util::print_timer("Process and Preview", start);

        (iced_preview_columns, iced_preview_rows)
    }
}

fn ignore_row_if_empty(row: &[(usize, String)], ignore_enabled_index: &[usize]) -> bool {
    if ignore_enabled_index.is_empty() {
        return false;
    }
    row.iter()
        .any(|(i, val)| ignore_enabled_index.contains(i) && val.is_empty())
}

fn ignore_row_if_value(
    row: &[(usize, String)],
    ignore_enabled_index: &HashMap<usize, String>,
) -> bool {
    if ignore_enabled_index.is_empty() {
        return false;
    }

    row.iter().any(|(i, val)| {
        if let Some(expected_val) = ignore_enabled_index.get(i) {
            expected_val == val
        } else {
            false
        }
    })
}
