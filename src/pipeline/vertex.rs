use iced::widget::shader::wgpu;

#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vertex {
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

pub fn vertices_square() -> [Vertex; 6] {
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