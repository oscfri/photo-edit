use iced::widget::shader::wgpu;
use iced::widget::shader::wgpu::RenderPipeline;
use wgpu::util::DeviceExt;

use std::mem::size_of;

use crate::pipeline::pipeline;
use crate::pipeline::vertex;
use crate::pipeline::viewport;

use super::camera_uniform::CameraUniform;
use super::crop_uniform::CropUniform;
use super::parameter_uniform::ParameterUniform;
use super::radial_parameter::RadialParameters;

pub struct PipelineFactory<'a> {
    workspace: &'a viewport::ViewportWorkspace,
    device: &'a wgpu::Device,
    format: wgpu::TextureFormat
}

impl<'a> PipelineFactory<'a> {
    pub fn new(
            workspace: &'a viewport::ViewportWorkspace,
            device: &'a wgpu::Device,
            format: wgpu::TextureFormat) -> Self {
        Self { workspace, device, format }
    }

    pub fn create(&self) -> pipeline::Pipeline {
        let vertex_buffer = self.create_vertex_buffer("vertex_buffer");

        let camera_buffer = self.create_uniform_buffer(size_of::<CameraUniform>(), "camera_buffer");
        let parameter_buffer = self.create_uniform_buffer(size_of::<ParameterUniform>(), "parameter_buffer");
        let crop_buffer = self.create_uniform_buffer(size_of::<CropUniform>(), "crop_buffer");
        let radial_parameters_buffer = self.create_uniform_buffer(size_of::<RadialParameters>(), "radial_parameters_buffer");
        let output_texture_buffer = self.create_storage_buffer(4 * 256 * 256 as usize, "output_texture_buffer");

        let buffers = &[
            &camera_buffer,
            &parameter_buffer,
            &crop_buffer,
            &radial_parameters_buffer
        ];
        let uniform_bind_group_layout = self.create_bind_group_layout(4, "uniform_bind_group_layout");
        let uniform_bind_group = self.create_bind_group(&uniform_bind_group_layout, buffers, "uniform_bind_group");

        let diffuse_texture = self.create_image_texture("diffuse_texture"); 
        let output_texture = self.create_storage_texture("output_texture"); 
        let texture_bind_group_layout = self.create_texture_bind_group_layout("texture_bind_group_layout");
        let diffuse_bind_group = self.create_diffuse_bind_group(&diffuse_texture, &output_texture, &texture_bind_group_layout, "diffuse_bind_group");

        let pipeline = self.create_render_pipeline(&uniform_bind_group_layout, &texture_bind_group_layout);
        
        pipeline::Pipeline::new(
            pipeline,
            vertex_buffer,
            camera_buffer,
            parameter_buffer,
            crop_buffer,
            radial_parameters_buffer,
            uniform_bind_group,
            diffuse_texture,
            diffuse_bind_group,
            output_texture,
            output_texture_buffer
        )
    }

    fn create_vertex_buffer(&self, label: &str) -> wgpu::Buffer {
        self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(label),
            contents: bytemuck::cast_slice(&vertex::vertices_square()),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    fn create_image_texture(&self, label: &str) -> wgpu::Texture {
        self.device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some(label),
                size: wgpu::Extent3d {
                    width: self.workspace.get_image_width() as u32,
                    height: self.workspace.get_image_height() as u32,
                    depth_or_array_layers: 1
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            }
        )
    }

    fn create_storage_texture(&self, label: &str) -> wgpu::Texture {
        // TODO: This should correspond to the crop size
        self.device.create_texture(
            &wgpu::TextureDescriptor {
                label: Some(label),
                size: wgpu::Extent3d {
                    width: 256,
                    height: 256,
                    depth_or_array_layers: 1
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8Unorm,
                usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::COPY_SRC,
                view_formats: &[],
            }
        )
    }

    fn create_uniform_buffer(&self, size: usize, label: &str) -> wgpu::Buffer {
        self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: size as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn create_storage_buffer(&self, size: usize, label: &str) -> wgpu::Buffer {
        self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: size as u64,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn create_bind_group_layout(&self, count: usize, label: &str) -> wgpu::BindGroupLayout {
        let entries: Vec<wgpu::BindGroupLayoutEntry> = (0..count)
            .map(|index| {
                wgpu::BindGroupLayoutEntry {
                    binding: index as u32,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            })
            .collect();

        self.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(label),
            entries: &entries,
        })
    }

    fn create_bind_group(
            &self,
            uniform_bind_group_layout: &wgpu::BindGroupLayout,
            buffers: &[&wgpu::Buffer],
            label: &str) -> wgpu::BindGroup {
        let entries: Vec<wgpu::BindGroupEntry> = buffers.iter()
            .enumerate()
            .map(|(index, buffer)| {
                wgpu::BindGroupEntry {
                    binding: index as u32,
                    resource: buffer.as_entire_binding(),
                }
            })
            .collect();

        self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(label),
            layout: &uniform_bind_group_layout,
            entries: &entries,
        })
    }

    fn create_texture_bind_group_layout(&self, label: &str) -> wgpu::BindGroupLayout {
        self.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some(label),
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
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT, // TODO: Should this be in COMPUTE shader?
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2
                    },
                    count: None,
                }
            ],
        })
    }

    fn create_diffuse_bind_group(
            &self,
            diffuse_texture: &wgpu::Texture,
            output_texture: &wgpu::Texture,
            texture_bind_group_layout: &wgpu::BindGroupLayout,
            label: &str) -> wgpu::BindGroup {
        let diffuse_texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let diffuse_sampler = self.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let output_texture_view = output_texture.create_view(&wgpu::TextureViewDescriptor::default());

        self.device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                    label: Some(label),
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
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: wgpu::BindingResource::TextureView(&output_texture_view)
                        }
                    ]
                }
        )
    }

    fn create_render_pipeline(
            &self,
            uniform_bind_group_layout: &wgpu::BindGroupLayout,
            texture_bind_group_layout: &wgpu::BindGroupLayout) -> RenderPipeline {
        let shader = self.device.create_shader_module(wgpu::include_wgsl!("shaders/image.wgsl"));

        let render_pipeline_layout = self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("render_pipeline_layout"),
            bind_group_layouts: &[&uniform_bind_group_layout, &texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("pipeline"),
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
                    format: self.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        })
    }
}