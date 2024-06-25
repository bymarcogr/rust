use native_dialog::FileDialog;

pub fn load_csv() -> String {
    let path = FileDialog::new()
        .add_filter("CSV Files", &["csv"])
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

    for header in rdr.byte_headers() {
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
