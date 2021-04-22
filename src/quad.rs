use std::mem;

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::renderable::Renderable;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
struct Vertex {
    position: [f32; 2],
    tex_coord: [f32; 2],
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-1.0, 1.0],
        tex_coord: [0.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0],
        tex_coord: [0.0, 0.0],
    },
    Vertex {
        position: [1.0, -1.0],
        tex_coord: [1.0, 0.0],
    },
    Vertex {
        position: [1.0, 1.0],
        tex_coord: [1.0, 1.0],
    },
];

const VERTICES_LAYOUT: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
    array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
    step_mode: wgpu::InputStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array![0 => Float2, 1 => Float2],
};

#[rustfmt::skip]
const INDICES: &[u16] = &[
    0, 1, 2,
    2, 3, 0,
];

pub struct Quad {
    pipeline: wgpu::RenderPipeline,
    vert_buffer: wgpu::Buffer,
    idx_buffer: wgpu::Buffer,
    idx_num: u32,
}

impl Quad {
    pub fn new(device: &wgpu::Device, sc_desc: &wgpu::SwapChainDescriptor) -> Quad {
        let quad_vert = device.create_shader_module(&wgpu::include_spirv!("quad.vert.spv"));
        let quad_frag = device.create_shader_module(&wgpu::include_spirv!("board.frag.spv"));

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Quad Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Quad Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &quad_vert,
                entry_point: "main",
                buffers: &[VERTICES_LAYOUT],
            },
            fragment: Some(wgpu::FragmentState {
                module: &quad_frag,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: sc_desc.format,
                    alpha_blend: wgpu::BlendState::REPLACE,
                    color_blend: wgpu::BlendState::REPLACE,
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

        let vert_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsage::VERTEX,
        });

        let idx_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage: wgpu::BufferUsage::INDEX,
        });

        let idx_num = INDICES.len() as u32;

        Quad {
            pipeline,
            vert_buffer,
            idx_buffer,
            idx_num,
        }
    }
}

impl Renderable for Quad {
    fn render(&self, encoder: &mut wgpu::CommandEncoder, frame: &wgpu::SwapChainTexture) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
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
        render_pass.set_vertex_buffer(0, self.vert_buffer.slice(..));
        render_pass.set_index_buffer(self.idx_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.idx_num, 0, 0..1);
    }
}
