use crate::{ui::{message::{MainParameterMessage, MaskChangeMessage, MaskMessage, MiscMessage, ToolboxMessage}, utils::{icon_button, slider_scaled}}, workspace::parameters::{CropPreset, Parameters, RadialMask}};

pub struct ToolboxPane {
    parameters: Parameters,
    angle_degrees: f32,
    crop_scale: f32,
    mask_edit_index: Option<usize>,
    enabled: bool
}

impl <'a> ToolboxPane {
    pub fn new(
            parameters: Parameters,
            angle_degrees: f32,
            crop_scale: f32,
            mask_edit_index: Option<usize>,
            enabled: bool) -> Self {
        Self { parameters, angle_degrees, crop_scale, mask_edit_index, enabled }
    }

    pub fn view(&self) -> iced::Element<'a, ToolboxMessage> {
        let contents = self.view_contents();

        iced::widget::container(contents)
            .style(iced::widget::container::bordered_box)
            .into()
    }

    fn view_contents(&self) -> iced::Element<'a, ToolboxMessage> {
        if self.enabled {
            let column = iced::widget::column![
                    self.view_base_parameter_sliders().map(ToolboxMessage::MainParameterMessage),
                    iced::widget::horizontal_rule(2),
                    self.view_all_mask_parameter_sliders().map(ToolboxMessage::MaskMessage),
                    iced::widget::horizontal_rule(2),
                    self.view_misc_buttons().map(ToolboxMessage::MiscMessage)
                ]
                .spacing(15);
            let contents = iced::widget::container(column)
                .padding(15);

            iced::widget::scrollable(contents)
                .width(iced::Fill)
                .height(iced::Fill)
                .into()
        } else {
            iced::widget::container(iced::widget::text("..."))
                .center(iced::Fill)
                .width(iced::Fill)
                .height(iced::Fill)
                .into()
        }
    }

    fn view_base_parameter_sliders(&self) -> iced::Element<'a, MainParameterMessage> {
        let base_parameters = &self.parameters.base_parameters;

        let main_group = iced::widget::column![
                self.view_slider("Exposure", base_parameters.exposure, MainParameterMessage::ExposureChanged),
                self.view_slider("Contrast", base_parameters.contrast, MainParameterMessage::ContrastChanged),
            ];
        let tones_group = iced::widget::column![
                self.view_slider("Shadows", base_parameters.shadows, MainParameterMessage::ShadowsChanged),
                self.view_slider("Midtones", base_parameters.midtones, MainParameterMessage::MidtonesChanged),
                self.view_slider("Highlights", base_parameters.highlights, MainParameterMessage::HighlightsChanged),
            ];
        let colors_group = iced::widget::column![
                self.view_slider("Tint", base_parameters.tint, MainParameterMessage::TintChanged),
                self.view_slider("Temperature", base_parameters.temperature, MainParameterMessage::TemperatureChanged),
                self.view_slider("Saturation", base_parameters.saturation, MainParameterMessage::SaturationChanged),
            ];

        iced::widget::column![
                main_group,
                tones_group,
                colors_group,
            ]
            .spacing(15)
            .into()
    }

    fn view_slider<T: Clone + 'a>(&self, label: &'a str, value: f32, message: impl Fn(f32) -> T + 'a) -> iced::Element<'a, T> {
        let label_text = iced::widget::container(iced::widget::text(label))
            .align_left(iced::Fill);
        let value_text = iced::widget::container(iced::widget::text(format!("{:.1}", value)))
            .align_right(iced::Fill);
        let label_row = iced::widget::row![
                label_text,
                value_text
            ];
        let slider = iced::widget::slider(-100.0..=100.0, value, message);
        iced::widget::column![
                label_row,
                slider
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
                icon_button(iced_fonts::Nerd::PlusCircle).on_press(MaskMessage::AddMask),
            ]
            .into()
    }

    fn view_mask_parameter_sliders(&self, mask_index: usize, radial_mask: &RadialMask) -> iced::Element<'a, MaskChangeMessage> {
        let buttons = iced::widget::row![
                icon_button(self.mask_edit_icon(mask_index)).on_press(MaskChangeMessage::ToggleMaskMode),
                icon_button(iced_fonts::Nerd::Trash).on_press(MaskChangeMessage::DeleteMask),
                iced::widget::checkbox("Linear", radial_mask.is_linear)
                    .on_toggle(MaskChangeMessage::MaskToggleLinear),
            ];
        iced::widget::column![
                self.view_slider("Brightness", radial_mask.brightness, MaskChangeMessage::BrightnessChanged),
                iced::widget::text("Angle"),
                iced::widget::slider(
                    -180.0..=180.0,
                    radial_mask.angle_degrees,
                    MaskChangeMessage::MaskAngleChanged),
                iced::widget::text("Feather"),
                    iced::widget::slider(
                        -100.0..=100.0,
                        radial_mask.feather,
                        MaskChangeMessage::FeatherChanged),
                buttons,
            ]
            .into()
    }

    fn view_misc_buttons(&self) -> iced::Element<'a, MiscMessage> {
        iced::widget::column![
                iced::widget::text("Crop"),
                self.view_crop_buttons(),
                iced::widget::text("Angle"),
                slider_scaled(-3600.0..=3600.0, self.angle_degrees, 40.0, MiscMessage::AngleChanged),
                iced::widget::text("Scale"),
                slider_scaled(-500.0..=0.0, self.crop_scale, 100.0, MiscMessage::CropScaleChanged)
            ]
            .into()
    }

    fn view_crop_buttons(&self) -> iced::Element<'a, MiscMessage> {
        let crop_presets = [
            CropPreset::Original,
            CropPreset::Ratio(1, 1),
            CropPreset::Ratio(5, 4),
            CropPreset::Ratio(4, 3),
            CropPreset::Ratio(3, 2),
            CropPreset::Ratio(16, 9),
            CropPreset::Ratio(4, 5),
            CropPreset::Ratio(3, 4),
            CropPreset::Ratio(2, 3),
            CropPreset::Ratio(9, 16),
        ];

        let crop_preset = self.parameters.crop.as_ref().map(|crop| crop.preset);
        iced::widget::row![
                icon_button(self.crop_icon()).on_press(MiscMessage::ToggleCropMode),
                icon_button(iced_fonts::Nerd::RotateLeftVariant).on_press(MiscMessage::CropRotateLeft),
                icon_button(iced_fonts::Nerd::RotateRightVariant).on_press(MiscMessage::CropRotateRight),
                iced::widget::pick_list(crop_presets, crop_preset, MiscMessage::CropPresetChanged),
            ]
            .into()
    }

    fn mask_edit_icon(&self, mask_index: usize) -> iced_fonts::Nerd {
        if self.mask_edit_index == Some(mask_index) {
            iced_fonts::Nerd::PencilTwo
        } else {
            iced_fonts::Nerd::PencilOutline
        }
    }

    fn crop_icon(&self) -> iced_fonts::Nerd {
        iced_fonts::Nerd::CropOne
    }
}