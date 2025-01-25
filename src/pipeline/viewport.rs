use crate::album::Crop;
use crate::album::Parameters;
use crate::types::RawImage;
use crate::view_mode::ViewMode;
use crate::pipeline::pipeline;
use crate::pipeline::vertex;
use crate::pipeline::camera_uniform;

use iced::mouse;
use iced::widget::shader;
use iced::widget::shader::wgpu;
use wgpu::util::DeviceExt;

use super::crop_uniform;
use super::parameter_uniform;

// Hack to access viewport size. It doesn't seem like we can access the viewport size directly (at least not according
// to any documentation I've found). We need to know the viewport size so we can convert mouse coordinates from "window"
// space to "image" space.
static mut VIEWPORT_WIDTH: f32 = 0.0;
static mut VIEWPORT_HEIGHT: f32 = 0.0;

pub fn get_viewport_width() -> f32 {
    unsafe {
        VIEWPORT_WIDTH
    }
}

pub fn get_viewport_height() -> f32 {
    unsafe {
        VIEWPORT_HEIGHT
    }
}

fn update_viewport(bounds: &iced::Rectangle) {
    unsafe {
        VIEWPORT_WIDTH = bounds.width;
        VIEWPORT_HEIGHT = bounds.height;
    }
}

#[derive(Debug, Clone)]
pub struct ViewportWorkspace {
    image: RawImage,
    image_index: usize,
    parameters: Parameters,
    crop: Crop
}

impl ViewportWorkspace {
    pub fn new(
            image: RawImage,
            image_index: usize,
            parameters: Parameters,
            crop: Crop) -> Self {
        Self { image, image_index, parameters, crop }
    }
}

#[derive(Debug, Clone)]
pub struct Viewport {
    workspace: ViewportWorkspace,
    view_mode: ViewMode,
}

impl Viewport {
    pub fn new(workspace: ViewportWorkspace, view_mode: ViewMode) -> Self {
        Self { workspace, view_mode }
    }
}

struct ImageIndex {
    index: usize
}

impl<Message> shader::Program<Message> for Viewport {
    type State = ();
    type Primitive = Self;

    fn draw(&self, _state: &Self::State, _cursor: mouse::Cursor, _bounds: iced::Rectangle) -> Self::Primitive {
        self.clone()
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

        update_viewport(&*bounds);

        let pipeline = storage.get_mut::<pipeline::Pipeline>().unwrap();

        let camera_uniform = camera_uniform::CameraUniform::new(&bounds, &viewport);
        let parameter_uniform = parameter_uniform::ParameterUniform::new(&self.workspace.parameters);
        let crop_uniform = crop_uniform::CropUniform::new(
                &self.workspace.crop,
                &self.view_mode,
                self.workspace.image.width,
                self.workspace.image.height);

        pipeline.update(queue, &self.workspace.image, &camera_uniform, &parameter_uniform, &crop_uniform);
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
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex buffer"),
                contents: bytemuck::cast_slice(&vertex::vertices_square()),
                usage: wgpu::BufferUsages::VERTEX,
            });
        
        let texture_size = wgpu::Extent3d {
            width: self.workspace.image.width as u32,
            height: self.workspace.image.height as u32,
            depth_or_array_layers: 1
        };
        let diffuse_texture = device.create_texture(
            &wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("Image texture"),
                view_formats: &[],
            }
        );

        let diffuse_texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let camera_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Camera Uniform Buffer"),
            size: std::mem::size_of::<camera_uniform::CameraUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let parameter_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Parameter Uniform Buffer"),
            size: std::mem::size_of::<parameter_uniform::ParameterUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let crop_uniform_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Crop Uniform Buffer"),
            size: 32, // Not sure why below is not working
            // size: std::mem::size_of::<crop_uniform::CropUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("The uniform bind group layout"),
            entries: &[
                // Camera
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Parameters
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // Crop
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("The uniform bind group"),
            layout: &uniform_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: parameter_uniform_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: crop_uniform_buffer.as_entire_binding(),
                },
            ],
        });

        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type:  wgpu::TextureSampleType::Float {
                            filterable: true
                        },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                }
            ],
            label: Some("Texture Bind Group Layout")
        });
        let diffuse_bind_group = device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                    label: Some("Diffuse Bind Group"),
                    layout: &texture_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                        },
                    ]
                }
        );

        let shader = device.create_shader_module(wgpu::include_wgsl!("shaders/image.wgsl"));

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("The pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[vertex::Vertex::desc()],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });
        
        pipeline::Pipeline::new(
            pipeline,
            vertices,
            camera_uniform_buffer,
            parameter_uniform_buffer,
            crop_uniform_buffer,
            uniform_bind_group,
            diffuse_texture,
            diffuse_bind_group
        )
    }
}