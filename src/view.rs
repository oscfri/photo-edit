use crate::{album, pipeline::viewport, Main, Message, MouseMessage, Point};

impl Main {
    pub fn view(&self) -> iced::Element<Message> {
        let view = View::new(
            &self.viewport,
            self.mouse_position,
            self.workspace.current_crop(),
            self.workspace.current_parameters(),
            self.workspace.album_images());
        view.view()
    }
}

pub struct View<'a> {
    viewport: &'a viewport::Viewport,
    mouse_position: Point,
    crop: &'a album::Crop,
    parameters: &'a album::Parameters,
    album_images: &'a Vec<album::AlbumImage>
}

impl<'a> View<'a> {
    pub fn new(
            viewport: &'a viewport::Viewport,
            mouse_position: Point,
            crop: &'a album::Crop,
            parameters: &'a album::Parameters,
            album_images: &'a Vec<album::AlbumImage>) -> Self {
        Self { viewport, mouse_position, crop, parameters, album_images }
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
        let debug_str: String = format!("{:?}, {:?}", self.mouse_position, self.crop);
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
                iced::widget::text("Brightness"),
                iced::widget::slider(-100.0..=100.0, self.parameters.brightness, Message::BrightnessChanged),
                iced::widget::text("Contrast"),
                iced::widget::slider(-100.0..=100.0, self.parameters.contrast, Message::ContrastChanged),
                iced::widget::text("Tint"),
                iced::widget::slider(-100.0..=100.0, self.parameters.tint, Message::TintChanged),
                iced::widget::text("Temperature"),
                iced::widget::slider(-100.0..=100.0, self.parameters.temperature, Message::TemperatureChanged),
                iced::widget::text("Saturation"),
                iced::widget::slider(-100.0..=100.0, self.parameters.saturation, Message::SaturationChanged),
                iced::widget::text("Mask Brightness"),
                iced::widget::slider(-100.0..=100.0, self.parameters.radial_masks[0].brightness, |brightness| Message::MaskBrightnessChanged(0, brightness)),
                iced::widget::button("Next").on_press(Message::NextImage),
                iced::widget::button("Crop").on_press(Message::ToggleCropMode),
                iced::widget::button("Mask").on_press(Message::ToggleMaskMode(0)),
                iced::widget::text("Angle"),
                iced::widget::slider(-180.0..=180.0, self.crop.angle_degrees, Message::AngleChanged),
            ];
        iced::widget::container(column)
            .padding(10)
            .width(300)
            .height(iced::Fill)
            .style(iced::widget::container::bordered_box)
            .into()
    }
}