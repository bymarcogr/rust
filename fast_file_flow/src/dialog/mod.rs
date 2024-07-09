use native_dialog::FileDialog;

pub fn load_csv() -> String {
    let path = FileDialog::new()
        .add_filter("CSV Files|Fast File Flow Project", &["csv", "ffflow"])
        .set_location("~")
        .show_open_single_file()
        .unwrap();

    let path: String = match path {
        Some(path) => path.to_string_lossy().to_string(),
        None => "".to_string(),
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
        let record = header;
        println!("{:?}", record);
    }

    let mut it: u8 = 0;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
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

// pub async fn filter_by_region(
//     region: &str,
//     file_in: &str,
//     file_out: &str,
// ) -> Result<(), Box<dyn Error>> {
//     // Function reads CSV file that has column named "region" at second position (index = 1).
//     // It writes to new file only rows with region equal to passed argument
//     // and removes region column.
//     let mut rdr = csv_async::AsyncReader::from_reader(tokio::fs::File::open(file_in).await?);
//     let mut wri = csv_async::AsyncWriter::from_writer(tokio::fs::File::create(file_out).await?);
//     wri.write_record(rdr.headers().await?.into_iter().filter(|h| *h != "region"))
//         .await?;
//     let mut records = rdr.records();
//     while let Some(record) = records.next().await {
//         let record = record?;
//         match record.get(1) {
//             Some(reg) if reg == region => {
//                 wri.write_record(
//                     record
//                         .iter()
//                         .enumerate()
//                         .filter(|(i, _)| *i != 1)
//                         .map(|(_, s)| s),
//                 )
//                 .await?
//             }
//             _ => {}
//         }
//     }
//     Ok(())
// }

pub async fn open_file_async(file_in: &str) -> Result<(), Box<dyn Error>> {
    // Function reads CSV file that has column named "region" at second position (index = 1).
    // It writes to new file only rows with region equal to passed argument
    // and removes region column.

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
        wri.write_record(
            // record
            //     .iter()
            //     .enumerate() //.filter(|(i, _)| *i != 1)
            //     .map(|(_, s)| s),
            &record,
        )
        .await?;
    }
    Ok(())
}
