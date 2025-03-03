use crate::ui::message::{MouseMessage, RenderMessage};
use crate::viewport::Viewport;
use crate::view_mode::ViewMode;

pub struct RenderPane<'a> {
    viewport: &'a Viewport,
    view_mode: ViewMode
}

fn on_scroll(scroll_delta: iced::mouse::ScrollDelta) -> MouseMessage {
    match scroll_delta {
        iced::mouse::ScrollDelta::Pixels { x: _, y } => {
            MouseMessage::Scroll(y)
        },
        iced::mouse::ScrollDelta::Lines { x: _, y } => {
            MouseMessage::Scroll(y)
        },
    }
}

impl <'a> RenderPane<'a> {
    pub fn new(
            viewport: &'a Viewport,
            view_mode: ViewMode) -> Self {
        Self { viewport, view_mode }
    }

    pub fn view(&self) -> iced::Element<'a, RenderMessage> {
        iced::widget::column![
                self.view_viewport().map(RenderMessage::MouseMessage),
                self.view_debugger()
            ]
            .into()
    }

    fn view_viewport(&self) -> iced::Element<'a, MouseMessage> {
        let image_area = iced::widget::shader(self.viewport)
            .width(iced::Fill)
            .height(iced::Fill);
        let image_mouse_area = iced::widget::mouse_area(image_area)
            .on_move(|_point| MouseMessage::Over)
            .on_press(MouseMessage::Press)
            .on_right_press(MouseMessage::RightPress)
            .on_release(MouseMessage::Release)
            .on_scroll(on_scroll);
        image_mouse_area.into()
    }

    fn view_debugger(&self) -> iced::Element<'a, RenderMessage> {
        let debug_str: String = format!("{:?}", self.view_mode);
        iced::widget::container(iced::widget::text(debug_str))
            .style(iced::widget::container::dark)
            .width(iced::Fill)
            .into()
    }
}