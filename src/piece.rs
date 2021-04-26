use bytemuck::{Pod, Zeroable};
use nalgebra::{Matrix4, Vector2, Vector3};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    pub color: Color,
    pub type_: Type,
}

fn board_coord_to_world(board: usize) -> f32 {
    ((board as f32 + 0.5) / 8.0 * 2.0) - 1.0
}

impl Piece {
    pub const fn new(color: Color, type_: Type) -> Piece {
        Piece { color, type_ }
    }

    pub fn to_raw(&self, x: usize, y: usize) -> PieceRaw {
        PieceRaw {
            position: Matrix4::new_scaling(1.0 / 8.0).append_translation(&Vector3::new(
                board_coord_to_world(x),
                board_coord_to_world(y),
                0.0,
            )),
            types: Vector2::new(self.type_ as u16, self.color as u16),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Pod, Zeroable)]
pub struct PieceRaw {
    position: Matrix4<f32>,
    types: Vector2<u16>,
}

impl PieceRaw {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<PieceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<Matrix4<f32>>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Ushort2,
                },
            ],
        }
    }
}
