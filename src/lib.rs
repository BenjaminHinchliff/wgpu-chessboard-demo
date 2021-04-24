use std::iter;

use renderable::Renderable;
use winit::{dpi::PhysicalSize, window::Window};

mod piece;
use piece::Piece;

mod renderable;

mod quad;

mod background;
use background::Background;

mod pieces;
use pieces::PiecesView;

type Board = [[Option<Piece>; 8]; 8];

pub struct BoardView {
    pub board: Board,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: PhysicalSize<u32>,
    background: Background,
    pieces_view: PiecesView, 
}

impl BoardView {
    pub async fn create(window: &Window) -> BoardView {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let background = Background::new(&device, &sc_desc);

        let pieces_view = PiecesView::new(&device, &queue, &sc_desc);

        Self {
            board: [[None; 8]; 8],
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,
            background,
            pieces_view,
        }
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        return self.size;
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        log::info!("resizing to {:?}", new_size);
        self.size = new_size;
        self.sc_desc.width = new_size.width.max(1);
        self.sc_desc.height = new_size.height.max(1);
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let _ = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
        }

        self.background.render(&mut encoder, &frame);

        self.pieces_view.render(&mut encoder, &frame);

        self.queue.submit(iter::once(encoder.finish()));

        Ok(())
    }
}
