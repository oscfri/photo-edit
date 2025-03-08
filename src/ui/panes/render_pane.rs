use crate::ui::message::{MouseMessage, RenderMessage};
use crate::viewport::Viewport;

pub struct RenderPane<'a> {
    viewport: &'a Option<Viewport>
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
    pub fn new(viewport: &'a Option<Viewport>) -> Self {
        Self { viewport }
    }

    pub fn view(&self) -> iced::Element<'a, RenderMessage> {
        self.view_viewport().map(RenderMessage::MouseMessage).into()
    }

    fn view_viewport(&self) -> iced::Element<'a, MouseMessage> {
        if let Some(viewport) = self.viewport {
            let image_area = iced::widget::shader(viewport)
                .width(iced::Fill)
                .height(iced::Fill);
            let image_mouse_area = iced::widget::mouse_area(image_area)
                .on_move(|_point| MouseMessage::Over)
                .on_press(MouseMessage::Press)
                .on_right_press(MouseMessage::RightPress)
                .on_release(MouseMessage::Release)
                .on_scroll(on_scroll);
            image_mouse_area.into()
        } else {
            iced::widget::text("Please wait...")
                .center()
                .width(iced::Fill)
                .height(iced::Fill)
                .into()
        }
    }
}