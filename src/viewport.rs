use glam;
use iced;
use iced::advanced::layout;
use iced::mouse;
use iced::widget::shader;
use iced::widget::shader::wgpu;

use wgpu::util::DeviceExt;

use crate::types::RawImage;

pub struct Viewport {
    // TODO: Probably should put nice things here
    // - Parameters
    // - Image
    pub image: RawImage
}

impl<Message> shader::Program<Message> for Viewport {
    type State = ();
    type Primitive = Primitive;

    fn draw(&self, _state: &Self::State, _cursor: mouse::Cursor, _bounds: iced::Rectangle) -> Self::Primitive {
        Primitive {
            image: self.image.clone()
        }
    }
}

#[derive(Debug)]
pub struct Primitive {
    image: RawImage
}

impl shader::Primitive for Primitive {
    fn prepare(
            &self,
            device: &wgpu::Device,
            queue: &wgpu::Queue,
            format: wgpu::TextureFormat,
            storage: &mut shader::Storage,
            bounds: &iced::Rectangle,
            viewport: &shader::Viewport) {
        if !storage.has::<Pipeline>() {
            storage.store(Pipeline::new(&self, &device, format))
        }

        let pipeline = storage.get_mut::<Pipeline>().unwrap();

        pipeline.update(&self, queue, &bounds, &viewport);
    }

    fn render(
            &self,
            encoder: &mut wgpu::CommandEncoder,
            storage: &shader::Storage,
            target: &wgpu::TextureView,
            clip_bounds: &iced::Rectangle<u32>) {
        let pipeline = storage.get::<Pipeline>().unwrap();
        pipeline.render(
            encoder,
            target,
            *clip_bounds);
    }
}

pub struct Pipeline {
    pipeline: wgpu::RenderPipeline,
    vertices: wgpu::Buffer,
    uniforms: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
    texture_size: wgpu::Extent3d,
    diffuse_texture: wgpu::Texture,
    diffuse_bind_group: wgpu::BindGroup
}

impl Pipeline {
    fn new(
            primitive: &Primitive,
            device: &wgpu::Device,
            format: wgpu::TextureFormat) -> Self {
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex buffer"),
                contents: bytemuck::cast_slice(&vertices()),
                usage: wgpu::BufferUsages::VERTEX,
            });
        
        let texture_size = wgpu::Extent3d {
            width: primitive.image.width as u32,
            height: primitive.image.height as u32,
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

        let uniforms = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("The uniform buffer"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("The uniform bind group layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
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
                    resource: uniforms.as_entire_binding(),
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
                buffers: &[Vertex::desc()],
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
        
        Self {
            pipeline,
            vertices,
            uniforms,
            uniform_bind_group,
            texture_size,
            diffuse_texture,
            diffuse_bind_group
        }
    }

    fn update(
            &self,
            primitive: &Primitive,
            queue: &wgpu::Queue,
            bounds: &iced::Rectangle,
            viewport: &shader::Viewport) {
        queue.write_buffer(&self.uniforms, 0, bytemuck::bytes_of(&Uniforms::new(bounds, viewport)));
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &self.diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &primitive.image.pixels,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * primitive.image.width as u32),
                rows_per_image: Some(primitive.image.height as u32)
            },
            self.texture_size
        );
    }

    fn render(
            &self,
            encoder: &mut wgpu::CommandEncoder,
            target: &wgpu::TextureView,
            viewport: iced::Rectangle<u32>) {
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
        pass.set_scissor_rect(viewport.x, viewport.y, viewport.width, viewport.height);
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        pass.set_bind_group(1, &self.diffuse_bind_group, &[]);
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.draw(0..6, 0..1);
    }
}

#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Uniforms {
    camera_position: glam::Vec2,
    camera_size: glam::Vec2,
}

impl Uniforms {
    pub fn new(bounds: &iced::Rectangle, viewport: &shader::Viewport) -> Self {
        let bottom_y = bounds.y * 2.0 + bounds.height;
        Self {
            camera_position: glam::vec2(
                bounds.x / (viewport.physical_width() as f32),
                1.0 - bottom_y / (viewport.physical_height() as f32)
            ),
            camera_size: glam::vec2(        
                bounds.width / (viewport.physical_width() as f32),
                bounds.height / (viewport.physical_height() as f32)
            )
        }
    }
}

fn vertices() -> [Vertex; 6] {
    [
        Vertex {
            position: glam::vec2(-1.0, -1.0),
            uv: glam::vec2(0.0, 1.0)
        },
        Vertex {
            position: glam::vec2(1.0, -1.0),
            uv: glam::vec2(1.0, 1.0)
        },
        Vertex {
            position: glam::vec2(1.0, 1.0),
            uv: glam::vec2(1.0, 0.0)
        },
        Vertex {
            position: glam::vec2(1.0, 1.0),
            uv: glam::vec2(1.0, 0.0)
        },
        Vertex {
            position: glam::vec2(-1.0, 1.0),
            uv: glam::vec2(0.0, 0.0)
        },
        Vertex {
            position: glam::vec2(-1.0, -1.0),
            uv: glam::vec2(0.0, 1.0)
        },
    ]
}

#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
struct Vertex {
    position: glam::Vec2,
    uv: glam::Vec2
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        //position
        0 => Float32x2,
        //uv
        1 => Float32x2,
    ];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}