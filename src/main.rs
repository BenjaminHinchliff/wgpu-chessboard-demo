use anyhow::Context;
use image::GenericImageView;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Icon, WindowBuilder},
};

use chessboard_rs::{board, BoardView};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let icon_data = include_bytes!("images/icon.png");
    let icon_image = image::load_from_memory(icon_data)?;
    let icon_dims = icon_image.dimensions();
    let icon_rgba = icon_image.into_rgba8();
    let icon = Icon::from_rgba(icon_rgba.into_raw(), icon_dims.0, icon_dims.1)?;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Chess Board")
        .with_window_icon(Some(icon))
        .with_inner_size(PhysicalSize::new(400, 400))
        .build(&event_loop)?;

    let board = board::default_board();
    let mut board_view = BoardView::create(&window).await;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            WindowEvent::Resized(physical_size) => board_view.resize(*physical_size),
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                board_view.resize(**new_inner_size)
            }
            _ => {}
        },
        Event::RedrawRequested(_) => match board_view.render(&board) {
            Ok(_) => {}
            Err(wgpu::SwapChainError::Lost) => board_view.resize(board_view.size()),
            Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
            Err(e) => eprintln!("{:?}", e),
        },
        _ => {}
    });
}
