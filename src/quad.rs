use std::mem;

use wgpu::util::DeviceExt;

use crate::renderable::Renderable;

const VERTICES: &[[f32; 2]] = &[[0.0, 0.5], [-0.5, -0.5], [0.5, -0.5]];

const VERTICES_LAYOUT: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
    array_stride: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
    step_mode: wgpu::InputStepMode::Vertex,
    attributes: &[wgpu::VertexAttribute {
        offset: 0,
        shader_location: 0,
        format: wgpu::VertexFormat::Float2,
    }],
};

pub struct Quad {
    pipeline: wgpu::RenderPipeline,
    vert_buffer: wgpu::Buffer,
    vert_num: u32,
}

impl Quad {
    pub fn new(device: &wgpu::Device, sc_desc: &wgpu::SwapChainDescriptor) -> Quad {
        let quad_vert = device.create_shader_module(&wgpu::include_spirv!("quad.vert.spv"));
        let quad_frag = device.create_shader_module(&wgpu::include_spirv!("test.frag.spv"));

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

        let vert_num = VERTICES.len() as u32;

        Quad {
            pipeline,
            vert_buffer,
            vert_num,
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
        render_pass.draw(0..self.vert_num, 0..1);
    }
}
