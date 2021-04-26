use std::ops::Range;

use image::GenericImageView;
use wgpu::util::DeviceExt;

use crate::{
    piece::PieceRaw,
    quad::{INDICES, LAYOUT, VERTICES},
    renderable::Renderable,
    Board,
};

type PiecesBuffer = [PieceRaw; 64];

pub struct PiecesView {
    pipeline: wgpu::RenderPipeline,
    vert_buffer: wgpu::Buffer,
    idx_buffer: wgpu::Buffer,
    piece_buffer: wgpu::Buffer,
    indices: Range<u32>,
    texture_bind_group: wgpu::BindGroup,
}

impl PiecesView {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        sc_desc: &wgpu::SwapChainDescriptor,
    ) -> PiecesView {
        let quad_vert =
            device.create_shader_module(&wgpu::include_spirv!("shaders/piece.vert.spv"));
        let quad_frag =
            device.create_shader_module(&wgpu::include_spirv!("shaders/piece.frag.spv"));

        let vert_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Piece Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsage::VERTEX,
        });

        let idx_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Piece Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsage::INDEX,
        });

        let indices = 0..INDICES.len() as u32;

        let piece_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Piece Pieces Buffer"),
            size: std::mem::size_of::<PiecesBuffer>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });

        let mut texture_image =
            image::load_from_memory(include_bytes!("images/pieces.png")).unwrap();
        image::imageops::flip_vertical_in_place(&mut texture_image);
        let texture_rgba = texture_image.as_rgba8().unwrap();
        let dimensions = texture_image.dimensions();

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth: 1,
        };

        let texture_texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            label: Some("Pieces Diffuse Texture"),
        });

        queue.write_texture(
            wgpu::TextureCopyView {
                texture: &texture_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            texture_rgba,
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * dimensions.0,
                rows_per_image: dimensions.1,
            },
            texture_size,
        );

        let texture_texture_view =
            texture_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            comparison: false,
                            filtering: true,
                        },
                        count: None,
                    },
                ],
                label: Some("Pieces Texture Bind Group Layout"),
            });

        let texture_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&texture_sampler),
                },
            ],
            label: Some("Pieces Texture Bind Group"),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pieces Render Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Pieces Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &quad_vert,
                entry_point: "main",
                buffers: &[LAYOUT, PieceRaw::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &quad_frag,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: sc_desc.format,
                    alpha_blend: wgpu::BlendState {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    color_blend: wgpu::BlendState {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                polygon_mode: wgpu::PolygonMode::Fill,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        PiecesView {
            pipeline,
            vert_buffer,
            idx_buffer,
            piece_buffer,
            indices,
            texture_bind_group,
        }
    }
}

impl Renderable for PiecesView {
    fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        queue: &mut wgpu::Queue,
        frame: &wgpu::SwapChainTexture,
        board: &Board,
    ) {
        let mut pieces = Vec::new();
        for (y, row) in board.iter().rev().enumerate() {
            for (x, piece) in row.iter().enumerate() {
                if let Some(piece) = piece {
                    pieces.push(piece.to_raw(x, y));
                }
            }
        }
        queue.write_buffer(&self.piece_buffer, 0, bytemuck::cast_slice(&pieces));
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Pieces Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &frame.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.texture_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vert_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.piece_buffer.slice(..));
        render_pass.set_index_buffer(self.idx_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(self.indices.clone(), 0, 0..pieces.len() as u32);
    }
}
