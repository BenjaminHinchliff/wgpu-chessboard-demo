use crate::Board;

pub trait Renderable {
    fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        queue: &mut wgpu::Queue,
        frame: &wgpu::SwapChainTexture,
        board: &Board,
    );
}
