use std::sync::Arc;

use crate::types::RawImage;
use crate::pipeline::pipeline;
use crate::pipeline::camera_uniform;
use crate::workspace::parameters::Crop;
use crate::workspace::parameters::CropPreset;
use crate::workspace::parameters::{Parameters, RadialMask};
use crate::workspace::workspace::Workspace;

use iced::mouse;
use iced::widget::shader;
use iced::widget::shader::wgpu;

use super::pipeline_factory::PipelineFactory;
use super::transform::Rectangle;

// Hack to access viewport size. It doesn't seem like we can access the viewport size directly (at least not according
// to any documentation I've found). We need to know the viewport size so we can convert mouse coordinates from "window"
// space to "image" space.
static mut IMAGE_MOUSE_X: i32 = 0;
static mut IMAGE_MOUSE_Y: i32 = 0;
static mut RELATIVE_MOUSE_X: i32 = 0;
static mut RELATIVE_MOUSE_Y: i32 = 0;

pub fn get_image_mouse_x() -> i32 {
    unsafe {
        IMAGE_MOUSE_X
    }
}

pub fn get_image_mouse_y() -> i32 {
    unsafe {
        IMAGE_MOUSE_Y
    }
}

pub fn get_relative_mouse_x() -> i32 {
    unsafe {
        RELATIVE_MOUSE_X
    }
}

pub fn get_relative_mouse_y() -> i32 {
    unsafe {
        RELATIVE_MOUSE_Y
    }
}

fn update_image_mouse(mouse_x: i32, mouse_y: i32) {
    unsafe {
        IMAGE_MOUSE_X = mouse_x;
        IMAGE_MOUSE_Y = mouse_y;
    }
}

fn update_relative_mouse(mouse_x: i32, mouse_y: i32) {
    unsafe {
        RELATIVE_MOUSE_X = mouse_x;
        RELATIVE_MOUSE_Y = mouse_y;
    }
}

#[derive(Default, Debug, Clone)]
pub struct ViewportCrop {
    pub center_x: i32,
    pub center_y: i32,
    pub width: i32,
    pub height: i32,
    pub angle_degrees: f32
}

fn width_height_from_crop(crop: &Crop) -> (f32, f32) {
    match crop.preset {
        CropPreset::Original => {
            if crop.rotation % 2 == 0 {
                (crop.source_image_width as f32, crop.source_image_height as f32)
            } else {
                (crop.source_image_height as f32, crop.source_image_width as f32)
            }
        },
        CropPreset::Ratio(width, height) => {
            let crop_width = crop.source_image_width as f32;
            let crop_height = crop_width * (height as f32) / (width as f32);
            (crop_width, crop_height)
        }
    }
}

