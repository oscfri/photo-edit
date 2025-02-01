use cgmath::{self, Matrix, SquareMatrix};
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

fn create_rotation_transform(crop: &Crop, image_width: usize, image_height: usize) -> cgmath::Matrix4<f32> {
    let angle: f32 = crop.degrees_angle / 180.0 * std::f32::consts::PI;
    let cos: f32 = f32::cos(angle);
    let sin: f32 = f32::sin(angle);
    let center_area: Rectangle = Rectangle {
        x: -crop.center_x as f32,
        y: -crop.center_y as f32,
        width: image_width as f32,
        height: image_height as f32
    };
    let center = transform(&Rectangle::default(), &center_area);
    let rotate = cgmath::Matrix4::new(
        cos, -sin, 0.0, 0.0,
        sin, cos, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );
    let revert_center = transform(&center_area, &Rectangle::default());
    center * rotate * revert_center
}

pub fn apply_image_transform(
        point: &iced::Point,
        bounds: &iced::Rectangle,
        crop: &Crop,
        image_width: usize,
        image_height: usize) -> iced::Point {
    let crop_area: Rectangle = create_crop_area(crop);
    let viewport_area: Rectangle = create_viewport_area(bounds, crop);
    let rotation: cgmath::Matrix4<f32> = create_rotation_transform(crop, image_width, image_height);
    let crop_transform = transform(&Rectangle::default(), &crop_area);
    let viewport_transform = transform(&viewport_area, &Rectangle::default());
    let transform = viewport_transform * rotation * crop_transform;
    let transformed_point = transform.transpose() * cgmath::vec4(point.x, point.y, 0.0, 1.0);
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
    let crop_aspect_ratio: f32 = (crop.width as f32) / (crop.height as f32);
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
    let x: f32 = (crop.center_x - crop.width / 2) as f32;
    let y: f32 = (crop.center_y - crop.height / 2) as f32;
    let width: f32 = crop.width as f32;
    let height: f32 = crop.height as f32;
    Rectangle {
        x,
        y,
        width,
        height
    }
}

fn create_crop_relative_area(crop: &Crop, image_width: usize, image_height: usize) -> Rectangle {
    let x: f32 = ((crop.center_x - crop.width / 2) as f32) / (image_width as f32);
    let y: f32 = ((crop.center_y - crop.height / 2) as f32) / (image_height as f32);
    let width: f32 = (crop.width as f32) / (image_width as f32);
    let height: f32 = (crop.height as f32) / (image_height as f32);
    Rectangle {
        x,
        y,
        width,
        height
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
        let crop_area: Rectangle = create_crop_relative_area(view, image_width, image_height);
        let crop_area2: Rectangle = create_crop_relative_area(crop, image_width, image_height);
        let rotation: cgmath::Matrix4<f32> = create_rotation_transform(view, image_width, image_height);
        Self {
            window_to_render: transform(&window_area, &render_area).into(),
            base_to_viewport_window: transform(&Rectangle::default(), &viewport_area).into(),
            base_to_cropped_base: (transform(&Rectangle::default(), &crop_area) * rotation).into(),
            base_to_cropped_base2: (rotation * transform(&crop_area2, &Rectangle::default())).into(),
        }
    }
}