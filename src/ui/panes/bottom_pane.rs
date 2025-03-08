use crate::ui::{message::BottomPaneMessage, utils::icon_button};

pub struct BottomPane {}

impl<'a> BottomPane {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> iced::Element<'a, BottomPaneMessage> {
        let row = iced::widget::row![
                icon_button(iced_fonts::Bootstrap::ChevronLeft).on_press(BottomPaneMessage::PreviousImage),
                icon_button(iced_fonts::Bootstrap::Heart),
                icon_button(iced_fonts::Bootstrap::ChevronRight).on_press(BottomPaneMessage::NextImage),
                icon_button(iced_fonts::Bootstrap::Trashthree).on_press(BottomPaneMessage::DeleteImage),
            ];
        iced::widget::container(row)
            .center_x(iced::Fill)
            .into()
    }
}