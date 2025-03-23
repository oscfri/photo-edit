use crate::ui::{message::TopPaneMessage, utils::icon_button};

pub struct TopPane {
    is_filter_active: bool
}

impl<'a> TopPane {
    pub fn new(is_filter_active: bool) -> Self {
        Self {
            is_filter_active
        }
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
                icon_button(iced_fonts::Nerd::FileImagePlus).on_press(TopPaneMessage::AddImages),
                icon_button(iced_fonts::Nerd::Download).on_press(TopPaneMessage::Export),
            ];
        iced::widget::container(row)
            .align_left(iced::Fill)
            .into()
    }

    fn view_center(&self) -> iced::Element<'a, TopPaneMessage> {
        let row = iced::widget::row![
                icon_button(self.make_filter_icon()).on_press(TopPaneMessage::ToggleFilter),
            ];
        iced::widget::container(row)
            .center_x(iced::Fill)
            .into()
    }

    fn view_right(&self) -> iced::Element<'a, TopPaneMessage> {
        let row = iced::widget::row![
                icon_button(iced_fonts::Nerd::UndoOne).on_press(TopPaneMessage::Undo),
                icon_button(iced_fonts::Nerd::RedoOne).on_press(TopPaneMessage::Redo),
            ];
        iced::widget::container(row)
            .align_right(iced::Fill)
            .into()
    }

    fn make_filter_icon(&self) -> iced_fonts::Nerd {
        if self.is_filter_active {
            iced_fonts::Nerd::FilterFilled
        } else {
            iced_fonts::Nerd::Filter
        }
    }
}