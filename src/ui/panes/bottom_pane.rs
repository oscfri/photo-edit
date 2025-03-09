use crate::ui::{message::BottomPaneMessage, utils::icon_button};

pub struct BottomPane {
    photo_id: Option<i32>
}

impl<'a> BottomPane {
    pub fn new(photo_id: Option<i32>) -> Self {
        Self { photo_id }
    }

    pub fn view(&self) -> iced::Element<'a, BottomPaneMessage> {
        let row = iced::widget::row![
                icon_button(iced_fonts::Bootstrap::ChevronLeft).on_press(BottomPaneMessage::PreviousImage),
                icon_button(iced_fonts::Bootstrap::Heart),
                icon_button(iced_fonts::Bootstrap::ChevronRight).on_press(BottomPaneMessage::NextImage),
                icon_button(iced_fonts::Bootstrap::Trashthree).on_press_maybe(self.photo_id.map(BottomPaneMessage::DeleteImage)),
            ];
        iced::widget::container(row)
            .center_x(iced::Fill)
            .into()
    }
}