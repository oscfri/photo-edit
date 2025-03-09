use iced::widget::{container::Style, scrollable::{Direction, Scrollbar}};

use crate::{ui::message::ImageSelectionMessage, workspace::album_image::AlbumImage};

pub struct ImageSelectionPane<'a> {
    album_images: &'a Vec<AlbumImage>,
    selected_index: usize
}

impl <'a> ImageSelectionPane<'a> {
    pub fn new(album_images: &'a Vec<AlbumImage>, selected_index: usize) -> Self {
        Self {
            album_images,
            selected_index
        }
    }

    pub fn view(&self) -> iced::Element<'a, ImageSelectionMessage> {
        let thumbnails = self.album_images.iter().enumerate()
            .map(|(index, album_image)| self.view_thumbnail_image(index, &album_image))
            .collect();

        let row = iced::widget::Row::from_vec(thumbnails);

        let container = iced::widget::container(row);
        iced::widget::scrollable(container)
            .direction(Direction::Horizontal(Scrollbar::new()))
            .into()
    }

    fn view_thumbnail_image(&self, index: usize, album_image: &AlbumImage) -> iced::Element<'a, ImageSelectionMessage> {
        match &album_image.thumbnail {
            Some(thumbnail) => {
                let image_handle = iced::widget::image::Handle::from_rgba(
                    thumbnail.width as u32,
                    thumbnail.height as u32,
                    thumbnail.pixels.clone());
                self.wrap_thumbnail_area(index, iced::widget::image(image_handle)
                    .width(100)
                    .height(100)
                    .into())
            },
            None => {
                self.wrap_thumbnail_area(index, iced::widget::text("...").center()
                    .width(100)
                    .height(100)
                    .into())
            }
        }
    }

    fn wrap_thumbnail_area(&self, index: usize, element: iced::Element<'a, ImageSelectionMessage>) -> iced::Element<'a, ImageSelectionMessage> {
        let is_selected_thumbnail = self.selected_index == index;
        let container = iced::widget::container(element)
            .style(move |theme| {
                if is_selected_thumbnail {
                    let color = theme.palette().primary;
                    let background = iced::Background::Color(color);
                    Style {
                        background: Some(background),
                        ..Style::default()
                    }
                } else {
                    Style::default()
                }
            })
            .padding(5);
        iced::widget::mouse_area(container)
            .on_press(ImageSelectionMessage::SelectImage(index))
            .into()
    }
}