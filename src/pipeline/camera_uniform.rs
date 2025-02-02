use cgmath::{self, Matrix};
use iced::widget::shader;

use crate::album::Crop;

// It's important we're working with 4x4 matrixes. Otherwise we'll run into annoying memory alignment issues.
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct CameraUniform {
    window_to_render: [[f32; 4]; 4],
    base_to_viewport_window: [[f32; 4]; 4],
    base_to_cropped_base: [[f32; 4]; 4],
    base_to_cropped_base2: [[f32; 4]; 4],
}

#[derive(Debug)]
struct Rectangle {
    center_x: f32,
    center_y: f32,
    width: f32,
    height: f32,
    angle_degrees: f32
}

impl Default for Rectangle {
    fn default() -> Self {
        Self { center_x: 0.0, center_y: 0.0, width: 1.0, height: 1.0, angle_degrees: 0.0 }
    }
}

fn translate_transform(x: f32, y: f32) -> cgmath::Matrix4<f32> {
    cgmath::Matrix4::new(
        1.0, 0.0, 0.0, x,
        0.0, 1.0, 0.0, y,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn scale_transform(x: f32, y: f32) -> cgmath::Matrix4<f32> {
    cgmath::Matrix4::new(
        x, 0.0, 0.0, 0.0,
        0.0, y, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn rotate_transform(angle_degrees: f32) -> cgmath::Matrix4<f32> {
    let angle_radians: f32 = angle_degrees / 180.0 * std::f32::consts::PI;
    let cos: f32 = f32::cos(angle_radians);
    let sin: f32 = f32::sin(angle_radians);
    cgmath::Matrix4::new(
        cos, -sin, 0.0, 0.0,
        sin, cos, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn transform(from: &Rectangle, to: &Rectangle) -> cgmath::Matrix4<f32> {
    let from_center: cgmath::Matrix4<f32> = translate_transform(-from.center_x, -from.center_y);
    let from_scale: cgmath::Matrix4<f32> = scale_transform(1.0 / from.width, 1.0 / from.height);
    let from_rotate: cgmath::Matrix4<f32> = rotate_transform(-from.angle_degrees);
    let to_rotate: cgmath::Matrix4<f32> = rotate_transform(to.angle_degrees);
    let to_scale: cgmath::Matrix4<f32> = scale_transform(to.width, to.height);
    let to_center: cgmath::Matrix4<f32> = translate_transform(to.center_x, to.center_y);
    from_center * from_rotate * from_scale * to_scale * to_rotate * to_center
}

pub fn apply_image_transform(
        point: &iced::Point,
        bounds: &iced::Rectangle,
        crop: &Crop) -> iced::Point {
    let crop_area: Rectangle = create_crop_area(crop);
    let viewport_area: Rectangle = create_viewport_area(bounds, crop);
    let crop_transform = transform(&Rectangle::default(), &crop_area);
    let viewport_transform = transform(&viewport_area, &Rectangle::default());
    let transform = viewport_transform * crop_transform;
    let transformed_point = transform.transpose() * cgmath::vec4(point.x, point.y, 0.0, 1.0);
    iced::Point {
        x: transformed_point.x / transformed_point.w,
        y: transformed_point.y / transformed_point.w,
    }
}

fn create_render_area() -> Rectangle {
    Rectangle {
        center_x: 0.0,
        center_y: 0.0,
        width: 2.0,
        height: -2.0,
        angle_degrees: 0.0
    }
}

fn create_window_area(viewport: &shader::Viewport) -> Rectangle {
    Rectangle {
        center_x: (viewport.physical_width() as f32) / 2.0,
        center_y: (viewport.physical_height() as f32) / 2.0,
        width: viewport.physical_width() as f32,
        height: viewport.physical_height() as f32,
        angle_degrees: 0.0
    }
}

fn create_viewport_area(bounds: &iced::Rectangle, crop: &Crop) -> Rectangle {
    let bounds_center_x: f32 = bounds.x + bounds.width / 2.0;
    let bounds_center_y: f32 = bounds.y + bounds.height / 2.0;
    let crop_aspect_ratio: f32 = (crop.width as f32) / (crop.height as f32);
    let bounds_aspect_ratio: f32 = bounds.width / bounds.height;
    let width: f32 = bounds.width * (crop_aspect_ratio / bounds_aspect_ratio).min(1.0);
    let height: f32 = bounds.height * (bounds_aspect_ratio / crop_aspect_ratio).min(1.0);
    Rectangle {
        center_x: bounds_center_x,
        center_y: bounds_center_y,
        width,
        height,
        angle_degrees: 0.0
    }
}

fn create_crop_area(crop: &Crop) -> Rectangle {
    Rectangle {
        center_x: crop.center_x as f32,
        center_y: crop.center_y as f32,
        width: crop.width as f32,
        height: crop.height as f32,
        angle_degrees: crop.angle_degrees
    }
}

fn create_crop_relative_area(crop: &Crop, image_width: usize, image_height: usize) -> Rectangle {
    let center_x: f32 = (crop.center_x as f32) / (image_width as f32);
    let center_y: f32 = (crop.center_y as f32) / (image_height as f32);
    let width: f32 = (crop.width as f32) / (image_width as f32);
    let height: f32 = (crop.height as f32) / (image_height as f32);
    Rectangle {
        center_x,
        center_y,
        width,
        height,
        angle_degrees: crop.angle_degrees
    }
}

impl CameraUniform {
    pub fn new(
            bounds: &iced::Rectangle,
            viewport: &shader::Viewport,
            view: &Crop,
            crop: &Crop,
            image_width: usize,
            image_height: usize) -> Self {
        let render_area: Rectangle = create_render_area();
        let window_area: Rectangle = create_window_area(viewport);
        let viewport_area: Rectangle = create_viewport_area(bounds, view);
        let view_area: Rectangle = create_crop_relative_area(view, image_width, image_height);
        let crop_area: Rectangle = create_crop_relative_area(crop, image_width, image_height);
        Self {
            window_to_render: transform(&window_area, &render_area).into(),
            base_to_viewport_window: transform(&Rectangle::default(), &viewport_area).into(),
            base_to_cropped_base: transform(&Rectangle::default(), &view_area).into(),
            base_to_cropped_base2: transform(&crop_area, &view_area).into(),
        }
    }
}