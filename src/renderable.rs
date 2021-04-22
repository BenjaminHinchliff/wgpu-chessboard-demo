pub trait Renderable {
    fn render(&self, encoder: &mut wgpu::CommandEncoder, frame: &wgpu::SwapChainTexture);
}
