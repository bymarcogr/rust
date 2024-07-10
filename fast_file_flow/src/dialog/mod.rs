use native_dialog::FileDialog;

pub fn load_csv() -> String {
    let path = FileDialog::new()
        .add_filter("CSV Files|Fast File Flow Project", &["csv", "ffflow"])
        .set_location("~")
        .show_open_single_file()
        .unwrap();

    let path: String = match path {
        Some(path) => path.to_string_lossy().to_string(),
        None => String::from(""),
    };

    return path;
}

use std::error::Error;
use std::fs::File;
use std::path::Path;

pub async fn read_csv<P: AsRef<Path>>(filename: P, max: u8) -> Result<(), Box<dyn Error>> {
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);

    if let Ok(header) = rdr.byte_headers() {
        let _record = header;
    }

    let mut it: u8 = 0;
    for result in rdr.records() {
        let _record = result?;
        it = it + 1;
        if it == max {
            break;
        }
    }

    Ok(())
}

#[cfg(not(feature = "tokio"))]
use futures::stream::StreamExt;
#[cfg(feature = "tokio")]
use tokio::fs::File;
#[cfg(feature = "tokio")]
use tokio1 as tokio;
#[cfg(feature = "tokio")]
use tokio_stream::StreamExt;

pub async fn open_file_async(file_in: &str) -> Result<(), Box<dyn Error>> {
    let file_out: &str = &crate::util::add_processed_to_filename(file_in)
        .as_str()
        .to_owned();
    let mut rdr = csv_async::AsyncReader::from_reader(tokio::fs::File::open(file_in).await?);
    let mut wri = csv_async::AsyncWriter::from_writer(tokio::fs::File::create(file_out).await?);
    wri.write_record(rdr.headers().await?.into_iter()).await?;
    let mut records = rdr.records();

    let mut counter: u32 = 0;
    while let Some(record) = records.next().await {
        if counter == 100 {
            return Ok(());
        }
        counter = counter + 1;
        let record = record?;
        wri.write_record(&record).await?;
    }
    Ok(())
}
