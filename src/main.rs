use std::time::{Duration, Instant};

use glutin::{dpi::PhysicalSize, event::{ElementState, Event, MouseButton, VirtualKeyCode, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder, Api, ContextBuilder, GlRequest};

use library::constants::{
    WIDTH,
    HEIGHT
};
use renderer::{color::Color, render::{Render, Size}, vertice::{Position, Vertice}};

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

    let mut render = Render::new(Size{ width: window_size.width as f32, height: window_size.height as f32 }).expect("Failed to created a render");

    let mut last_update = Instant::now();

    let mut is_left_button_pressed = false;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        
        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(new_physical_size) => {
                        gl_context.resize(new_physical_size);
                        render.resize(Size { width: new_physical_size.width as f32, height: new_physical_size.height as f32 });
                    },
                    WindowEvent::KeyboardInput { input, .. } => {
                        if input.state == ElementState::Pressed {
                            if let Some(virtual_keycode) = input.virtual_keycode {
                                match virtual_keycode {
                                    VirtualKeyCode::W => {
                                        let black_background = render.fill_with_color(Color::Black);
                                        render.update(Some(black_background), vec![]);
                                    },
                                    VirtualKeyCode::S => {
                                        let white_background = render.fill_with_color(Color::White);
                                        render.update(Some(white_background), vec![]);
                                    },
                                    VirtualKeyCode::A => {
                                        let green_background = render.fill_with_color(Color::Green);
                                        render.update(Some(green_background), vec![]);
                                    },
                                    VirtualKeyCode::D => {
                                        let blue_background = render.fill_with_color(Color::Blue);
                                        render.update(Some(blue_background), vec![]);
                                    },
                                    VirtualKeyCode::Up => {
                                        let gray_background = render.fill_with_color(Color::RGB(50, 50, 50));
                                        render.update(Some(gray_background), vec![]);
                                    },
                                    VirtualKeyCode::Down => {
                                        let yellow_background = render.fill_with_color(Color::RGBA(255, 255, 0, 50));
                                        render.update(Some(yellow_background), vec![]);
                                    },
                                    VirtualKeyCode::Left => {
                                        let red_background = render.fill_with_color(Color::Red);
                                        render.update(Some(red_background), vec![]);
                                    },
                                    VirtualKeyCode::Right => {
                                        let circle = render.draw_circle("circle".to_string(), Position { x: 400.0, y: 300.0 }, 200.0, Color::RGBA(0, 255, 255, 50), Some(10000)).expect("Unable to draw circle");
                                        render.update(None, vec![circle]);
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
                    },
                    WindowEvent::MouseInput { state, button, .. } => {
                        if state == ElementState::Pressed {
                            is_left_button_pressed = button == MouseButton::Left;
                        } else if state == ElementState::Released {
                            is_left_button_pressed = false;
                        }
                    },
                    WindowEvent::CursorMoved { position, .. } => {
                        if is_left_button_pressed {
                            let x = position.x as f32;
                            let y = position.y as f32;

                            let line = render.draw_curved_line("curved_line".to_string(), Position { x: 100.0, y: 100.0 }, Position { x, y }, Color::White, None).expect("Unable to draw curved_line");
                            render.update(None, vec![line]);
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
                let image_background = render.fill_with_image("assets/test/background-test.jpg").expect("Unable to fill window with image");

                let triangle = render.draw_triangle(
                    "triangle".to_string(),
                    [
                        Vertice(Position { x: 200.0, y: 300.0 }, Color::Red),
                        Vertice(Position { x: 600.0, y: 300.0 }, Color::Green),
                        Vertice(Position { x: 0.0, y: 0.0 }, Color::Blue),
                    ]
                ).expect("Unable to draw triangle");
                //let rectangle1 = render.draw_rectangle("rectangle1".to_string(), Position { x: 200.0, y: 150.0 }, Size { width: 400.0, height: 300.0 }, Color::RGB(255, 0, 255)).expect("Unable to draw rectangle");
                // let curved_line = render.draw_curved_line("curved_line".to_string(), Position { x: 200.0, y: 200.0 }, Position { x: 500.0, y: 100.0 }, Color::Red, None).expect("Unable to draw curved line");
                // let line1 = render.draw_line("line1".to_string(), Position { x: 50.0, y: 50.0 }, Position { x: 150.0, y: 50.0 }, Color::Red).expect("Unable to draw line");
                // let line2 = render.draw_line("line2".to_string(), Position { x: 150.0, y: 50.0 }, Position { x: 150.0, y: 100.0 }, Color::Red).expect("Unable to draw line");
                // let image1 = render.load_image("image1".to_string(), "assets/test/test-ferris.png", Position { x: 200.0, y: 200.0 }, Size { width: 200.0, height: 200.0 }).expect("Unable to load image");
                // let image2 = render.load_image("image2".to_string(), "assets/test/test.jpg", Position { x: 500.0, y: 200.0 }, Size { width: 200.0, height: 200.0 }).expect("Unable to load image");

                render.update(Some(image_background), vec![triangle]);

                render.render();

                gl_context.swap_buffers().unwrap();
            }
            _ => ()
        }
    });
}
