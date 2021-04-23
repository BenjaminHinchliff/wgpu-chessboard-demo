use std::mem;

use nalgebra::Vector2;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct Vertex {
    pub position: Vector2<f32>,
    pub tex_coord: Vector2<f32>,
}

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: Vector2::new(-1.0, 1.0),
        tex_coord: Vector2::new(0.0, 1.0),
    },
    Vertex {
        position: Vector2::new(-1.0, -1.0),
        tex_coord: Vector2::new(0.0, 0.0),
    },
    Vertex {
        position: Vector2::new(1.0, -1.0),
        tex_coord: Vector2::new(1.0, 0.0),
    },
    Vertex {
        position: Vector2::new(1.0, 1.0),
        tex_coord: Vector2::new(1.0, 1.0),
    },
];

#[rustfmt::skip]
pub const INDICES: &[u16] = &[
    0, 1, 2,
    2, 3, 0,
];

pub const LAYOUT: wgpu::VertexBufferLayout = wgpu::VertexBufferLayout {
    array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
    step_mode: wgpu::InputStepMode::Vertex,
    attributes: &wgpu::vertex_attr_array![0 => Float2, 1 => Float2],
};


