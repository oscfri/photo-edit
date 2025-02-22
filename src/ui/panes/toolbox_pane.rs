use crate::ui::message::Message;
use crate::album::{RadialMask, Parameters};

pub struct ToolboxPane<'a> {
    parameters: &'a Parameters,
    angle_degrees: f32,
}

impl <'a> ToolboxPane<'a> {
    pub fn new(
            parameters: &'a Parameters,
            angle_degrees: f32) -> Self {
        Self { parameters, angle_degrees }
    }

    pub fn view(&self) -> iced::Element<'a, Message> {
        let column = iced::widget::column![
                iced::widget::button("Load").on_press(Message::LoadAlbum),
                self.view_main_parameter_sliders(),
                self.view_all_mask_parameter_sliders(),
                self.view_misc_buttons()
            ]
            .spacing(30);
        iced::widget::container(column)
            .padding(10)
            .width(300) // TODO: It should be responsibility of Window to know this
            .height(iced::Fill)
            .style(iced::widget::container::bordered_box)
            .into()
    }

    fn view_main_parameter_sliders(&self) -> iced::Element<'a, Message> {
        iced::widget::column![
                iced::widget::text("Brightness"),
                iced::widget::slider(-100.0..=100.0, self.parameters.brightness, Message::BrightnessChanged),
                iced::widget::text("Contrast"),
                iced::widget::slider(-100.0..=100.0, self.parameters.contrast, Message::ContrastChanged),
                iced::widget::text("Tint"),
                iced::widget::slider(-100.0..=100.0, self.parameters.tint, Message::TintChanged),
                iced::widget::text("Temperature"),
                iced::widget::slider(-100.0..=100.0, self.parameters.temperature, Message::TemperatureChanged),
                iced::widget::text("Saturation"),
                iced::widget::slider(-100.0..=100.0, self.parameters.saturation, Message::SaturationChanged)
            ]
            .into()
    }

    fn view_all_mask_parameter_sliders(&self) -> iced::Element<'a, Message> {
        let mask_sliders = self.parameters.radial_masks.iter()
            .enumerate()
            .map(|(mask_index, radial_mask)| self.view_mask_parameter_sliders(radial_mask, mask_index));
        
        let mask_elements = iced::widget::Column::with_children(mask_sliders)
            .spacing(10);

        iced::widget::column![
                mask_elements,
                iced::widget::button("Add mask").on_press(Message::AddMask),
            ]
            .spacing(10)
            .into()
    }

    fn view_mask_parameter_sliders(&self, radial_mask: &RadialMask, mask_index: usize) -> iced::Element<'a, Message> {
        let buttons = iced::widget::row![
                iced::widget::button("Edit").on_press(Message::ToggleMaskMode(mask_index)),
                iced::widget::button("Delete").on_press(Message::DeleteMask(mask_index)),
            ]
            .spacing(10);
        iced::widget::column![
                iced::widget::text("Brightness"),
                iced::widget::slider(-100.0..=100.0, radial_mask.brightness, move |brightness| Message::MaskBrightnessChanged(mask_index, brightness)),
                buttons,
            ]
            .into()
    }

    fn view_misc_buttons(&self) -> iced::Element<'a, Message> {
        iced::widget::column![
                iced::widget::button("Next").on_press(Message::NextImage),
                iced::widget::button("Crop").on_press(Message::ToggleCropMode),
                iced::widget::text("Angle"),
                iced::widget::slider(-180.0..=180.0, self.angle_degrees, Message::AngleChanged)
            ]
            .into()
    }
}