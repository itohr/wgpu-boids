mod state;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

use winit::dpi::PhysicalSize;
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
    window.set_inner_size(PhysicalSize::new(1600, 1600));

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("wasm-canvas")?;
                let canvas = web_sys::Element::from(window.canvas());
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Could not get canvas element");
    }

    let mut state = state::State::new(&window).await;

    let mut t = 0;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = winit::event_loop::ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = winit::event_loop::ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    state.resize(*physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    state.resize(**new_inner_size);
                }
                _ => {}
            },
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                match state.render(t) {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size()),
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        *control_flow = winit::event_loop::ControlFlow::Exit
                    }
                    Err(e) => eprintln!("{:?}", e),
                }
                t += 1;
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
