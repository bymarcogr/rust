use csv::WriterBuilder;
use futures::stream::StreamExt;
use std::{collections::HashMap, fs::remove_file};
use tokio::{fs::File, time::Instant};

use crate::{
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
            preview_enabled: bool::default(),
            max_preview_rows: 70,
        }
    }
    pub fn default() -> Self {
        Self {
            simple_column: vec![],
            stored_file: StoredFile::default(),
            preview: (vec![], vec![]),
            preview_enabled: bool::default(),
            max_preview_rows: 0,
        }
    }

    pub async fn save(&mut self) -> String {
        self.preview_enabled = false;
        let save_path = self.stored_file.get_export_path();

        match self
            .write_csv(save_path.clone(), self.stored_file.file_path.clone())
            .await
        {
            Ok(_) => save_path,
            Err(_) => "Error".to_string(),
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

        // Step 1
        let columns_ignore = self.get_ignore_column();
        let row_ignore_if_empty = self.get_ignored_row_if_empty_indexes();
        let row_ignore_if_value = self.get_ignored_row_if_value_indexes();
        let replace_with = self.get_replace_value_with();
        let replace_if_empty = self.get_replace_value_if_empty();
        let replace_with_trim = self.get_do_trim();
        let replace_if_value = self.get_replace_value_if_value();

        // Add headers
        let headers: Vec<String> = self
            .simple_column
            .iter()
            .filter(|f| filter_column_fn(&columns_ignore, f.index))
            .map(|s| s.header.to_string())
            .collect();

        let mut counter = self.max_preview_rows;
        let preview_enabled = self.preview_enabled;
        let mut preview_rows: Vec<Vec<String>> = vec![];
        let headers_clone = headers.clone();

        let handle_records = tokio::spawn(async move {
            let _ = wtr.serialize(&headers_clone);

            let mut records = rdr.records();
            let outer_loop_result: Result<(Vec<String>, Vec<Vec<String>>), std::io::Error> = 'outer: loop {
                while let Some(record) = records.next().await {
                    let record = record.unwrap();

                    let mut values: Vec<(usize, String)> = record
                        .iter()
                        .enumerate()
                        .map(|s| (s.0, s.1.to_string()))
                        .collect();

                    if ignore_row_if_empty(&values, &row_ignore_if_empty) {
                        continue;
                    }

                    if ignore_row_if_value(&values, &row_ignore_if_value) {
                        continue;
                    }

                    values = remove_columns(&values, &columns_ignore);
                    values = replace_do_trim(values, &replace_with_trim);
                    values = replace_value_if_empty(values, &replace_if_empty);
                    values = replace_value_with(values, &replace_with);
                    values = replace_value_if_equals(values, &replace_if_value);

                    let finals: Vec<String> = values.iter().map(|s| s.1.to_string()).collect();
                    if preview_enabled {
                        preview_rows.push(finals.clone());
                        counter -= 1;
                        if counter == 0 {
                            break 'outer Ok((headers.clone(), preview_rows));
                        }
                    }

                    let _ = wtr.serialize(&finals);
                }
                _ = wtr.flush();
                break Ok((headers, preview_rows));
            };
            outer_loop_result
        });

        let result = handle_records.await.unwrap();
        let duration = start.elapsed();
        println!("Exported Execution time: {:?}", duration.as_secs_f64());
        result
    }

    fn get_ignore_column(&self) -> Vec<usize> {
        self.simple_column
            .iter()
            .filter(|f| f.save_options.filter.ignore_column)
            .map(|item| item.index)
            .collect()
    }

    fn get_ignored_row_if_empty_indexes(&self) -> Vec<usize> {
        self.simple_column
            .iter()
            .filter(|f| f.save_options.filter.ignore_row_if_empty)
            .map(|item| item.index)
            .collect()
    }

    fn get_ignored_row_if_value_indexes(&self) -> HashMap<usize, String> {
        self.simple_column
            .iter()
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
            .iter()
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
            .iter()
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
            .iter()
            .filter(|f| f.save_options.process.trim)
            .map(|item| item.index)
            .collect()
    }

    fn get_replace_value_if_value(&self) -> HashMap<usize, (String, String)> {
        self.simple_column
            .iter()
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
        self.preview_enabled = true;
        let save_path = self.stored_file.get_export_path();
        let (columns, rows) = self
            .write_csv(save_path.clone(), self.stored_file.file_path.clone())
            .await
            .unwrap();

        let _ = remove_file(save_path);

        let iced_preview_columns: Vec<IcedColumn> = columns
            .iter()
            .map(|s| IcedColumn::new(s.to_string()))
            .collect();

        let iced_preview_rows: Vec<IcedRow> = rows
            .into_iter()
            .enumerate()
            .map(|(i, val)| IcedRow::new(val, i))
            .collect();

        (iced_preview_columns, iced_preview_rows)
    }
}

