use crate::album;
use crate::ui::message::Message;

pub struct ImageSelectionPane<'a> {
    album_images: &'a Vec<album::AlbumImage>,
}

impl <'a> ImageSelectionPane<'a> {
    pub fn new(album_images: &'a Vec<album::AlbumImage>) -> Self {
        Self { album_images }
    }

    pub fn view(&self) -> iced::Element<'a, Message> {
        let thumbnails = self.album_images.iter().enumerate()
            .map(|(index, album_image)| self.view_thumbnail_image(index, &album_image))
            .collect();

        let row = iced::widget::Row::from_vec(thumbnails)
            .spacing(10);

        iced::widget::container(row)
            .padding(10)
            .height(120) // TODO: It should be the windows responsibility to set the height
            .into()
    }

    fn view_thumbnail_image(&self, index: usize, album_image: &album::AlbumImage) -> iced::Element<'a, Message> {
        let image_handle = iced::widget::image::Handle::from_rgba(
            album_image.thumbnail.width as u32,
            album_image.thumbnail.height as u32,
            album_image.thumbnail.pixels.clone());
        iced::widget::mouse_area(iced::widget::image(image_handle))
            .on_press(Message::SetImage(index))
            .into()
    }
}