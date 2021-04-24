use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    dpi::PhysicalSize,
};

use chessboard_rs::BoardView;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(400, 400))
        .build(&event_loop)?;

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
        Event::RedrawRequested(_) => match board_view.render() {
            Ok(_) => {}
            Err(wgpu::SwapChainError::Lost) => board_view.resize(board_view.size()),
            Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
            Err(e) => eprintln!("{:?}", e),
        },
        _ => {}
    });
}
