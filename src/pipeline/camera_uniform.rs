use cgmath::{self, Matrix};

use crate::{pipeline::transform::{transform, Rectangle}, workspace::parameters::Crop};

use super::viewport::ViewportWorkspace;

// It's important we're working with 4x4 matrixes. Otherwise we'll run into annoying memory alignment issues.
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct CameraUniform {
    window_to_render: [[f32; 4]; 4],
    base_to_viewport_window: [[f32; 4]; 4],
    base_to_cropped_base: [[f32; 4]; 4],
    view_to_crop: [[f32; 4]; 4],
    base_to_image_area: [[f32; 4]; 4],
    base_to_export_area: [[f32; 4]; 4],
}

pub fn point_to_image_position(
        point: &iced::Point,
        bounds: &Rectangle,
        crop: &Crop) -> iced::Point {
    let crop_area: Rectangle = create_crop_area(crop);
    let viewport_area: Rectangle = create_viewport_area(bounds, crop);

    let view_to_crop = transform(&viewport_area, &crop_area);

    let transform = view_to_crop;
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

fn create_viewport_area(bounds: &Rectangle, crop: &Crop) -> Rectangle {
    let crop_aspect_ratio: f32 = (crop.width as f32) / (crop.height as f32);
    let bounds_aspect_ratio: f32 = bounds.width / bounds.height;
    let width: f32 = bounds.width * (crop_aspect_ratio / bounds_aspect_ratio).min(1.0);
    let height: f32 = bounds.height * (bounds_aspect_ratio / crop_aspect_ratio).min(1.0);
    Rectangle {
        center_x: bounds.center_x,
        center_y: bounds.center_y,
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
    let image_width_f32: f32 = image_width as f32;
    let image_height_f32: f32 = image_height as f32;
    // Need to offset the center point to accomodate for the aspect ratio conversion
    // (not exactly sure why this is needed, or why this works)
    let max_side: f32 = (image_width_f32).max(image_height_f32);
    let offset_x: f32 = (image_height_f32 - image_width_f32).max(0.0) / 2.0;
    let offset_y: f32 = (image_width_f32 - image_height_f32).max(0.0) / 2.0;
    let center_x: f32 = (crop.center_x as f32 + offset_x) / max_side;
    let center_y: f32 = (crop.center_y as f32 + offset_y) / max_side;
    let width: f32 = (crop.width as f32) / image_width_f32;
    let height: f32 = (crop.height as f32) / image_height_f32;
    Rectangle {
        center_x,
        center_y,
        width,
        height,
        angle_degrees: crop.angle_degrees
    }
}

fn create_crop_image_area(crop: &Crop) -> Rectangle {
    let center_x: f32 = crop.center_x as f32;
    let center_y: f32 = crop.center_y as f32;
    let width: f32 = crop.width as f32;
    let height: f32 = crop.height as f32;
    Rectangle {
        center_x,
        center_y,
        width,
        height,
        angle_degrees: crop.angle_degrees
    }
}

fn create_aspect_area(image_width: usize, image_height: usize) -> Rectangle {
    let image_width_f32: f32 = image_width as f32;
    let image_height_f32: f32 = image_height as f32;
    let width: f32 = (image_width_f32 / image_height_f32).min(1.0);
    let height: f32 = (image_height_f32 / image_width_f32).min(1.0);
    Rectangle {
        center_x: 0.5,
        center_y: 0.5,
        width,
        height,
        angle_degrees: 0.0
    }
}

fn create_uv_area() -> Rectangle {
    Rectangle {
        center_x: 0.5,
        center_y: 0.5,
        width: 1.0,
        height: 1.0,
        angle_degrees: 0.0
    }
}

fn create_export_area(crop: &Crop) -> Rectangle {
    Rectangle {
        center_x: (crop.width as f32) / 2.0,
        center_y: (crop.height as f32) / 2.0,
        width: crop.width as f32,
        height: crop.height as f32,
        angle_degrees: 0.0
    }
}

impl CameraUniform {
    pub fn new(
            bounds: &Rectangle,
            window_area: &Rectangle,
            workspace: &ViewportWorkspace) -> Self {
                
        let view = &workspace.view;
        let crop = &workspace.crop;
        let image_width = workspace.image.width;
        let image_height = workspace.image.height;
                
        let render_area: Rectangle = create_render_area();
        let viewport_area: Rectangle = create_viewport_area(bounds, view);
        let view_area: Rectangle = create_crop_relative_area(view, image_width, image_height);
        let crop_area: Rectangle = create_crop_relative_area(crop, image_width, image_height);
        let image_area: Rectangle = create_crop_image_area(view);
        let aspect_area: Rectangle = create_aspect_area(image_width, image_height);
        let export_area: Rectangle = create_export_area(crop);
        let uv_area: Rectangle = create_uv_area();

        let base_to_aspect = transform(&uv_area, &aspect_area);
        let aspect_to_base = transform(&aspect_area, &uv_area);
        let uv_to_view = transform(&uv_area, &view_area);
        let crop_to_uv = transform(&crop_area, &uv_area);

        Self {
            window_to_render: transform(&window_area, &render_area).into(),
            base_to_viewport_window: transform(&uv_area, &viewport_area).into(),
            base_to_cropped_base: (base_to_aspect * uv_to_view * aspect_to_base).into(),
            view_to_crop: (base_to_aspect * crop_to_uv * aspect_to_base).into(),
            base_to_image_area: transform(&uv_area, &image_area).into(),
            base_to_export_area: transform(&uv_area, &export_area).into(),
        }
    }
}