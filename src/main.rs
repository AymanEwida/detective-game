extern crate freetype;
extern crate glfw;

use std::collections::HashSet;
use std::time::{Duration, Instant};

use detective_game::game::level::LevelStatus;
use detective_game::game::player::{PlayerInteraction, PlayerMouseInteraction};
use detective_game::game::store::StoreItem;
use detective_game::renderer::button::ButtonAction;
use detective_game::renderer::render::MouseInteraction;
use glfw::{
    fail_on_errors, flush_messages, Action, Context, Key, OpenGlProfileHint, WindowEvent,
    WindowHint, WindowMode,
};

use detective_game::game::{character::Direction, level::GameLevel, player::Player};
use detective_game::library::constants::{FPS, HEIGHT, WIDTH};
use detective_game::renderer::{render::Render, styles::Size, vertice::Position};

fn main() {
    let mut glfw = glfw::init(fail_on_errors).expect("Failed on init.");

    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(WindowHint::Resizable(true));

    let (mut window, events) = glfw
        .create_window(WIDTH, HEIGHT, "Derective Game", WindowMode::Windowed)
        .expect("Failed on window creation.");

    window.make_current();
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos_polling(true);
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    let (window_width, window_height) = window.get_framebuffer_size();

    let mut render = Render::new(Size {
        width: window_width as f32,
        height: window_height as f32,
    })
    .expect("Failed to created a render.");

    let mut player = Player::new(Position { x: 90.0, y: 180.0 }, true);
    let mut level = GameLevel::default();
    level.set_level(1);
    level
        .load_level(&mut player)
        .expect("Unable to load level!");

    let mut store_items = vec![
        StoreItem {
            image_path: "assets/game/upgrade.png",
            title: String::from("Q Ability Upgrade"),
            description: String::from(
                "increases the coverd space of the q ability, by adding 50 to the q ability radius",
            ),
            price: 2,
            upgrade_info: Some((0, 3)),
            error_message: String::new(),
            buy_func: Box::new(|player: &mut Player<'_>| {
                player.set_ability_radius(player.get_ability_radius() + 50.0);

                Ok(())
            }),
        },
        StoreItem {
            image_path: "assets/game/path_track_ability.png",           
            title: String::from("Q Ability New Feature"),
            description: String::from(
                "when any enemy in the ability covered space, the path of the enemy in Regular mode is shown",
            ),
            price: 3,
            upgrade_info: None,
            error_message: String::new(),
            buy_func: Box::new(|player: &mut Player<'_>| {
                if player.get_track_path_ability() {
                    return Err(String::from("You have already purchased this item"));
                }

                player.set_track_path_ability(true);

                Ok(())
            }),
        },
        StoreItem {
            image_path: "assets/game/disturb_camera.png",           
            title: String::from("Increase Disturb Duration"),
            description: String::from(
                "camera disturb duration is incresed by 1.5 seconds",
            ),
            price: 2,
            upgrade_info: Some((0, 2)),
            error_message: String::new(),
            buy_func: Box::new(|player: &mut Player<'_>| {
                player.set_camera_disturb_lifttime(player.get_camera_disturb_lifttime().as_secs() + 1);

                Ok(())
            }),
        },
    ];

    let mut last_update = Instant::now();

    window.set_framebuffer_size_callback(|window, new_width, new_height| {
        window.set_size(new_width, new_height);
    });

    let mut cursor_position = Position { x: 0.0, y: 0.0 };

    let mut pressed_buttons = HashSet::new();

    while !window.should_close() {
        let (fb_window_width, fb_window_height) = window.get_framebuffer_size();
        let (window_width, window_height) = window.get_size();

        let render_size = render.get_size();

        if (fb_window_width as f32 != render_size.width)
            || (fb_window_height as f32 != render_size.height)
        {
            render.resize(Size {
                width: window_width as f32,
                height: window_height as f32,
            });
        }

        glfw.poll_events();

        for (_, event) in flush_messages(&events) {
            match event {
                WindowEvent::CursorPos(x, y) => {
                    cursor_position = Position {
                        x: x as f32 * (fb_window_width as f32 / window_width as f32),
                        y: y as f32 * (fb_window_height as f32 / window_height as f32),
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
                                level.set_is_paused(!level.get_is_paused());
                            }
                            _ => (),
                        },

                        Key::W => match action {
                            Action::Press | Action::Repeat => {
                                if !player.get_is_using_ability() {
                                    player.move_player(Direction::Up);
                                }
                            }
                            _ => (),
                        },

                        Key::S => match action {
                            Action::Press | Action::Repeat => {
                                if !player.get_is_using_ability() {
                                    player.move_player(Direction::Down);
                                }
                            }
                            _ => (),
                        },

                        Key::A => match action {
                            Action::Press | Action::Repeat => {
                                if !player.get_is_using_ability() {
                                    player.move_player(Direction::Left);
                                }
                            }
                            _ => (),
                        },

                        Key::D => match action {
                            Action::Press | Action::Repeat => {
                                if !player.get_is_using_ability() {
                                    player.move_player(Direction::Right);
                                }
                            }
                            _ => (),
                        },

                        Key::Q => match action {
                            Action::Repeat => {
                                if level.get_status() == &LevelStatus::NotDetermine {
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

        if delta >= Duration::from_secs_f32(1.0 / FPS) {
            last_update = now;

            if player.is_off_window(render.get_size())
                || player.is_off_border(
                    Some(level.get_boder_start_position()),
                    level.get_boder_size(),
                )
            {
                player.move_to_prev_position();
            }

            level
                .draw(&mut player, &mut store_items, &mut render)
                .expect("Unable to draw level");

            render
                .handle_buttons_events(cursor_position)
                .expect("Unable to handle all buttons");
            match render.get_button_click_action() {
                ButtonAction::RetryLevel => {
                    level.load_level(&mut player).expect("Unable to load level");
                }

                ButtonAction::Exit => {
                    window.set_should_close(true);
                }

                ButtonAction::Unpause => {
                    level.set_is_paused(false);
                }

                ButtonAction::NextLevel => {
                    level.next_level();
                    level.load_level(&mut player).expect("Unable to load level");
                }

                ButtonAction::BuyStoreItem(idx) => {
                    store_items[idx].buy(&mut player);
                }

                ButtonAction::None => (),
            }

            render.render().expect("Uable to render object on window");

            window.swap_buffers();

            player.set_interaction(None);
            player.set_mouse_interaction(None);
            render.set_mouse_interaction(None);
        }
    }
}
