extern crate glfw;
extern crate detective_game;

use std::time::{Duration, Instant};

use glfw::{fail_on_errors, flush_messages, Action, Context, Key, OpenGlProfileHint, WindowEvent, WindowHint, WindowMode};

use detective_game::game::{character::Direction, enemy::{Enemy, EnemyType}, level::{GameObject, ObjectLevel, ObjectLevelType}, player::Player};
use detective_game::renderer::{render::{Render, Size}, vertice::Position};
use simulator::Simulator;

pub mod simulator;

pub const SIMULATOR_WINDOW_WIDTH: u32 = 800;
pub const SIMULATOR_WINDOW_HEIGHT: u32 = 600;

fn main() {
    let mut glfw = glfw::init(fail_on_errors).expect("Failed on init.");

    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::Resizable(true));

    let (mut window, events) = glfw.create_window(SIMULATOR_WINDOW_WIDTH, SIMULATOR_WINDOW_HEIGHT, "Derective Game", WindowMode::Windowed).expect("Failed on window creation.");

    window.make_current();
    window.set_key_polling(true);
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let (window_width, window_height) = window.get_framebuffer_size();

    let mut render = Render::new(Size{ width: window_width as f32, height: window_height as f32 }).expect("Failed to created a render.");

    let mut player = Player::new(Position { x: 10.0, y: 10.0 });
    let simulator = Simulator::from(
        vec![
            ObjectLevel::new(ObjectLevelType::Wall, Position { x: 90.0, y: 90.0 }, Size { width: 200.0, height: 30.0 }),
            ObjectLevel::new(ObjectLevelType::Camera, Position { x: 170.0, y: 95.0 }, Size { width: 30.0, height: 30.0 })
        ]
    );
    let mut enemy =  Enemy::new(EnemyType::Regular, Position { x: 400.0, y: 10.0 }, "5d/0 4r/2000 4l/0 5u/2000");

    let mut last_update = Instant::now();

    window.set_framebuffer_size_callback(| window, new_width, new_height | {
        window.set_size(new_width, new_height);
    });
    
    while !window.should_close() {
        
        let (window_width, window_height) = window.get_framebuffer_size();
        let render_size = render.get_size();

        if (window_width as f32 != render_size.width) || (window_height as f32 != render_size.height) {
            render.resize(Size { width: window_width as f32, height: window_height as f32});
        }

        glfw.poll_events();
        
        for (_, event) in flush_messages(&events) {
            match event {
                WindowEvent::Key(key, _, action, _) => {
                    match key {
                        Key::Escape => {
                            match action {
                                Action::Release => {
                                    window.set_should_close(true);
                                },
                                _ => ()
                            }
                        },

                        Key::W => {
                            match action {
                                Action::Press | Action::Repeat => {
                                    player.move_player(Direction::Up, None);
                                },
                                _ => ()
                            }
                        },

                        Key::S => {
                            match action {
                                Action::Press | Action::Repeat => {
                                    player.move_player(Direction::Down, None);
                                },
                                _ => ()
                            }
                        },

                        Key::A => {
                            match action {
                                Action::Press | Action::Repeat => {
                                    player.move_player(Direction::Left, None);
                                },
                                _ => ()
                            }
                        },

                        Key::D => {
                            match action {
                                Action::Press | Action::Repeat => {
                                    player.move_player(Direction::Right, None);
                                },
                                _ => ()
                            }
                        },
                        
                        _ => ()
                    }
                },
        
                WindowEvent::Close => {
                    window.set_should_close(true);
                },
                
                _ => ()
            }
        }

        let now = Instant::now();
        let delta = now.duration_since(last_update);

        if delta >= Duration::from_secs_f32(1.0/60.0) {
            last_update = now;
            
            simulator.draw(&mut player, &mut render).expect("Unable to draw player");
            
            enemy.draw(&mut render).expect("Unable to draw enemy");
            enemy.move_enemy(None);
            
            render.render().expect("Uable to render object on window");
            
            window.swap_buffers();
        }
    }
}
