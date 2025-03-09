use crate::ui::message::WelcomeMessage;

use iced;

pub struct WelcomePane {}

impl<'a> WelcomePane {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> iced::Element<'a, WelcomeMessage> {
        let column = iced::widget::column![
                    iced::widget::text("Get started by loading an image"),
                    iced::widget::button("Load").on_press(WelcomeMessage::AddImages)
                ]
                .spacing(30);

        iced::widget::container(column)
            .center(iced::Length::Fill)
            .into()
    }
}