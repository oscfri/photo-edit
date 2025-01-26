use cgmath::{self, Matrix};
use iced::widget::shader;

use crate::album::Crop;

// It's important we're working with 4x4 matrixes. Otherwise we'll run into annoying memory alignment issues.
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct CameraUniform {
    render_transform: [[f32; 4]; 4], // window coordinates -> render coordinates
    window_transform: [[f32; 4]; 4], // crop coordinates -> window coordinates
    crop_transform: [[f32; 4]; 4], // uv coordinates -> crop coordinates
    image_transform: [[f32; 4]; 4], // uv coordinates -> image coordinates
}

#[derive(Debug)]
struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32
}

impl Default for Rectangle {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, width: 1.0, height: 1.0 }
    }
}

fn transform(from: &Rectangle, to: &Rectangle) -> cgmath::Matrix4<f32> {
    let translate_x: f32 = to.x - from.x / from.width * to.width;
    let translate_y: f32 = to.y - from.y / from.height * to.height;
    let scale_x: f32 = to.width / from.width;
    let scale_y: f32 = to.height / from.height;
    cgmath::Matrix4::new(
        scale_x, 0.0, 0.0, translate_x,
        0.0, scale_y, 0.0, translate_y,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

pub fn apply_image_transform(point: &iced::Point, bounds: &iced::Rectangle, crop: &Crop) -> iced::Point {
    let from: Rectangle = create_viewport_area(bounds, crop);
    let to: Rectangle = create_crop_area(crop);
    let transform = transform(&from, &to).transpose();
    let transformed_point = transform * cgmath::vec4(point.x, point.y, 0.0, 1.0);
    iced::Point {
        x: transformed_point.x / transformed_point.w,
        y: transformed_point.y / transformed_point.w,
    }
}

fn create_render_area() -> Rectangle {
    Rectangle {
        x: -1.0,
        y: 1.0,
        width: 2.0,
        height: -2.0,
    }
}

fn create_window_area(viewport: &shader::Viewport) -> Rectangle {
    Rectangle {
        x: 0.0,
        y: 0.0,
        width: viewport.physical_width() as f32,
        height: viewport.physical_height() as f32,
    }
}

fn create_viewport_area(bounds: &iced::Rectangle, crop: &Crop) -> Rectangle {
    let crop_width: f32 = (crop.x2 - crop.x1).abs() as f32;
    let crop_height: f32 = (crop.y2 - crop.y1).abs() as f32;
    let crop_aspect_ratio: f32 = crop_width / crop_height;
    let bounds_aspect_ratio: f32 = bounds.width / bounds.height;
    let width: f32 = bounds.width * (crop_aspect_ratio / bounds_aspect_ratio).min(1.0);
    let height: f32 = bounds.height * (bounds_aspect_ratio / crop_aspect_ratio).min(1.0);
    let offset_x: f32 = (bounds.width - width) / 2.0;
    let offset_y: f32 = (bounds.height - height) / 2.0;
    Rectangle {
        x: bounds.x + offset_x,
        y: bounds.y + offset_y,
        width,
        height
    }
}

fn create_crop_area(crop: &Crop) -> Rectangle {
    let crop_width: f32 = (crop.x2 - crop.x1).abs() as f32;
    let crop_height: f32 = (crop.y2 - crop.y1).abs() as f32;
    Rectangle {
        x: 0.0,
        y: 0.0,
        width: crop_width,
        height: crop_height
    }
}

fn create_image_area(image_width: usize, image_height: usize) -> Rectangle {
    Rectangle {
        x: 0.0,
        y: 0.0,
        width: image_width as f32,
        height: image_height as f32
    }
}

impl CameraUniform {
    pub fn new(
            bounds: &iced::Rectangle,
            viewport: &shader::Viewport,
            view: &Crop,
            image_width: usize,
            image_height: usize) -> Self {
        let render_area: Rectangle = create_render_area();
        let window_area: Rectangle = create_window_area(viewport);
        let viewport_area: Rectangle = create_viewport_area(bounds, view);
        let crop_area: Rectangle = create_crop_area(view);
        let image_area: Rectangle = create_image_area(image_width, image_height);
        Self {
            render_transform: transform(&window_area, &render_area).into(),
            window_transform: transform(&crop_area, &viewport_area).into(),
            crop_transform: transform(&Rectangle::default(), &crop_area).into(),
            image_transform: transform( &Rectangle::default(), &image_area).into(),
        }
    }
}