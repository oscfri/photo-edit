use glam;
use iced;
use iced::mouse;
use iced::widget::shader;
use iced::widget::shader::wgpu;
use iced::widget::shader::wgpu::util::DeviceExt;

pub struct Viewport {
}

impl<Message> shader::Program<Message> for Viewport {
    type State = ();
    type Primitive = Primitive;

    fn draw(&self, _state: &Self::State, _cursor: mouse::Cursor, bounds: iced::Rectangle) -> Self::Primitive {
        Primitive{}
    }
}

#[derive(Debug)]
pub struct Primitive {}

impl shader::Primitive for Primitive {
    fn prepare(
            &self,
            device: &wgpu::Device,
            queue: &wgpu::Queue,
            format: wgpu::TextureFormat,
            storage: &mut shader::Storage,
            _bounds: &iced::Rectangle,
            viewport: &shader::Viewport) {
        if !storage.has::<Pipeline>() {
            storage.store(Pipeline::new(&device, format))
        }
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
}

impl Pipeline {
    fn new(
            device: &wgpu::Device,
            format: wgpu::TextureFormat) -> Pipeline {
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: bytemuck::cast_slice(&[
                Vertex {
                    pos: glam::vec3(-0.5, -0.5, -1.0),
                    uv: glam::vec2(0.0, 0.0)
                },
                Vertex {
                    pos: glam::vec3(0.5, -0.5, -1.0),
                    uv: glam::vec2(1.0, 0.0)
                },
                Vertex {
                    pos: glam::vec3(0.5, 0.5, -1.0),
                    uv: glam::vec2(1.0, 1.0)
                },
                Vertex {
                    pos: glam::vec3(-0.5, 0.5, -1.0),
                    uv: glam::vec2(0.0, 1.0)
                },
            ]),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("The shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(
                include_str!("shaders/image.wgsl"),
            )),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("The pipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()]
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
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Max,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });
        
        Self {
            pipeline,
            vertices
        }
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
        pass.set_scissor_rect(
            viewport.x,
            viewport.y,
            viewport.width,
            viewport.height,
        );
        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.draw(0..4, 0..1);
    }
}

#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub pos: glam::Vec3,
    pub uv: glam::Vec2
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] = wgpu::vertex_attr_array![
        //position
        0 => Float32x3,
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