impl From<Crop> for ViewportCrop {
    fn from(crop: Crop) -> Self {
        let scale = f32::powf(2.0, crop.scale);
        let (width, height) = width_height_from_crop(&crop);

        Self {
            center_x: crop.center_x,
            center_y: crop.center_y,
            width: (width * scale) as i32,
            height: (height * scale) as i32,
            angle_degrees: crop.get_full_angle()
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct ViewportParameters {
    pub exposure: f32,
    pub contrast: f32,
    pub shadows: f32,
    pub midtones: f32,
    pub highlights: f32,
    pub tint: f32,
    pub temperature: f32,
    pub saturation: f32,
    pub radial_masks: Vec<RadialMask>,
    pub crop: ViewportCrop
}

impl From<Parameters> for ViewportParameters {
    fn from(parameters: Parameters) -> ViewportParameters {
        let crop = match parameters.crop {
            Some(crop) => crop.into(),
            _ => ViewportCrop::default()
        };
        let base_parameters = &parameters.base_parameters;
        ViewportParameters {
            exposure: base_parameters.exposure,
            contrast: base_parameters.contrast,
            shadows: base_parameters.shadows,
            midtones: base_parameters.midtones,
            highlights: base_parameters.highlights,
            tint: base_parameters.tint,
            temperature: base_parameters.temperature,
            saturation: base_parameters.saturation,
            radial_masks: parameters.radial_masks.clone(),
            crop: crop
        }
    }
}

#[derive(Debug, Clone)]
pub struct ViewportWorkspace {
    pub image: Arc<RawImage>,
    pub photo_id: i32,
    pub parameters: ViewportParameters,
    pub view: ViewportCrop,
    pub display_grid: bool
}

impl ViewportWorkspace {
    pub fn try_new(workspace: &Workspace) -> Option<Self> {
        if let Some(image) = workspace.current_source_image() {
            let photo_id = workspace.get_photo_id();
            let parameters = workspace.parameters_to_display();
            let view = workspace.current_view();
            let display_grid = workspace.is_crop_mode();
            Some(Self {
                image,
                photo_id,
                parameters,
                view,
                display_grid
            })
        } else {
            None
        }
    }

    pub fn get_image_width(&self) -> usize {
        self.image.width
    }

    pub fn get_image_height(&self) -> usize {
        self.image.height
    }
}

#[derive(Debug, Clone)]
pub struct Viewport {
    workspace: ViewportWorkspace,
    cursor: mouse::Cursor
}

impl Viewport {
    pub fn try_new(workspace: &Workspace) -> Option<Self> {
        if let Some(viewport_workspace) = ViewportWorkspace::try_new(workspace) {
            let cursor: mouse::Cursor = mouse::Cursor::Unavailable;
            Some(Self {
                workspace: viewport_workspace,
                cursor
            })
        } else {
            None
        }
    }

    fn update_mouse(&self, bounds: &iced::Rectangle) {
        match self.cursor {
            mouse::Cursor::Available(point) => {
                let bounds_rectangle = Self::bounds_to_rectangle(bounds);
                let image_point: iced::Point = camera_uniform::point_to_image_position(
                    &point,
                    &bounds_rectangle,
                    &self.workspace.view);
                update_image_mouse(image_point.x as i32, image_point.y as i32);
                let relative_point: iced::Point = camera_uniform::point_to_image_position(
                    &point,
                    &bounds_rectangle,
                    &ViewportCrop {
                        center_x: 0,
                        center_y: 0,
                        ..self.workspace.view.clone()
                    });
                update_relative_mouse(relative_point.x as i32, relative_point.y as i32);
            },
            mouse::Cursor::Unavailable => {} // Do nothing
        }
    }

    fn needs_update(&self, storage: &shader::Storage) -> bool {
        if storage.has::<ImageIndex>() {
            let image_index: &ImageIndex = storage.get::<ImageIndex>().unwrap();
            image_index.photo_id != self.workspace.photo_id
        } else {
            !storage.has::<pipeline::Pipeline>()
        }
    }

    fn create_pipeline(&self, device: &wgpu::Device, format: wgpu::TextureFormat) -> pipeline::Pipeline {
        let image_width = self.workspace.get_image_width();
        let image_height = self.workspace.get_image_height();
        PipelineFactory::new(image_width, image_height, device, format).create()
    }

    fn bounds_to_rectangle(bounds: &iced::Rectangle) -> Rectangle {
        let center_x: f32 = bounds.x + bounds.width / 2.0;
        let center_y: f32 = bounds.y + bounds.height / 2.0;
        Rectangle {
            center_x,
            center_y,
            width: bounds.width,
            height: bounds.height,
            angle_degrees: 0.0
        }
    }

    fn viewport_to_rectangle(viewport: &shader::Viewport) -> Rectangle {
        Rectangle {
            center_x: viewport.logical_size().width / 2.0,
            center_y: viewport.logical_size().height / 2.0,
            width: viewport.logical_size().width,
            height: viewport.logical_size().height,
            angle_degrees: 0.0
        }
    }
}

struct ImageIndex {
    photo_id: i32
}

impl<Message> shader::Program<Message> for Viewport {
    type State = ();
    type Primitive = Self;

    fn draw(&self, _state: &Self::State, cursor: mouse::Cursor, _bounds: iced::Rectangle) -> Self::Primitive {
        let mut cloned: Viewport = self.clone();
        cloned.cursor = cursor;
        cloned
    }

    // TODO: There's a `mouse_interaction` that could potentially be used
}

impl shader::Primitive for Viewport {
    fn prepare(
            &self,
            device: &wgpu::Device,
            queue: &wgpu::Queue,
            format: wgpu::TextureFormat,
            storage: &mut shader::Storage,
            bounds: &iced::Rectangle,
            viewport: &shader::Viewport) {
        
        let needs_update: bool = self.needs_update(&storage);

        if needs_update {
            storage.store(self.create_pipeline(device, format));
            storage.store(ImageIndex { photo_id: self.workspace.photo_id });
        }

        self.update_mouse(&bounds);

        let pipeline = storage.get_mut::<pipeline::Pipeline>().unwrap();

        let bounds_rectangle = Self::bounds_to_rectangle(bounds);
        let viewport_rectangle = Self::viewport_to_rectangle(viewport);

        pipeline.update(queue, &self.workspace, &bounds_rectangle, &viewport_rectangle, viewport.scale_factor() as f32);
    }

    fn render(
            &self,
            encoder: &mut wgpu::CommandEncoder,
            storage: &shader::Storage,
            target: &wgpu::TextureView,
            clip_bounds: &iced::Rectangle<u32>) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("viewport"),
            color_attachments: &[Some(
                wgpu::RenderPassColorAttachment {
                    view: target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    }
                }
            )],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None
        });
        pass.set_scissor_rect(clip_bounds.x, clip_bounds.y, clip_bounds.width, clip_bounds.height);

        let pipeline = storage.get::<pipeline::Pipeline>().unwrap();
        pipeline.render_pass(&mut pass);
    }
}