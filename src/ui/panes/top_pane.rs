use crate::ui::{message::TopPaneMessage, utils::icon_button};

pub struct TopPane {}

impl<'a> TopPane {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> iced::Element<'a, TopPaneMessage> {
        iced::widget::row![
                self.view_left(),
                self.view_center(),
                self.view_right(),
            ]
            .into()
    }

    fn view_left(&self) -> iced::Element<'a, TopPaneMessage> {
        let row = iced::widget::row![
                icon_button(iced_fonts::Bootstrap::FileEarmarkImage).on_press(TopPaneMessage::AddImages),
                icon_button(iced_fonts::Bootstrap::Download).on_press(TopPaneMessage::Export),
            ];
        iced::widget::container(row)
            .align_left(iced::Fill)
            .into()
    }

    fn view_center(&self) -> iced::Element<'a, TopPaneMessage> {
        let row = iced::widget::row![
                icon_button(iced_fonts::Bootstrap::Funnel),
            ];
        iced::widget::container(row)
            .center_x(iced::Fill)
            .into()
    }

    fn view_right(&self) -> iced::Element<'a, TopPaneMessage> {
        let row = iced::widget::row![
                icon_button(iced_fonts::Bootstrap::ArrowCounterclockwise).on_press(TopPaneMessage::Undo),
                icon_button(iced_fonts::Bootstrap::ArrowClockwise).on_press(TopPaneMessage::Redo),
            ];
        iced::widget::container(row)
            .align_right(iced::Fill)
            .into()
    }
}