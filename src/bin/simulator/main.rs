extern crate detective_game;
extern crate glfw;

use std::collections::HashSet;
use std::time::{Duration, Instant};

use detective_game::game::player::{PlayerInteraction, PlayerMouseInteraction};
use detective_game::renderer::button::ButtonAction;
use detective_game::renderer::render::MouseInteraction;
use glfw::{
    fail_on_errors, flush_messages, Action, Context, Key, OpenGlProfileHint, WindowEvent,
    WindowHint, WindowMode,
};

use detective_game::game::{character::Direction, player::Player};
use detective_game::renderer::{render::Render, styles::Size, vertice::Position};
use simulator::{SimulationStatus, Simulator, SimulatorType};

pub mod simulator;

pub const SIMULATOR_WINDOW_WIDTH: u32 = 800;
pub const SIMULATOR_WINDOW_HEIGHT: u32 = 600;
pub const SIMULATION_FPS: f32 = 60.0;

fn main() {
    let mut glfw = glfw::init(fail_on_errors).expect("Failed on init.");

    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::Resizable(false));

    let (mut window, events) = glfw
        .create_window(
            SIMULATOR_WINDOW_WIDTH,
            SIMULATOR_WINDOW_HEIGHT,
            "Derective Game",
            WindowMode::Windowed,
        )
        .expect("Failed on window creation.");

    window.make_current();
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos_polling(true);
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let (fb_width, fb_height) = window.get_framebuffer_size();
    let (window_width, window_height) = window.get_size();
    unsafe {
        gl::Viewport(0, 0, fb_width, fb_height);
    }

    let mut render = Render::with_projection_size(
        Size {
            width: window_width as f32,
            height: window_height as f32,
        },
        Size {
            width: fb_width as f32,
            height: fb_height as f32,
        },
        Size {
            width: window_width as f32,
            height: window_height as f32,
        },
    )
    .expect("Failed to created a render.");

    let mut player = Player::new(Position { x: 10.0, y: 10.0 }, true);
    let mut simulator = Simulator::new();

    let simulator_type = SimulatorType::CameraRotation;
    simulator
        .load_simulation(simulator_type.clone(), &mut player)
        .expect("Unable to load simulation");

    let mut last_update = Instant::now();

    let mut cursor_position = Position { x: 0.0, y: 0.0 };

    let mut pressed_buttons = HashSet::new();

    while !window.should_close() {
        let (fb_window_width, fb_window_height) = window.get_framebuffer_size();
        let (window_width, window_height) = window.get_size();

        let render_size = render.get_window_size();

        if (window_width as f32 != render_size.width)
            || (window_height as f32 != render_size.height)
        {
            render.resize(Size {
                width: fb_window_width as f32,
                height: fb_window_height as f32,
            });

            render.set_window_size(Size {
                width: window_width as f32,
                height: window_height as f32,
            });
        }

        glfw.poll_events();

        for (_, event) in flush_messages(&events) {
            match event {
                WindowEvent::CursorPos(x, y) => {
                    cursor_position = Position {
                        x: x as f32,
                        y: y as f32,
                    };
                }

                WindowEvent::MouseButton(mouse_button, action, _) => {
                    player.set_mouse_interaction(Some(PlayerMouseInteraction::new(
                        mouse_button,
                        action,
                        cursor_position,
                    )));
                    render.set_mouse_interaction(Some(MouseInteraction::new(
                        cursor_position,
                        mouse_button,
                        action,
                    )));

                    match action {
                        Action::Press => {
                            pressed_buttons.insert(mouse_button);
                        }

                        Action::Release => {
                            pressed_buttons.remove(&mouse_button);
                        }

                        _ => (),
                    }
                }

                WindowEvent::Key(key, _, action, _) => {
                    player.set_interaction(Some(PlayerInteraction::new(key, action)));

                    match key {
                        Key::Escape => match action {
                            Action::Release => {
                                window.set_should_close(true);
                            }
                            _ => (),
                        },

                        Key::W => match action {
                            Action::Press | Action::Repeat => {
                                if !player.get_is_using_ability()
                                    && simulator.get_status() == &SimulationStatus::NotDetermine
                                {
                                    player.move_player(Direction::Up);
                                }
                            }
                            _ => (),
                        },

                        Key::S => match action {
                            Action::Press | Action::Repeat => {
                                if !player.get_is_using_ability()
                                    && simulator.get_status() == &SimulationStatus::NotDetermine
                                {
                                    player.move_player(Direction::Down);
                                }
                            }
                            _ => (),
                        },

                        Key::A => match action {
                            Action::Press | Action::Repeat => {
                                if !player.get_is_using_ability()
                                    && simulator.get_status() == &SimulationStatus::NotDetermine
                                {
                                    player.move_player(Direction::Left);
                                }
                            }
                            _ => (),
                        },

                        Key::D => match action {
                            Action::Press | Action::Repeat => {
                                if !player.get_is_using_ability()
                                    && simulator.get_status() == &SimulationStatus::NotDetermine
                                {
                                    player.move_player(Direction::Right);
                                }
                            }
                            _ => (),
                        },

                        Key::Q => match action {
                            Action::Repeat => {
                                if simulator.get_status() == &SimulationStatus::NotDetermine {
                                    player.set_is_using_ability(true);
                                }
                            }

                            Action::Release => {
                                player.set_is_using_ability(false);
                            }

                            _ => (),
                        },

                        _ => (),
                    }
                }

                WindowEvent::Close => {
                    window.set_should_close(true);
                }

                _ => (),
            }
        }

        for pressed_button in pressed_buttons.iter() {
            player.set_mouse_interaction(Some(PlayerMouseInteraction::new(
                *pressed_button,
                Action::Press,
                cursor_position,
            )));
            render.set_mouse_interaction(Some(MouseInteraction::new(
                cursor_position,
                *pressed_button,
                Action::Press,
            )));
        }

        let now = Instant::now();
        let delta = now.duration_since(last_update);

        if delta >= Duration::from_secs_f32(1.0 / SIMULATION_FPS) {
            last_update = now;

            if player.is_off_window(render.get_window_size()) {
                player.move_to_prev_position();
            }

            simulator
                .draw(&mut player, &mut render)
                .expect("Unable to draw player");

            render
                .handle_buttons_events(cursor_position)
                .expect("Unable to handle all buttons");
            match render.get_button_click_action() {
                ButtonAction::RetryLevel => {
                    simulator
                        .load_simulation(simulator_type.clone(), &mut player)
                        .expect("Unable to load level");
                }

                ButtonAction::Exit => {
                    window.set_should_close(true);
                }

                ButtonAction::None
                | ButtonAction::NextLevel
                | ButtonAction::BuyStoreItem(_)
                | ButtonAction::Unpause => (),
            }

            render.render().expect("Uable to render object on window");

            window.swap_buffers();

            player.set_interaction(None);
            player.set_mouse_interaction(None);
            render.set_mouse_interaction(None);
        }
    }
}