fn filter_column_fn(columns_ignore: &Vec<usize>, index: usize) -> bool {
    !columns_ignore.contains(&index)
}

fn ignore_row_if_empty(row: &Vec<(usize, String)>, ignore_enabled_index: &Vec<usize>) -> bool {
    if ignore_enabled_index.is_empty() {
        return false;
    }
    row.iter()
        .filter(|(i, _)| ignore_enabled_index.contains(i))
        .any(|(_, val)| val.is_empty())
}

fn ignore_row_if_value(
    row: &Vec<(usize, String)>,
    ignore_enabled_index: &HashMap<usize, String>,
) -> bool {
    if ignore_enabled_index.is_empty() {
        return false;
    }

    row.iter().any(|(i, val)| {
        ignore_enabled_index
            .get(i)
            .map_or(false, |expected_val| expected_val == val)
    })
}

fn replace_value_with(
    row: Vec<(usize, String)>,
    replace_index: &HashMap<usize, String>,
) -> Vec<(usize, String)> {
    if replace_index.is_empty() {
        return row;
    }

    row.into_iter()
        .map(|(i, v)| {
            replace_index
                .get(&i)
                .map(|new_value| (i, new_value.clone()))
                .unwrap_or((i, v))
        })
        .collect()
}

fn replace_value_if_empty(
    row: Vec<(usize, String)>,
    replace_index: &HashMap<usize, String>,
) -> Vec<(usize, String)> {
    if replace_index.is_empty() {
        return row;
    }

    row.into_iter()
        .map(|(i, v)| {
            if v.is_empty() {
                replace_index
                    .get(&i)
                    .map(|new_value| (i, new_value.clone()))
                    .unwrap_or((i, v))
            } else {
                (i, v)
            }
        })
        .collect()
}

fn replace_do_trim(row: Vec<(usize, String)>, replace_index: &Vec<usize>) -> Vec<(usize, String)> {
    if replace_index.is_empty() {
        return row;
    }

    row.into_iter()
        .map(|(i, v)| {
            if replace_index.contains(&i) {
                (i, v.trim().to_string())
            } else {
                (i, v)
            }
        })
        .collect()
}

fn replace_value_if_equals(
    row: Vec<(usize, String)>,
    replace_index: &HashMap<usize, (String, String)>,
) -> Vec<(usize, String)> {
    if replace_index.is_empty() {
        return row;
    }

    row.into_iter()
        .map(|(i, v)| {
            if let Some((current_value, new_value)) = replace_index.get(&i) {
                if &v == current_value {
                    (i, new_value.clone())
                } else {
                    (i, v)
                }
            } else {
                (i, v)
            }
        })
        .collect()
}

fn remove_columns(row: &Vec<(usize, String)>, columns_ignore: &Vec<usize>) -> Vec<(usize, String)> {
    row.iter()
        .filter(|(i, _)| !columns_ignore.contains(i))
        .map(|(i, s)| (*i, s.clone()))
        .collect()
}
