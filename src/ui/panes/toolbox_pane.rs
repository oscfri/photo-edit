use crate::{ui::{message::{MainParameterMessage, MaskChangeMessage, MaskMessage, MiscMessage, ToolboxMessage}, utils::icon_button}, workspace::parameters::{Parameters, RadialMask}};

pub struct ToolboxPane {
    parameters: Parameters,
    angle_degrees: f32,
    mask_edit_index: Option<usize>
}

impl <'a> ToolboxPane {
    pub fn new(
            parameters: Parameters,
            angle_degrees: f32,
            mask_edit_index: Option<usize>) -> Self {
        Self { parameters, angle_degrees, mask_edit_index }
    }

    pub fn view(&self) -> iced::Element<'a, ToolboxMessage> {
        let column = iced::widget::column![
                self.view_main_parameter_sliders().map(ToolboxMessage::MainParameterMessage),
                self.view_all_mask_parameter_sliders().map(ToolboxMessage::MaskMessage),
                self.view_misc_buttons().map(ToolboxMessage::MiscMessage)
            ]
            .spacing(30);
        let container = iced::widget::container(column)
            .padding(10);
        let scrollable = iced::widget::scrollable(container)
            .width(iced::Fill)
            .height(iced::Fill);
        iced::widget::container(scrollable)
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
                self.view_mask_parameter_sliders(mask_index, radial_mask)
                    .map(move |message| MaskMessage::MaskChanged(mask_index, message))
            });
        
        let mask_elements = iced::widget::Column::with_children(mask_sliders)
            .spacing(10);

        iced::widget::column![
                iced::widget::text("Mask"),
                mask_elements,
                icon_button(iced_fonts::Bootstrap::PlusCircle).on_press(MaskMessage::AddMask),
            ]
            .into()
    }

    fn view_mask_parameter_sliders(&self, mask_index: usize, radial_mask: &RadialMask) -> iced::Element<'a, MaskChangeMessage> {
        let buttons = iced::widget::row![
                icon_button(self.mask_edit_icon(mask_index)).on_press(MaskChangeMessage::ToggleMaskMode),
                icon_button(iced_fonts::Bootstrap::Trashthree).on_press(MaskChangeMessage::DeleteMask),
                iced::widget::checkbox("Linear", radial_mask.is_linear)
                    .on_toggle(MaskChangeMessage::MaskToggleLinear),
            ];
        iced::widget::column![
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
                iced::widget::button("Crop").on_press(MiscMessage::ToggleCropMode),
                iced::widget::button("Save").on_press(MiscMessage::SaveAlbum),
                iced::widget::text("Angle"),
                iced::widget::slider(-180.0..=180.0, self.angle_degrees, MiscMessage::AngleChanged)
            ]
            .into()
    }

    fn mask_edit_icon(&self, mask_index: usize) -> iced_fonts::Bootstrap {
        if self.mask_edit_index == Some(mask_index) {
            iced_fonts::Bootstrap::PencilFill
        } else {
            iced_fonts::Bootstrap::Pencil
        }
    }
}