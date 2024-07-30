use std::time::{Duration, Instant};

use glutin::{dpi::PhysicalSize, event::{ElementState, Event, VirtualKeyCode, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder, Api, ContextBuilder, GlRequest};

use detective_game::library::constants::{
    WIDTH,
    HEIGHT
};
use detective_game::game::{character::{Character, Direction}, player::Player};
use detective_game::renderer::render::{Render, Size};

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

    let mut player = Player::default();

    let mut last_update = Instant::now();
    
    // let mut is_left_button_pressed = false;
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
                                        player.move_character(Direction::Up, None);
                                    },
                                    VirtualKeyCode::S => {
                                        player.move_character(Direction::Down, None);
                                    },
                                    VirtualKeyCode::A => {
                                        player.move_character(Direction::Left, None);
                                    },
                                    VirtualKeyCode::D => {
                                        player.move_character(Direction::Right, None);
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
                    // WindowEvent::MouseInput { state, button, .. } => {
                    //     if state == ElementState::Pressed {
                    //         is_left_button_pressed = button == MouseButton::Left;
                    //     } else if state == ElementState::Released {
                    //         is_left_button_pressed = false;
                    //     }
                    // },
                    // WindowEvent::CursorMoved { position, .. } => {
                    //     if is_left_button_pressed {
                    //         let x = position.x as f32;
                    //         let y = position.y as f32;
                    //     }
                    // }
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
                render.fill_with_image("assets/game/background.jpg").expect("Unable to fill window with image");

                player.draw(&render).expect("Unable to draw player");

                render.update();

                render.render();

                gl_context.swap_buffers().unwrap();
            }
            _ => ()
        }
    });
}
