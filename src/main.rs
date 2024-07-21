use std::time::{Duration, Instant};

use glutin::{dpi::PhysicalSize, event::{ElementState, Event, VirtualKeyCode, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder, Api, ContextBuilder, GlRequest};

use library::constants::{
    WIDTH,
    HEIGHT
};
use renderer::{color::Color, render::Render, vertice::Vertice};

mod renderer;
mod library;

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

    let window_size = gl_context.window().inner_size();

    let mut render = Render::new((window_size.width as usize, window_size.height as usize)).expect("Failed to created a render");

    let mut last_update = Instant::now();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        
        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(new_physical_size) => {
                        gl_context.resize(new_physical_size);
                        render.resize((new_physical_size.width as usize, new_physical_size.height as usize)).expect("Faild to resize render");
                    },
                    WindowEvent::KeyboardInput { input, .. } => {
                        if input.state == ElementState::Pressed {
                            if let Some(virtual_keycode) = input.virtual_keycode {
                                match virtual_keycode {
                                    VirtualKeyCode::W => {
                                        render.fill_with_color(Color::Black);
                                    },
                                    VirtualKeyCode::S => {
                                        render.fill_with_color(Color::White);
                                    },
                                    VirtualKeyCode::A => {
                                        render.fill_with_color(Color::Green);
                                    },
                                    VirtualKeyCode::D => {
                                        render.fill_with_color(Color::Blue);
                                    },
                                    VirtualKeyCode::Up => {
                                        render.fill_with_color(Color::RGB(50, 50, 50))
                                    },
                                    VirtualKeyCode::Down => {
                                        render.fill_with_color(Color::RGBA(255, 255, 0, 255))
                                    },
                                    VirtualKeyCode::Left => {
                                        render.fill_with_color(Color::Red);
                                    },
                                    VirtualKeyCode::Right => {
                                        todo!()
                                    },
                                    _ => ()
                                }
                            }
                        } else if input.state == ElementState::Released {
                            if let Some(virtual_keycode) = input.virtual_keycode {
                                match virtual_keycode {
                                    VirtualKeyCode::Escape => *control_flow = ControlFlow::Exit,
                                    _ => ()
                                }
                            }
                        }
                    }
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
                render.draw_triangle([
                    Vertice([-200.0, -300.0], Color::Red),
                    Vertice([200.0, -300.0], Color::Green),
                    Vertice([0.0, 300.0], Color::Blue),
                ]);
                render.draw();
                gl_context.swap_buffers().unwrap();
            }
            _ => ()
        }
    });
}
