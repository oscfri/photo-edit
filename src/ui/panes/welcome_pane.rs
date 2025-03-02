use crate::ui::message::{AlbumMessage, Message};

use iced;

pub struct WelcomePane {}

impl<'a> WelcomePane {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> iced::Element<'a, Message> {
        let column = iced::widget::column![
                    iced::widget::text("Get started by loading an image"),
                    iced::widget::button("Load").on_press(AlbumMessage::LoadAlbum.into())
                ]
                .spacing(30);

        iced::widget::container(column)
            .center(iced::Length::Fill)
            .into()
    }
}