use csv::WriterBuilder;
use futures::stream::StreamExt;
use tokio::{fs::File, time::Instant};

use crate::{dynamictable::simple_column::SimpleColumn, stored_file::StoredFile};

pub struct Export {
    pub stored_file: StoredFile,
    pub simple_column: Vec<SimpleColumn>,
}

impl Export {
    pub fn new(stored_file: StoredFile, simple_column: Vec<SimpleColumn>) -> Self {
        Self {
            simple_column,
            stored_file,
        }
    }
    pub fn default() -> Self {
        Self {
            simple_column: vec![],
            stored_file: StoredFile::default(),
        }
    }

    pub async fn save(&mut self) -> String {
        let save_path = self.stored_file.get_export_path();

        match self.write_csv(save_path.clone()).await {
            Ok(_) => save_path,
            Err(_) => "Error".to_string(),
        }
    }

    async fn write_csv(&self, file_path: String) -> Result<(), std::io::Error> {
        let start = Instant::now();
        let mut wtr = WriterBuilder::new().from_path(file_path)?;

        let mut rdr = csv_async::AsyncReader::from_reader(
            File::open(&self.stored_file.file_path).await.unwrap(),
        );

        // Step 1
        let columns_ignore = self.get_ignore_column();

        // Add headers
        let headers: Vec<String> = self
            .simple_column
            .iter()
            .filter(|f| filter_column_fn(&columns_ignore, f.index))
            .map(|s| s.header.to_string())
            .collect();

        let handle_records = tokio::spawn(async move {
            let _ = wtr.serialize(headers);

            let mut records = rdr.records();

            while let Some(record) = records.next().await {
                let record = record.unwrap();
                let values: Vec<String> = record
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| filter_column_fn(&columns_ignore, i.clone()))
                    .map(|s| s.1.to_string())
                    .collect();
                let _ = wtr.serialize(values);
            }

            wtr.flush()
        });

        let result = handle_records.await.unwrap();
        let duration = start.elapsed();
        println!("Exported Execution time: {:?}", duration.as_secs_f64());
        result
    }

    fn get_ignore_column(&self) -> Vec<usize> {
        self.simple_column
            .iter()
            .filter(|f| f.save_options.filter.ignore_column == true)
            .map(|item| item.index)
            .collect()
    }
}

fn filter_column_fn(columns_ignore: &Vec<usize>, index: usize) -> bool {
    !columns_ignore.contains(&index)
}
