use crate::types::RawImage;
use crate::view_mode::ViewMode;
use crate::pipeline::pipeline;
use crate::pipeline::camera_uniform;
use crate::workspace::parameters::Crop;
use crate::workspace::parameters::Parameters;
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

#[derive(Debug, Clone)]
pub struct ViewportWorkspace {
    pub image: RawImage,
    pub image_index: usize,
    pub parameters: Parameters,
    pub crop: Crop,
    pub view: Crop
}

impl ViewportWorkspace {
    pub fn new(workspace: &Workspace) -> Self {
        let image = workspace.current_source_image().clone();
        let image_index = workspace.get_image_index();
        let parameters = workspace.current_parameters().clone();
        let crop = workspace.current_crop().clone();
        let view = workspace.current_view();
        Self { image, image_index, parameters, crop, view }
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
    view_mode: ViewMode,
    cursor: mouse::Cursor
}

impl Viewport {
    pub fn new(workspace: &Workspace) -> Self {
        let viewport_workspace = ViewportWorkspace::new(workspace);
        let view_mode: ViewMode = workspace.get_view_mode();
        let cursor: mouse::Cursor = mouse::Cursor::Unavailable;
        Self {
            workspace: viewport_workspace,
            view_mode,
            cursor
        }
    }

    fn update_mouse(&self, bounds: &iced::Rectangle) {
        let bounds_rectangle = Self::bounds_to_rectangle(bounds);
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
                    &Crop {
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
            image_index.index != self.workspace.image_index
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
    index: usize
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
            storage.store(ImageIndex { index: self.workspace.image_index });
        }

        self.update_mouse(&bounds);

        let pipeline = storage.get_mut::<pipeline::Pipeline>().unwrap();

        let bounds_rectangle = Self::bounds_to_rectangle(bounds);
        let viewport_rectangle = Self::viewport_to_rectangle(viewport);

        pipeline.update(queue, &self.workspace, &self.view_mode, &bounds_rectangle, &viewport_rectangle);
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