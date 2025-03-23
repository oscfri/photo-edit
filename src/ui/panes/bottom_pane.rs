use crate::ui::{message::BottomPaneMessage, utils::icon_button};

pub struct BottomPane {
    photo_id: Option<i32>,
    parameters_visible: bool,
    is_favorite: bool,
    can_reset_view: bool
}

impl<'a> BottomPane {
    pub fn new(
            photo_id: Option<i32>,
            parameters_visible: bool,
            is_favorite: bool,
            can_reset_view: bool) -> Self {
        Self { photo_id, parameters_visible, is_favorite, can_reset_view }
    }

    pub fn view(&self) -> iced::Element<'a, BottomPaneMessage> {
        iced::widget::row![
                self.view_left(),
                self.view_center(),
                self.view_right(),
            ]
            .into()
    }

    fn view_left(&self) -> iced::Element<'a, BottomPaneMessage> {
        let row = iced::widget::row![
            ];
        iced::widget::container(row)
            .align_left(iced::Fill)
            .into()
    }

    fn view_center(&self) -> iced::Element<'a, BottomPaneMessage> {
        let row = iced::widget::row![
                icon_button(iced_fonts::Nerd::ChevronLeft).on_press(BottomPaneMessage::PreviousImage),
                icon_button(self.make_favorite_icon()).on_press(BottomPaneMessage::ToggleFavorite),
                icon_button(iced_fonts::Nerd::ChevronRight).on_press(BottomPaneMessage::NextImage),
                icon_button(iced_fonts::Nerd::Trash).on_press_maybe(self.photo_id.map(BottomPaneMessage::DeleteImage)),
            ];
        iced::widget::container(row)
            .center_x(iced::Fill)
            .into()
    }

    fn view_right(&self) -> iced::Element<'a, BottomPaneMessage> {
        let row = iced::widget::row![
                icon_button(iced_fonts::Nerd::Grid).on_press(BottomPaneMessage::ToggleDisplayGrid),
                icon_button(iced_fonts::Nerd::TargetVariant).on_press_maybe(self.can_reset_view.then(|| BottomPaneMessage::ResetView)),
                icon_button(self.make_parameters_visibility_icon()).on_press(BottomPaneMessage::ToggleParametersVisibility)
            ];
        iced::widget::container(row)
            .align_right(iced::Fill)
            .into()
    }

    fn make_parameters_visibility_icon(&self) -> iced_fonts::Nerd {
        if self.parameters_visible {
            iced_fonts::Nerd::EyeTwo
        } else {
            iced_fonts::Nerd::EyeOutline
        }
    }

    fn make_favorite_icon(&self) -> iced_fonts::Nerd {
        if self.is_favorite {
            iced_fonts::Nerd::HeartFill
        } else {
            iced_fonts::Nerd::Heart
        }
    }
}