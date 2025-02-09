use crate::{album::{self, RadialMask}, pipeline::viewport, view_mode, Main, Message, MouseMessage, Point};

impl Main {
    pub fn view(&self) -> iced::Element<Message> {
        let view = View::new(
            &self.viewport,
            self.mouse_position,
            self.workspace.current_crop(),
            self.workspace.current_parameters(),
            self.workspace.album_images(),
            &self.view_mode);
        view.view()
    }
}

pub struct View<'a> {
    viewport: &'a viewport::Viewport,
    mouse_position: Point,
    crop: &'a album::Crop,
    parameters: &'a album::Parameters,
    album_images: &'a Vec<album::AlbumImage>,
    view_mode: &'a view_mode::ViewMode,
}

impl<'a> View<'a> {
    pub fn new(
            viewport: &'a viewport::Viewport,
            mouse_position: Point,
            crop: &'a album::Crop,
            parameters: &'a album::Parameters,
            album_images: &'a Vec<album::AlbumImage>,
            view_mode: &'a view_mode::ViewMode) -> Self {
        Self { viewport, mouse_position, crop, parameters, album_images, view_mode }
    }

    pub fn view(&self) -> iced::Element<'a, Message> {
        iced::widget::row![
                self.view_image(),
                self.view_sliders()
            ]
            .width(iced::Fill)
            .height(iced::Fill)
            .into()
    }

    fn view_image(&self) -> iced::Element<'a, Message> {
        iced::widget::column![
                self.view_image_area(),
                self.view_debugger(),
                self.view_thumbnails()
            ]
            .into()
    }

    fn view_image_area(&self) -> iced::Element<'a, Message> {
        let image_area = iced::widget::shader(self.viewport)
            .width(iced::Fill)
            .height(iced::Fill);
        let image_mouse_area = iced::widget::mouse_area(image_area)
            .on_move(|_point| Message::ImageMouseMessage(MouseMessage::Over))
            .on_press(Message::ImageMouseMessage(MouseMessage::Press))
            .on_right_press(Message::ImageMouseMessage(MouseMessage::RightPress))
            .on_release(Message::ImageMouseMessage(MouseMessage::Release));
        image_mouse_area.into()
    }

    fn view_debugger(&self) -> iced::Element<'a, Message> {
        let debug_str: String = format!("{:?}, {:?}", self.mouse_position, self.view_mode);
        iced::widget::container(iced::widget::text(debug_str))
            .style(iced::widget::container::dark)
            .width(iced::Fill)
            .into()
    }

    fn view_thumbnails(&self) -> iced::Element<'a, Message> {
        let thumbnails = self.album_images.iter().enumerate()
            .map(|(index, album_image)| self.view_thumbnail_image(index, &album_image))
            .collect();

        let row = iced::widget::Row::from_vec(thumbnails)
            .spacing(10);

        iced::widget::container(row)
            .padding(10)
            .height(120)
            .into()
    }

    fn view_thumbnail_image(&self, index: usize, album_image: &album::AlbumImage) -> iced::Element<'a, Message> {
        let image_handle = iced::widget::image::Handle::from_rgba(
            album_image.thumbnail.width as u32,
            album_image.thumbnail.height as u32,
            album_image.thumbnail.pixels.clone());
        iced::widget::mouse_area(iced::widget::image(image_handle))
            .on_press(Message::SetImage(index))
            .into()
    }
    
    fn view_sliders(&self) -> iced::Element<'a, Message> {
        let column = iced::widget::column![
                iced::widget::button("Load").on_press(Message::LoadAlbum),
                self.view_main_parameter_sliders(),
                self.view_all_mask_parameter_sliders(),
                self.view_misc_buttons()
            ]
            .spacing(30);
        iced::widget::container(column)
            .padding(10)
            .width(300)
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
                iced::widget::slider(-180.0..=180.0, self.crop.angle_degrees, Message::AngleChanged)
            ]
            .into()
    }
}