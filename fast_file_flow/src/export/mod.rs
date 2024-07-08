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

        // Add headers
        let headers: Vec<String> = self
            .simple_column
            .iter()
            .map(|s| s.header.to_string())
            .collect();

        let handle_records = tokio::spawn(async move {
            let _ = wtr.serialize(headers);
            // let headers: Vec<String> = self
            //     .simple_column
            //     .iter()
            //     .map(|s| s.header.to_string())
            //     .collect();

            // // rdr
            // //     .headers()
            // //     .await
            //     .unwrap()
            //     .iter()
            //     .map(|s| s.to_string())
            //     .collect();

            let mut records = rdr.records();

            while let Some(record) = records.next().await {
                let record = record.unwrap();
                let values: Vec<String> = record.iter().map(|s| s.to_string()).collect();

                // if options.ignore_if_empty && record.name.is_empty() {
                //     continue;
                // }
                let _ = wtr.serialize(values);
            }

            wtr.flush()
        });

        let result = handle_records.await.unwrap();
        let duration = start.elapsed();
        println!("Exported Execution time: {:?}", duration.as_secs_f64());
        result
    }
}
