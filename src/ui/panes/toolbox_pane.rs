use crate::{ui::message::{AlbumMessage, Message, WorkspaceMessage}, workspace::parameters::{Parameters, RadialMask}};

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
                iced::widget::button("Load").on_press(AlbumMessage::LoadAlbum.into()),
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
                iced::widget::slider(-100.0..=100.0, self.parameters.brightness, |x| WorkspaceMessage::BrightnessChanged(x).into()),
                iced::widget::text("Contrast"),
                iced::widget::slider(-100.0..=100.0, self.parameters.contrast, |x| WorkspaceMessage::ContrastChanged(x).into()),
                iced::widget::text("Tint"),
                iced::widget::slider(-100.0..=100.0, self.parameters.tint, |x| WorkspaceMessage::TintChanged(x).into()),
                iced::widget::text("Temperature"),
                iced::widget::slider(-100.0..=100.0, self.parameters.temperature, |x| WorkspaceMessage::TemperatureChanged(x).into()),
                iced::widget::text("Saturation"),
                iced::widget::slider(-100.0..=100.0, self.parameters.saturation, |x| WorkspaceMessage::SaturationChanged(x).into())
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
                iced::widget::button("Add mask").on_press(WorkspaceMessage::AddMask.into()),
            ]
            .spacing(10)
            .into()
    }

    fn view_mask_parameter_sliders(&self, radial_mask: &RadialMask, mask_index: usize) -> iced::Element<'a, Message> {
        let buttons = iced::widget::row![
                iced::widget::button("Edit").on_press(WorkspaceMessage::ToggleMaskMode(mask_index).into()),
                iced::widget::button("Delete").on_press(WorkspaceMessage::DeleteMask(mask_index).into()),
            ]
            .spacing(10);
        iced::widget::column![
                iced::widget::checkbox("Linear", radial_mask.is_linear)
                    .on_toggle(move |is_checked| WorkspaceMessage::MaskToggleLinear(mask_index, is_checked).into()),
                iced::widget::text("Brightness"),
                iced::widget::slider(
                    -100.0..=100.0,
                    radial_mask.brightness,
                    move |brightness| WorkspaceMessage::MaskBrightnessChanged(mask_index, brightness).into()),
                iced::widget::text("Angle"),
                iced::widget::slider(
                    -180.0..=180.0,
                    radial_mask.angle,
                    move |angle| WorkspaceMessage::MaskAngleChanged(mask_index, angle).into()),
                buttons,
            ]
            .into()
    }

    fn view_misc_buttons(&self) -> iced::Element<'a, Message> {
        iced::widget::column![
                iced::widget::button("Next").on_press(AlbumMessage::NextImage.into()),
                iced::widget::button("Crop").on_press(WorkspaceMessage::ToggleCropMode.into()),
                iced::widget::button("Save").on_press(AlbumMessage::SaveAlbum.into()),
                iced::widget::button("Export").on_press(WorkspaceMessage::ExportImage.into()),
                iced::widget::button("Delete").on_press(AlbumMessage::DeleteImage.into()),
                iced::widget::text("Angle"),
                iced::widget::slider(-180.0..=180.0, self.angle_degrees, |x| WorkspaceMessage::AngleChanged(x).into())
            ]
            .into()
    }
}