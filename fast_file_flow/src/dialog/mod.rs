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
