use std::time::{Duration, Instant};

use glutin::{dpi::PhysicalSize, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder, Api, ContextBuilder, GlRequest};

use constants::{
    WIDTH,
    HEIGHT
};

mod constants;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().with_title("Derective Game").with_inner_size(PhysicalSize { width: WIDTH, height: HEIGHT });

    let gl_context = ContextBuilder::new()
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
        .build_windowed(window, &event_loop)
        .expect("Cannot create windowed context");

    let gl_context = unsafe {
        gl_context
            .make_current()
            .expect("Failed to make context current")
    };

    gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let mut last_update = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(new_physical_size) => gl_context.resize(new_physical_size),
                    _ => ()
                }
            },
            Event::MainEventsCleared => {
                let now = Instant::now();
                let delta = now.duration_since(last_update);

                if delta >= Duration::from_secs_f32(1.0/60.0) {
                    last_update = now;

                    gl_context.window().request_redraw();
                }
            },
            Event::RedrawRequested(_) => {
                gl_context.swap_buffers().unwrap();
            }
            _ => ()
        }
    });
}
