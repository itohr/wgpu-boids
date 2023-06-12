#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use winit::event::{Event, WindowEvent};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen(start))]
pub async fn run() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }
    #[cfg(target_arch = "wasm32")]
    {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
        console_log::init().expect("could not initialize logger");
    }

    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Boids")
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = winit::event_loop::ControlFlow::Exit,
                _ => {}
            },
            _ => {}
        }
    });
}
