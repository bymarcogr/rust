mod constants;
pub mod util {
    use iced::widget::image::{ Image, Handle };
    use iced::window::{Icon,icon::Error};
    use iced::Length::Fixed;

    use super::constants::english::{ERROR_GET_FOLDER, ERROR_LOAD_ICON};
    use super::constants::path::{LOGO_PATH};


    fn get_icon_from_file() -> Result<Icon, Error> {
        let mut icon =
            image::io::Reader::new(std::io::Cursor::new(include_bytes!("resources/icon.png")));

        icon.set_format(image::ImageFormat::Png);

        let icon_with_format: image::io::Reader<std::io::Cursor<&[u8; 1060]>> = icon;

        let pixels = icon_with_format
            .decode()
            .expect(ERROR_LOAD_ICON)
            .to_rgba8();

        Ok(iced::window::icon::from_rgba(
            pixels.to_vec(),
            pixels.width(),
            pixels.height(),
        )?)
    }

    pub fn get_icon() -> Icon { get_icon_from_file().expect(ERROR_GET_FOLDER) }

    pub fn get_logo() -> Image<Handle> {
       let path = get_full_directory();
      
        let image = Image::new(  format!("{path}{LOGO_PATH}"))  
        .width(Fixed(250.0))
        .height(Fixed(188.0));
        return image;
    }

    pub fn get_full_directory() -> String {
        let folder = std::env::current_dir().expect(ERROR_GET_FOLDER);
       return folder.display().to_string();
    }
}
