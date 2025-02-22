use crate::Point;
use crate::ui::message::{Message, MouseMessage};
use crate::viewport::Viewport;
use crate::view_mode::ViewMode;

pub struct RenderPane<'a> {
    viewport: &'a Viewport,
    mouse_position: &'a Point,
    view_mode: ViewMode
}

fn on_scroll(scroll_delta: iced::mouse::ScrollDelta) -> Message {
    let mouse_message: MouseMessage = match scroll_delta {
        iced::mouse::ScrollDelta::Pixels { x: _, y } => {
            MouseMessage::Scroll(y)
        },
        iced::mouse::ScrollDelta::Lines { x: _, y } => {
            MouseMessage::Scroll(y)
        },
    };
    Message::ImageMouseMessage(mouse_message)
}

impl <'a> RenderPane<'a> {
    pub fn new(
            viewport: &'a Viewport,
            mouse_position: &'a Point,
            view_mode: ViewMode) -> Self {
        Self { viewport, mouse_position, view_mode }
    }

    pub fn view(&self) -> iced::Element<'a, Message> {
        iced::widget::column![
                self.view_viewport(),
                self.view_debugger()
            ]
            .into()
    }

    fn view_viewport(&self) -> iced::Element<'a, Message> {
        let image_area = iced::widget::shader(self.viewport)
            .width(iced::Fill)
            .height(iced::Fill);
        let image_mouse_area = iced::widget::mouse_area(image_area)
            .on_move(|_point| Message::ImageMouseMessage(MouseMessage::Over))
            .on_press(Message::ImageMouseMessage(MouseMessage::Press))
            .on_right_press(Message::ImageMouseMessage(MouseMessage::RightPress))
            .on_release(Message::ImageMouseMessage(MouseMessage::Release))
            .on_scroll(on_scroll);
        image_mouse_area.into()
    }

    fn view_debugger(&self) -> iced::Element<'a, Message> {
        let debug_str: String = format!("{:?}, {:?}", self.mouse_position, self.view_mode);
        iced::widget::container(iced::widget::text(debug_str))
            .style(iced::widget::container::dark)
            .width(iced::Fill)
            .into()
    }
}