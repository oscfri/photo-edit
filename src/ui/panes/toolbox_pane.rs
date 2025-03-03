use crate::{ui::message::{MainParameterMessage, MaskChangeMessage, MaskMessage, MiscMessage, ToolboxMessage}, workspace::parameters::{Parameters, RadialMask}};

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

    pub fn view(&self) -> iced::Element<'a, ToolboxMessage> {
        let column = iced::widget::column![
                self.view_main_parameter_sliders().map(ToolboxMessage::MainParameterMessage),
                self.view_all_mask_parameter_sliders().map(ToolboxMessage::MaskMessage),
                self.view_misc_buttons().map(ToolboxMessage::MiscMessage)
            ]
            .spacing(30);
        iced::widget::container(column)
            .padding(10)
            .width(300) // TODO: It should be responsibility of Window to know this
            .height(iced::Fill)
            .style(iced::widget::container::bordered_box)
            .into()
    }

    fn view_main_parameter_sliders(&self) -> iced::Element<'a, MainParameterMessage> {
        iced::widget::column![
                iced::widget::text("Brightness"),
                iced::widget::slider(-100.0..=100.0, self.parameters.brightness, MainParameterMessage::BrightnessChanged),
                iced::widget::text("Contrast"),
                iced::widget::slider(-100.0..=100.0, self.parameters.contrast, MainParameterMessage::ContrastChanged),
                iced::widget::text("Tint"),
                iced::widget::slider(-100.0..=100.0, self.parameters.tint,  MainParameterMessage::TintChanged),
                iced::widget::text("Temperature"),
                iced::widget::slider(-100.0..=100.0, self.parameters.temperature, MainParameterMessage::TemperatureChanged),
                iced::widget::text("Saturation"),
                iced::widget::slider(-100.0..=100.0, self.parameters.saturation, MainParameterMessage::SaturationChanged)
            ]
            .into()
    }

    fn view_all_mask_parameter_sliders(&self) -> iced::Element<'a, MaskMessage> {
        let mask_sliders = self.parameters.radial_masks.iter()
            .enumerate()
            .map(|(mask_index, radial_mask)| {
                self.view_mask_parameter_sliders(radial_mask)
                    .map(move |message| MaskMessage::MaskChanged(mask_index, message))
            });
        
        let mask_elements = iced::widget::Column::with_children(mask_sliders)
            .spacing(10);

        iced::widget::column![
                mask_elements,
                iced::widget::button("Add mask").on_press(MaskMessage::AddMask),
            ]
            .spacing(10)
            .into()
    }

    fn view_mask_parameter_sliders(&self, radial_mask: &RadialMask) -> iced::Element<'a, MaskChangeMessage> {
        let buttons = iced::widget::row![
                iced::widget::button("Edit").on_press(MaskChangeMessage::ToggleMaskMode),
                iced::widget::button("Delete").on_press(MaskChangeMessage::DeleteMask),
            ]
            .spacing(10);
        iced::widget::column![
                iced::widget::checkbox("Linear", radial_mask.is_linear)
                    .on_toggle(MaskChangeMessage::MaskToggleLinear),
                iced::widget::text("Brightness"),
                iced::widget::slider(
                    -100.0..=100.0,
                    radial_mask.brightness,
                    MaskChangeMessage::BrightnessChanged),
                iced::widget::text("Angle"),
                iced::widget::slider(
                    -180.0..=180.0,
                    radial_mask.angle,
                    MaskChangeMessage::MaskAngleChanged),
                buttons,
            ]
            .into()
    }

    fn view_misc_buttons(&self) -> iced::Element<'a, MiscMessage> {
        iced::widget::column![
                iced::widget::button("Next").on_press(MiscMessage::NextImage),
                iced::widget::button("Crop").on_press(MiscMessage::ToggleCropMode),
                iced::widget::button("Save").on_press(MiscMessage::SaveAlbum),
                iced::widget::button("Load").on_press(MiscMessage::LoadAlbum),
                iced::widget::button("Export").on_press(MiscMessage::ExportImage),
                iced::widget::button("Delete").on_press(MiscMessage::DeleteImage),
                iced::widget::text("Angle"),
                iced::widget::slider(-180.0..=180.0, self.angle_degrees, MiscMessage::AngleChanged)
            ]
            .into()
    }
}