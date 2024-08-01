use std::time::{Duration, Instant};

use glutin::{dpi::PhysicalSize, event::{ElementState, Event, VirtualKeyCode, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder, Api, ContextBuilder, GlRequest};

use detective_game::library::constants::{
    HEIGHT, WIDTH
};
use detective_game::game::{character::Direction, level::{ObjectLevel, ObjectLevelType, GameObject}, player::Player, enemy::{Enemy, EnemyType}};
use detective_game::renderer::{render::{Render, Size}, vertice::Position};

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
    let mut enemy =  Enemy::new(EnemyType::Regular, Position { x: 400.0, y: 10.0 }, "5d 4r 4l 5u");
    let wall = ObjectLevel::new(ObjectLevelType::Wall, Position { x: 150.0, y: 30.0 }, Size { width: 50.0, height: 150.0 });
    
    let mut last_update = Instant::now();

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
                                        player.move_player(Direction::Up, None);
                                    },
                                    VirtualKeyCode::S => {
                                        player.move_player(Direction::Down, None);
                                    },
                                    VirtualKeyCode::A => {
                                        player.move_player(Direction::Left, None);
                                    },
                                    VirtualKeyCode::D => {
                                        player.move_player(Direction::Right, None);
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
                if player.collide(&wall) {
                    player.move_player_to_prev_position();
                }

                render.fill_with_image("assets/game/background.jpg").expect("Unable to fill window with image");

                player.draw(&mut render).expect("Unable to draw player");
                
                enemy.draw(&mut render).expect("Unable to draw enemy");
                enemy.move_enemy(None);
                
                wall.draw(&mut render).expect("Unable to draw level");
                
                render.render().expect("Uable to render object on window");
                
                gl_context.swap_buffers().unwrap();
            }
            _ => ()
        }
    });
}
