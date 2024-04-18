pub mod util {
    fn get_icon_from_file() -> Result<iced::window::Icon, iced::window::icon::Error> {
        let mut icon =
            image::io::Reader::new(std::io::Cursor::new(include_bytes!("resources\\icon.png")));

        icon.set_format(image::ImageFormat::Png);

        let icon_with_format: image::io::Reader<std::io::Cursor<&[u8; 1060]>> = icon;

        let pixels = icon_with_format
            .decode()
            .expect("Failed to decode icon")
            .to_rgba8();

        Ok(iced::window::icon::from_rgba(
            pixels.to_vec(),
            pixels.width(),
            pixels.height(),
        )?)
    }

    pub fn get_icon() -> iced::window::Icon { get_icon_from_file().expect("Failed to load icon") }
}
