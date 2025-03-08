use crate::ui::message::BottomPaneMessage;

pub struct BottomPane {}

impl<'a> BottomPane {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> iced::Element<'a, BottomPaneMessage> {
        let row = iced::widget::row![
                iced::widget::button("<").on_press(BottomPaneMessage::PreviousImage),
                iced::widget::button("Heart"),
                iced::widget::button(">").on_press(BottomPaneMessage::NextImage),
            ];
        iced::widget::container(row)
            .center_x(iced::Fill)
            .into()
    }
}