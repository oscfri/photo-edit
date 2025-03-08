use crate::ui::message::TopPaneMessage;

pub struct TopPane {}

impl<'a> TopPane {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> iced::Element<'a, TopPaneMessage> {
        iced::widget::row![
                iced::widget::button("+").on_press(TopPaneMessage::LoadAlbum),
                iced::widget::button("Export").on_press(TopPaneMessage::Export),
            ]
            .into()
    }
}