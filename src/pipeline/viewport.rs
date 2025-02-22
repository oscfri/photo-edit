use crate::album::Crop;
use crate::album::Parameters;
use crate::types::RawImage;
use crate::view_mode::ViewMode;
use crate::pipeline::pipeline;
use crate::pipeline::camera_uniform;
use crate::workspace::Workspace;

use iced::mouse;
use iced::widget::shader;
use iced::widget::shader::wgpu;

use super::crop_uniform;
use super::parameter_uniform;
use super::pipeline_factory::PipelineFactory;
use super::radial_parameter;

// Hack to access viewport size. It doesn't seem like we can access the viewport size directly (at least not according
// to any documentation I've found). We need to know the viewport size so we can convert mouse coordinates from "window"
// space to "image" space.
static mut IMAGE_MOUSE_X: i32 = 0;
static mut IMAGE_MOUSE_Y: i32 = 0;

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

fn update_image_mouse(mouse_x: i32, mouse_y: i32) {
    unsafe {
        IMAGE_MOUSE_X = mouse_x;
        IMAGE_MOUSE_Y = mouse_y;
    }
}

#[derive(Debug, Clone)]
pub struct ViewportWorkspace {
    image: RawImage,
    image_index: usize,
    parameters: Parameters,
    crop: Crop,
    view: Crop
}

impl ViewportWorkspace {
    pub fn new(
            image: RawImage,
            image_index: usize,
            parameters: Parameters,
            crop: Crop,
            view: Crop) -> Self {
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
    pub fn new(workspace: ViewportWorkspace, view_mode: ViewMode) -> Self {
        let cursor: mouse::Cursor = mouse::Cursor::Unavailable;
        Self { workspace, view_mode, cursor }
    }

    pub fn from_workspace(workspace: &Workspace) -> Self {
        let view: Crop = workspace.current_view();
        let viewport_workspace = ViewportWorkspace::new(
            workspace.current_source_image().clone(),
            workspace.get_image_index(),
            workspace.current_parameters().clone(),
            workspace.current_crop().clone(),
            view);
        let view_mode: ViewMode = workspace.get_view_mode();
        let cursor: mouse::Cursor = mouse::Cursor::Unavailable;
        Self {
            workspace: viewport_workspace,
            view_mode,
            cursor
        }
    }

    fn update_mouse(&self, bounds: &iced::Rectangle) {
        match self.cursor {
            mouse::Cursor::Available(point) => {
                let image_point: iced::Point = camera_uniform::point_to_image_position(
                    &point,
                    bounds,
                    &self.workspace.view);
                update_image_mouse(image_point.x as i32, image_point.y as i32);
            },
            mouse::Cursor::Unavailable => {} // Do nothing
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

        if self.needs_update(&storage) {
            storage.store(self.create_pipeline(device, format));
            storage.store(ImageIndex { index: self.workspace.image_index });
        }

        self.update_mouse(&bounds);

        let pipeline = storage.get_mut::<pipeline::Pipeline>().unwrap();

        let camera_uniform = camera_uniform::CameraUniform::new(
                &bounds,
                &viewport,
                &self.workspace.view,
                &self.workspace.crop,
                self.workspace.image.width,
                self.workspace.image.height);
        let parameter_uniform = parameter_uniform::ParameterUniform::new(&self.workspace.parameters);
        let crop_uniform = crop_uniform::CropUniform::new(&self.view_mode);
        let radial_parameters = radial_parameter::RadialParameters::new(&self.workspace.parameters);

        pipeline.update(queue, &self.workspace.image, &camera_uniform, &parameter_uniform, &crop_uniform, &radial_parameters);
    }

    fn render(
            &self,
            encoder: &mut wgpu::CommandEncoder,
            storage: &shader::Storage,
            target: &wgpu::TextureView,
            clip_bounds: &iced::Rectangle<u32>) {
        let pipeline = storage.get::<pipeline::Pipeline>().unwrap();
        pipeline.render(
            encoder,
            target,
            *clip_bounds);
    }
}

impl Viewport {
    fn needs_update(&self, storage: &shader::Storage) -> bool {
        if storage.has::<ImageIndex>() {
            let image_index: &ImageIndex = storage.get::<ImageIndex>().unwrap();
            image_index.index != self.workspace.image_index
        } else {
            !storage.has::<pipeline::Pipeline>()
        }
    }

    fn create_pipeline(&self, device: &wgpu::Device, format: wgpu::TextureFormat) -> pipeline::Pipeline {
        PipelineFactory::new(&self.workspace, device, format).create()
    }
}