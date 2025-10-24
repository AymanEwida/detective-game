use glfw::{Action, Key};
use queues::{IsQueue, Queue};

use crate::{game::{bullet::BulletType, enemy::{EnemyMode, EnemyType, SearchingMode}, player::{PlayerStatus, ShootObject}}, library::{constants::DEFAULT_MOVEMENT_VALUE, utils::{absolute_f32, get_attached_enemy_index, get_correct_start_position, get_level_challenges, get_nearest_enemy_id, is_in_circle, round_position_to_full_numbers}}, renderer::{button::{ButtonAction, OnHoverStylesBuilder}, color::Color, error::Result, render::{ButtonProps, Render}, styles::{Padding, Size}, vertice::Position}};

use super::{bullet::Bullet, camera::Camera, can::Can, challenge::{Challenge, ChallengeStatus}, character::Character, collectable::{Coin, DoorCollectable, DoorCollectableType}, detect_range::DetectRange, door::{Door, DoorType, ExitDoor, TeleportDoor}, enemy::Enemy, hide_place::HidePlace, player::{InventoryItem, Player, DEFAULT_SIZE_FOR_INVENTORY_ITEM}, store::StoreItem, wall::Wall};

pub const DEFAULT_SIZE: f32 = 30.0;
pub const DEFAULT_SIZE_FOR_HIDE_PLACE: Size = Size { width: 45.0, height: 65.0 };
pub const DEFAULT_SIZE_FOR_COLLECTABLE: Size = Size { width: 40.0, height: 40.0 };
pub const DEFAULT_SIZE_FOR_TELEPORT_DOOR: Size = Size { width: DEFAULT_SIZE + 20.0, height: 70.0 };
pub const DEFAULT_SIZE_FOR_EXIT_DOOR: Size = Size { width: 70.0, height: 60.0 };

pub type EndStartPositions = (Position, Position);

pub trait GameObject<'a> {
    fn draw(&self, render: &mut Render<'a>) -> Result<()>;
    fn get_position(&self) -> Position;
    fn set_position(&mut self, new_position: Position);
    fn get_size(&self) -> Size;
    fn get_calc_position(&self) -> EndStartPositions;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LevelStatus {
    Lose,
    Win,
    NotDetermine,
    ReLoadLevel
}

#[derive(Debug)]
pub enum AttachedType {
    CameraDetect,
   DetectRangeSearch 
}

#[derive(Debug)]
pub struct GameLevel<'a> {
    border_top_left: Position,
    border_size: Size,
    background_image: &'a str,
    current_level: u8,
    enemies: Vec<Enemy<'a>>,
    attached_enemies_ids: Vec<(usize, Position, AttachedType)>,
    walls: Vec<Wall<'a>>,
    doors: Vec<Door<'a>>,
    door_collectables: Vec<DoorCollectable<'a>>,
    teleport_doors: Vec<TeleportDoor<'a>>,
    exit_door: Option<ExitDoor<'a>>,
    hide_places: Vec<HidePlace<'a>>,
    coins: Vec<Coin<'a>>,
    cameras: Vec<Camera<'a>>,
    cans: Queue<Can<'a>>,
    bullets: Queue<Bullet<'a>>,
    detecting_ranges: Vec<DetectRange>,
    challenges: Vec<Challenge>,
    notoriety_level: u64,
    status: LevelStatus,
    add_amount_after_lost: usize,
    is_paused: bool
}

impl Default for GameLevel<'_> {
    fn default() -> Self {
        // TODO: do tutorial here

        let enemies = vec![];

        Self {
            border_top_left: Position { x: 50.0, y: 140.0 },
            border_size: Size { width: 1820.0, height: 740.0 },
            background_image: "assets/game/background.jpg",
            current_level: 0,
            enemies,
            attached_enemies_ids: Vec::new(),
            walls: Vec::new(),
            doors: Vec::new(),
            door_collectables: Vec::new(),
            teleport_doors: Vec::new(),
            exit_door: None,
            hide_places: Vec::new(),
            coins: Vec::new(),
            cameras: Vec::new(),
            cans: Queue::new(),
            bullets: Queue::new(),
            detecting_ranges: Vec::new(),
            challenges: Vec::new(),
            notoriety_level: 0,
            status: LevelStatus::NotDetermine,
            add_amount_after_lost: 5,
            is_paused: false,
        }
    }
}

pub fn display_holding_item<'a>(start_position: Position, holding_item: Option<InventoryItem<'a>>, scale: f32, render: &mut Render<'a>) -> Result<()> {
    let negate_scale = absolute_f32(1.0 - scale);

    if let Some(item) = holding_item {
        render.display_text(&format!("holding: {}", item.get_name()), start_position, scale, None, Color::White)?; 

        render.load_image(item.get_image(), Position { x: start_position.x + 100.0, y: start_position.y + (55.0 - (negate_scale * 50.0)) }, DEFAULT_SIZE_FOR_INVENTORY_ITEM, false, None, None, None, None)?;

        render.display_text(&format!("| {}", item.get_amount()), Position { x: start_position.x + 150.0, y: start_position.y + 50.0 }, scale, None, Color::White)?;

        if let Some(ammo) = item.get_ammo() {
            render.display_text(&format!("| {}", ammo), Position { x: start_position.x + 230.0, y: start_position.y + 50.0 }, scale, None, Color::White)?;

            render.load_image("assets/game/pile-of-ammo.png", Position { x: start_position.x + 310.0, y: start_position.y + (50.0 - (negate_scale * 45.0)) }, Size { width: 50.0, height: 50.0 }, false, None, None, None, None)?;
        }
    } else {
        render.display_text("holding: nothing", start_position, scale, None, Color::White)?; 
    }

    Ok(())
}

impl<'a> GameLevel<'a> {
    pub fn get_boder_size(&self) -> Size {
        Size { width: self.border_size.width - (DEFAULT_SIZE * 2.0), height: self.border_size.height - (DEFAULT_SIZE * 2.0) }
    }

    pub fn get_boder_start_position(&self) -> Position {
        self.border_top_left + DEFAULT_SIZE
    }

    pub fn draw(&mut self, player: &mut Player<'a>, store_items: &mut [StoreItem<'a>], render: &mut Render<'a>) -> Result<()> {
        // TODO: Remove this later
        if self.current_level == 1 {
            self.status = LevelStatus::Win;
        }
    
        if self.get_status() == &LevelStatus::ReLoadLevel {
            self.load_level(player).expect(&format!("Can not load level: {}", self.current_level));
        } else if self.get_status() == &LevelStatus::Lose {
            render.display_text("You Lost!", Position { x: 1200.0, y: 500.0 }, 2.0, None, Color::Red)?;
            render.display_button(ButtonProps {
                position: Position { x: 1300.0, y: 630.0 },
                bg_color: Color::Green,
                width: None,
                height: None,
                text: String::from("Retry Level"),
                text_scale: 1.0,
                text_color: Color::Black,
                padding: Padding::new(10.0, 15.0, 20.0, 15.0),
                on_hover_styles: OnHoverStylesBuilder::new()
                                .bg_color(Color::RGBA(0, 255, 0, 150))
                                .build(),
                click_action: ButtonAction::RetryLevel,
                on_click: Box::new(|| {}),
                on_hover: Box::new(|| {}),
                on_hover_release: Box::new(|| {})
            });

            render.display_button(ButtonProps {
                position: Position { x: 1310.0, y: 720.0 },
                bg_color: Color::Green,
                width: None,
                height: None,
                text: String::from("Exit Game"),
                text_scale: 1.0,
                text_color: Color::Black,
                padding: Padding::new(10.0, 15.0, 20.0, 15.0),
                on_hover_styles: OnHoverStylesBuilder::new()
                                .bg_color(Color::RGBA(0, 255, 0, 150))
                                .build(),
                click_action: ButtonAction::Exit,
                on_click: Box::new(|| {}),
                on_hover: Box::new(|| {}),
                on_hover_release: Box::new(|| {})
            });
        } else if self.status == LevelStatus::Win {
            render.display_text("You Won", Position { x: 1250.0, y: 200.0 }, 2.0, None, Color::Green)?;
        
            for (idx , challenge) in self.challenges.iter().enumerate() {
                let color = if challenge.get_status() == &ChallengeStatus::Completed {
                    Color::Green
                } else {
                    Color::Red
                };

                render.display_text(&format!("Challenge {}: {} - completed +{} coins", idx + 1, challenge.get_challenge_text(), challenge.get_reward()), Position { x: 1000.0, y: 330.0 + (idx as f32 * 40.0) }, 0.6, None, color)?;
            }

            render.display_text("Buy anything from the store", Position { x: 1060.0, y: 480.0 }, 0.6, None, Color::White)?;
            render.load_image("assets/game/coin.png", Position { x: 1650.0, y: 475.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None, None, None)?;
            render.display_text(&format!("{}", player.get_coins()), Position { x: 1700.0, y: 475.0 }, 1.0, None, Color::White)?;

            let offset = 530.0;
            for (idx, store_item) in store_items.iter_mut().enumerate() {
                render.draw_rectangle(Position { x: 50.0 + (idx as f32 * offset), y: 600.0 }, Size { width: 500.0, height: 600.0 }, Color::White, None, None, None);

                render.load_image(store_item.get_image_path(), Position { x: 175.0 + (idx as f32 * offset), y: 610.0 }, Size { width: 225.0, height: 190.0 }, false, None, None, None, None)?;
                render.display_text(store_item.get_title(), Position { x: 130.0 + (idx as f32 * offset), y: 820.0 }, 0.6, Some(370.0), Color::Black)?;
                render.display_text(store_item.get_description(), Position { x: 60.0 + (idx as f32 * offset), y: 890.0 }, 0.5, Some(490.0), Color::Black)?;

                render.display_button(ButtonProps {
                    position: Position { x: 80.0 + (idx as f32 * offset), y: 1150.0 },
                    bg_color: Color::Blue,
                    width: None,
                    height: None, 
                    padding: Padding::new(10.0, 15.0, 20.0, 15.0),
                    text: String::from("Buy"),
                    text_color: Color::White,
                    text_scale: 0.7,
                    on_hover_styles: OnHoverStylesBuilder::new()
                        .bg_color(Color::RGBA(0, 0, 255, 150))
                        .build(),
                    click_action: ButtonAction::BuyStoreItem(idx),
                    on_click: Box::new(|| {}),
                    on_hover: Box::new(|| {}),
                    on_hover_release: Box::new(|| {})
                });

                if let Some(upgrade_info) = store_item.get_upgrade_info() {
                    render.display_text(&format!("({}/{})", upgrade_info.0, upgrade_info.1), Position { x: 160.0 + (idx as f32 * offset), y: 1140.0 }, 0.7, None, Color::Black)?;
                }

                if store_item.get_error_message() != "" {
                    render.display_text(store_item.get_error_message(), Position { x: 60.0 + (idx as f32 * offset), y: 1230.0 }, 0.6, None, Color::Red)?;
                }

                render.load_image("assets/game/coin.png", Position { x: 430.0 + (idx as f32 * offset), y: 1140.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None, None, None)?;
                render.display_text(&format!("{}", store_item.get_price()), Position { x: 480.0 + (idx as f32 * offset), y: 1150.0 }, 0.6, None, Color::Black)?;
            }

            render.display_button(ButtonProps {
                position: Position { x: 1220.0, y: 1320.0 },
                width: None,
                height: None,
                padding: Padding::new(10.0, 15.0, 20.0, 15.0),
                bg_color: Color::Green,
                text: String::from("Continue to next level"),
                text_scale: 1.0,
                text_color: Color::Black,
                on_hover_styles: OnHoverStylesBuilder::new()
                                .bg_color(Color::RGBA(0, 255, 0, 150))
                                .build(),
                click_action: ButtonAction::NextLevel,
                on_click: Box::new(|| {}),
                on_hover: Box::new(|| {}),
                on_hover_release: Box::new(|| {})
            });
        } else {
            render.load_image(self.background_image, self.border_top_left, self.border_size, false, None, None, None, None)?;

            for num in 0..8 {
                // border top
                render.load_image("assets/game/wall.jpg", Position { x: self.border_top_left.x + num as f32 * (self.border_size.width / 8.0), y: self.border_top_left.y }, Size { width: self.border_size.width / 8.0, height: DEFAULT_SIZE }, false, None, None, None, None)?;
                
                // border bottom
                render.load_image("assets/game/wall.jpg", Position { x: self.border_top_left.x + num as f32 * (self.border_size.width / 8.0), y: self.border_top_left.y + self.border_size.height - DEFAULT_SIZE }, Size { width: self.border_size.width / 8.0, height: DEFAULT_SIZE }, false, None, None, None, None)?;
            }

            for num in 0..2 {
                // border right
                render.load_image("assets/game/wall.jpg", Position { x: self.border_top_left.x + self.border_size.width - DEFAULT_SIZE, y: self.border_top_left.y + ((num as f32 - 1.0) * DEFAULT_SIZE).abs() + num as f32 * (self.border_size.height / 2.0) }, Size { width: DEFAULT_SIZE, height: (self.border_size.height - 60.0) / 2.0 }, false, None, None, None, None)?;

                // border left
                render.load_image("assets/game/wall.jpg", Position { x: self.border_top_left.x, y: self.border_top_left.y + ((num as f32 - 1.0) * DEFAULT_SIZE).abs() + num as f32 * (self.border_size.height / 2.0) }, Size { width: DEFAULT_SIZE, height: (self.border_size.height - 60.0) / 2.0 }, false, None, None, None, None)?;
            }

            if self.is_paused {
                render.display_button(ButtonProps {
                    position: Position { x: 880.0, y: 430.0 },
                    bg_color: Color::Green,
                    width: None,
                    height: None,
                    text: String::from("Resume"),
                    text_scale: 1.0,
                    text_color: Color::Black,
                    padding: Padding::new(10.0, 10.0, 10.0, 20.0),
                    on_hover_styles: OnHoverStylesBuilder::new()
                        .bg_color(Color::RGBA(0, 255, 0, 150))
                        .build(),
                    click_action: ButtonAction::Unpause,
                    on_click: Box::new(|| {}),
                    on_hover: Box::new(|| {}),
                    on_hover_release: Box::new(|| {})
                });

                render.display_button(ButtonProps {
                    position: Position { x: 850.0, y: 520.0 },
                    bg_color: Color::Red,
                    width: None,
                    height: None,
                    text: String::from("Exit Game"),
                    text_scale: 1.0,
                    text_color: Color::Black,
                    padding: Padding::new(10.0, 15.0, 20.0, 15.0),
                    on_hover_styles: OnHoverStylesBuilder::new()
                        .bg_color(Color::RGBA(255, 0, 0, 150))
                        .build(),
                    click_action: ButtonAction::Exit,
                    on_click: Box::new(|| {}),
                    on_hover: Box::new(|| {}),
                    on_hover_release: Box::new(|| {})
                });

                return Ok(())
            }

            for (idx , challenge) in self.challenges.iter_mut().enumerate() {
                if challenge.get_status() == &ChallengeStatus::NotDetermine {
                    challenge.set_status(challenge.check_challenge(player, false, self.notoriety_level));
                }

                let color = match challenge.get_status() {
                    &ChallengeStatus::Completed => {
                        Color::Green
                    },

                    &ChallengeStatus::Failed => {
                        Color::Red
                    },

                    &ChallengeStatus::NotDetermine => {
                        Color::White
                    }
                };

                render.display_text(&format!("{} - {} coins", challenge.get_challenge_text(), challenge.get_reward()), Position { x: 50.0, y: 20.0 + (idx as f32 * 40.0) }, 0.5, None, color)?;
            }

            render.display_text(&format!("notoriety level: {}", self.notoriety_level), Position { x: 850.0, y: 60.0 }, 0.8, None, Color::White)?;

            render.display_text(&format!("level: {}", self.current_level), Position { x: 1580.0, y: 50.0 }, 0.6, None, Color::White)?;
            render.display_text(&format!("status: {}", player.get_status()), Position { x: 1580.0, y: 100.0 }, 0.7, None, Color::White)?;

            let holding_item = player.get_holding_item();

            display_holding_item(Position { x: 50.0, y:  900.0 }, holding_item, 0.8, render)?;

            render.display_text(&format!("lifes: {}", player.get_lifes()), Position { x: 920.0, y: 900.0 }, 0.8, None, Color::White)?;

            render.load_image("assets/game/coin.png", Position { x: 1650.0, y: 890.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None, None, None)?;
            render.display_text(&format!("{}", player.get_coins()), Position { x: 1700.0, y: 900.0 }, 0.7, None, Color::White)?;

            for wall in self.walls.iter() {
                if player.collide(wall) {
                    player.move_to_prev_position();
                }

                wall.draw(render)?;
            }


            for door in self.doors.iter_mut() {
                let collided_enemies: Vec<&Enemy<'a>> = self.enemies.iter().filter(| enemy | enemy.collide(door)).collect();

                match door.get_door_type() {
                    &DoorType::Regular => {
                        if player.collide(door) || collided_enemies.len() > 0 {
                            door.open();
                        } else {
                            door.close();
                        }
                    },

                    &DoorType::Coded | &DoorType::Locked => {
                        if collided_enemies.len() > 0 {
                            door.open();
                        } else {
                            door.close();
                        }

                        if door.is_locked() {
                            if player.collide(door) {
                                if player.can_open_door(door) {
                                    door.unlock();
                                } else {
                                    player.move_to_prev_position();
                                }
                            }
                        } else {
                            if player.collide(door) {
                                door.open();
                            } else {
                                door.close();
                            }
                        }
                    },

                    _ => ()
                }

                door.draw(render)?;
            }

            for teleport_door in self.teleport_doors.iter() {
                let want_to_teleport_enemies: Vec<&mut Enemy<'a>> = self.enemies.iter_mut().filter(| enemy | {
                    if enemy.get_want_to_teleport_id().is_none() {
                        return false;
                    }

                    !enemy.get_is_teleported() && enemy.get_want_to_teleport_id().unwrap() == teleport_door.get_id() && enemy.collide(teleport_door)
                }).collect();

                if want_to_teleport_enemies.len() > 0 {
                    for enemy in want_to_teleport_enemies {
                        teleport_door.teleport(enemy);

                        enemy.set_is_teleported(true);
                        enemy.set_want_to_teleport_id(None);
                        enemy.set_move_to_teleport_id(Some(teleport_door.get_move_to_id()));

                        if enemy.get_mode() != &EnemyMode::Regular {
                            if enemy.get_should_attach_teleport_door() {
                                enemy.attach_teleport_door(teleport_door.get_id(), teleport_door.get_move_to_id(), teleport_door.get_character_move_position());
                            } else {
                                enemy.set_should_attach_teleport_door(true);
                            }
                        }
                    }
                }           

                if player.is_colliding_with_object(teleport_door) {
                    if let Some(player_interaction) = player.get_interaction() {
                        if !player.get_is_teleported() && player_interaction.key() == &Key::Space && player_interaction.action() == &Action::Press {
                            teleport_door.teleport(player);
                            player.set_is_teleported(true);
                        }
                    }
                }

                teleport_door.draw(render)?;
            }

            for enemy in self.enemies.iter_mut() {
                if enemy.get_is_teleported() {
                    enemy.set_is_teleported(false);
                }
            }

            for door_collectable in self.door_collectables.iter_mut() {
                if !door_collectable.is_collected() && player.collide(door_collectable) {
                    player.add_door_collectable(door_collectable.get_door_collectable_type(), door_collectable.get_id(), door_collectable.opens());

                    door_collectable.set_is_collected(true);
                }

                door_collectable.draw(render)?;
            }

            for coin in self.coins.iter_mut() {
                if !coin.is_collected() && player.collide(coin) {
                    player.add_coin();

                    coin.set_is_collected(true);
                }


                coin.draw(render)?;
            }

            assert!(self.exit_door != None, "exit_door must not be null");

            let exit_door = self.exit_door.as_ref().unwrap();
            exit_door.draw(render)?;

            if player.get_status() == &PlayerStatus::NotHidden && player.collide(exit_door) {
                if let Some(player_interaction) = player.get_interaction() {
                    if player_interaction.key() == &Key::Space && player_interaction.action() == &Action::Press {
                        self.status = LevelStatus::Win;

                        for challenge in self.challenges.iter_mut() {
                            if challenge.get_status() == &ChallengeStatus::NotDetermine {
                                challenge.set_status(challenge.check_challenge(player, true, self.notoriety_level));
                            }

                            if challenge.get_status() == &ChallengeStatus::Completed {
                                player.add_to_coins(challenge.get_reward() as u32);
                            }
                        }
                    }
                }
            }

            for hide_place in self.hide_places.iter() {
                if player.is_colliding_with_object(hide_place) {
                    if let Some(player_interaction) = player.get_interaction() {
                        let player_status = player.get_status();

                        if (!player.get_is_detected_by_enemy() || (player.get_is_detected_by_enemy() && !player.get_is_seen_by_enemy()) || player_status != &PlayerStatus::Detectit) && player_interaction.key() == &Key::Space && player_interaction.action() == &Action::Press {
                            if player.get_status() == &PlayerStatus::Hidden {
                                player.set_status(PlayerStatus::NotHidden);
                            } else {
                                player.set_status(PlayerStatus::Hidden);
                            }
                        }
                    }
                }

                hide_place.draw(render)?;
            }

            for camera in self.cameras.iter_mut() {
                camera.draw(render)?;

                let (
                    new_notoriety_level,
                    enemy_id,
                    detected_player_position
                ) = camera.detect_player(
                    self.notoriety_level,
                    player,
                    &self.walls,
                    &self.doors,
                    &self.enemies
                );
                self.notoriety_level = new_notoriety_level;

                camera.set_new_repeat_interval(self.notoriety_level);

                if let Some(enemy_id) = enemy_id {
                    self.attached_enemies_ids.push((enemy_id, detected_player_position.unwrap(), AttachedType::CameraDetect));
                }
            }

            for _ in 0..self.detecting_ranges.len() {
                let detect_range = self.detecting_ranges.remove(0);

                let mut enemies_in_detect_area = Vec::new();

                for enemy in &self.enemies {
                    if !enemy.get_is_searching_detect_area() && detect_range.is_in_range(enemy) {
                        enemies_in_detect_area.push(enemy);
                    }
                }

                if enemies_in_detect_area.len() > 0 {
                    let mut start_position = round_position_to_full_numbers(detect_range.get_center_position(), DEFAULT_MOVEMENT_VALUE, true, true);

                    let enemy = enemies_in_detect_area[0];
                    let movement_grid = enemy.get_grid().as_ref().unwrap();

                    start_position = get_correct_start_position(start_position, movement_grid, enemy.get_movement_value());

                    let nearest_enemy_id = get_nearest_enemy_id(start_position, &enemies_in_detect_area);

                    if nearest_enemy_id != -1 {
                        self.attached_enemies_ids.push((nearest_enemy_id as usize, start_position, AttachedType::DetectRangeSearch));
                        player.add_to_enemies_trick_count(1);
                    }
                }
            }

            for enemy in self.enemies.iter_mut() {
                let idx = get_attached_enemy_index(&self.attached_enemies_ids, enemy.get_id());
                if idx != -1 {
                    let (.., attached_position, attached_type) = &self.attached_enemies_ids[idx as usize];

                    match attached_type {
                        AttachedType::CameraDetect => {
                            enemy.attach_camera(*attached_position);
                        },

                        AttachedType::DetectRangeSearch => {
                            enemy.search(SearchingMode::TrickCanSearch, *attached_position);
                        }
                    }

                    self.attached_enemies_ids.remove(idx as usize);
                }

                let mut is_enemy_colliding_with_a_wall = false;

                for wall in self.walls.iter() {
                    if enemy.collide(wall) {
                        is_enemy_colliding_with_a_wall = true;

                        break;
                    }
                }

                if enemy.is_off_window(render.get_size())
                || enemy.is_off_border(
                    Some(self.border_top_left + DEFAULT_SIZE),
                    Size { width: self.border_size.width - (DEFAULT_SIZE * 2.0), height: self.border_size.height - (DEFAULT_SIZE * 2.0) }
                ) || is_enemy_colliding_with_a_wall {
                    enemy.set_is_colliding(true);
                    enemy.move_to_prev_position();
                } else {
                    enemy.set_is_colliding(false);
                }
                
                if enemy.collide_with_player(&player) && (self.status != LevelStatus::ReLoadLevel || self.status != LevelStatus::Lose) { 
                    player.decrease_life();

                    if player.get_lifes() == 0 {
                        self.status = LevelStatus::Lose;
                    } else {
                        self.status = LevelStatus::ReLoadLevel;
                    }
                }

                if player.get_is_using_ability() {
                    let center = Position {
                        x: player.get_position().x + player.get_size().width / 2.0,
                        y: player.get_position().y + player.get_size().height / 2.0,
                    };

                    let is_in = is_in_circle(center, player.get_ability_radius(), enemy);

                    enemy.set_draw_detect_traingle(is_in);

                    if is_in && player.get_track_path_ability() {
                        enemy.set_draw_move_path(true);
                    } else {
                        enemy.set_draw_move_path(false);
                    }
                } else {
                    enemy.set_draw_detect_traingle(false);
                    enemy.set_draw_move_path(false);
                }

                enemy.draw(render)?;

                self.notoriety_level = enemy.move_enemy(
                    player, 
                    self.notoriety_level, 
                    self.border_top_left + DEFAULT_SIZE, 
                    Size { width: self.border_size.width - (DEFAULT_SIZE * 2.0), height: self.border_size.height - (DEFAULT_SIZE * 2.0) },
                    &self.walls,
                    &self.doors,
                    &self.teleport_doors,
                    &self.hide_places
                );
            }

            if player.get_is_teleported() {
                player.set_is_teleported(false);
            }

            player.set_notoriety_camera_disturb_lifttime(self.notoriety_level);
            let shooted_object = player.shoot(
                self.border_top_left + DEFAULT_SIZE, 
                Size { width: self.border_size.width - (DEFAULT_SIZE * 2.0), height: self.border_size.height - (DEFAULT_SIZE * 2.0) },
                render
            );
            if let Some(object) = shooted_object {
                match object {
                    ShootObject::Can(can) => { self.cans.add(can).unwrap(); },
                    ShootObject::Bullet(bullet) => { self.bullets.add(bullet).unwrap(); },
                }
            }

            for _ in 0..self.bullets.size() {
                let mut bullet = self.bullets.remove().unwrap();

                if !bullet.get_is_finished() {
                    let mut is_object_colliding = bullet.is_off_border(Some(self.get_boder_start_position()), self.get_boder_size());

                    if !is_object_colliding {
                        for camera in self.cameras.iter_mut() {
                            if bullet.collide_with_camera(camera) {
                                if bullet.get_bullet_type() == &BulletType::CameraGunBullet && !camera.get_is_disturbed() {
                                    camera.set_is_disturbed(true, Some(player.get_notoriety_camera_disturb_lifttime()));
                                    player.add_to_disturb_cameras_count(1);
                                } else if bullet.get_bullet_type() == &BulletType::Other {
                                    camera.destroy();
                                } 

                                is_object_colliding = true;
                            }
                        }
                    }

                    if !is_object_colliding {
                        for wall in self.walls.iter() {
                            if bullet.collide(wall) {
                                is_object_colliding = true;

                                break;
                            }
                        }
                    }

                    if !is_object_colliding {
                        for door in self.doors.iter() {
                            if bullet.collide(door) && door.is_closed() {
                                is_object_colliding = true;

                                break;
                            }
                        }
                    }

                    if !is_object_colliding {
                        for enemy in self.enemies.iter_mut() {
                            if bullet.collide(enemy) {
                                if enemy.get_mode() == &EnemyMode::Regular || enemy.is_search_mode() {
                                    // TODO: see if want to implement this in game
                                    // enemy.damage(bullet.get_damage_on_enemy());
                                    //
                                    // if !enemy.get_is_dead() {
                                    //     enemy.search(SearchingMode::BulletSearch, bullet.get_start_position());
                                    // } else {
                                    //     player.add_to_enemies_killed_count(1);
                                    // }

                                    enemy.search(SearchingMode::BulletSearch, bullet.get_start_position());
                                }

                                is_object_colliding = true;
                            }
                        }
                    }


                    if is_object_colliding {
                        bullet.set_is_finished(true);
                    }
                }

                bullet.draw(render)?;

                if !bullet.get_is_finished() {
                    bullet.calc_next_position();

                    self.bullets.add(bullet).unwrap();
                }
            }

            for _ in 0..self.cans.size() {
                let mut can = self.cans.remove().unwrap(); 

                if !can.get_is_finished() {
                    let mut is_object_colliding = can.is_off_border(Some(self.get_boder_start_position()), self.get_boder_size());

                    for wall in self.walls.iter() {
                        if can.collide(wall) {
                            is_object_colliding = true;

                            break;
                        }
                    }

                    if !is_object_colliding {
                        for door in self.doors.iter() {
                            if can.collide(door) && door.is_closed() {
                                is_object_colliding = true;

                                break;
                            }
                        }
                    }

                    if is_object_colliding {
                        can.set_is_finished(true);
                    }
                }

                can.draw(render)?;

                if !can.get_is_finished() {
                    can.calc_next_position();
                } else {
                    if !can.get_added_detect_range() {
                        self.detecting_ranges.push(DetectRange::new(player.get_can_detecting_radius(), can.get_calc_position().0));

                        can.set_added_detect_range(true);
                    }
                }

                if !can.get_done() {
                    self.cans.add(can).unwrap();
                }
            }

            player.draw(render)?;
            player.switch_items();
        }

        Ok(())
    }
    
    pub fn get_status(&self) -> &LevelStatus {
        &self.status
    }

    pub fn get_is_paused(&self) -> bool {
        self.is_paused
    }

    pub fn set_is_paused(&mut self, new_val: bool) {
        self.is_paused = new_val;
    }

    fn set_initial_object_position(&mut self, object: &mut impl GameObject<'a>) {
        let start_position = self.get_boder_start_position();

        let object_position = object.get_position();
        object.set_position(object_position + start_position);
    }

    fn insert_enemy(&mut self, mut enemy: Enemy<'a>) {
        let start_position = self.get_boder_start_position();

        let enemy_start_position = enemy.get_start_position();
        enemy._set_start_position(enemy_start_position + start_position);
        
        self.set_initial_object_position(&mut enemy);

        self.enemies.push(enemy);
    }


    fn insert_wall(&mut self, mut wall: Wall<'a>) {
        if self.status == LevelStatus::ReLoadLevel {
            return;
        }

        self.set_initial_object_position(&mut wall);

        self.walls.push(wall);
    }

    fn insert_door(&mut self, mut door: Door<'a>) {
        if self.status == LevelStatus::ReLoadLevel {
            return;
        }

        self.set_initial_object_position(&mut door);

        self.doors.push(door);
    }

    fn insert_door_collectable(&mut self, mut door_collectable: DoorCollectable<'a>) {
        if self.status == LevelStatus::ReLoadLevel {
            return;
        }

        self.set_initial_object_position(&mut door_collectable);

        self.door_collectables.push(door_collectable);
    }

    fn insert_teleport_door(&mut self, mut teleport_door: TeleportDoor<'a>) {
        if self.status == LevelStatus::ReLoadLevel {
            return;
        }

        self.set_initial_object_position(&mut teleport_door);

        self.teleport_doors.push(teleport_door);
    }

    fn insert_exit_door(&mut self, mut exit_door: ExitDoor<'a>) {
        if self.status == LevelStatus::ReLoadLevel {
            return;
        }

        self.set_initial_object_position(&mut exit_door);

        self.exit_door = Some(exit_door);
    }

    fn insert_hide_place(&mut self, mut hide_place: HidePlace<'a>) {
        if self.status == LevelStatus::ReLoadLevel {
            return;
        }

        self.set_initial_object_position(&mut hide_place);

        self.hide_places.push(hide_place);
    }

    fn insert_coin(&mut self, mut coin: Coin<'a>) {
        if self.status == LevelStatus::ReLoadLevel {
            return;
        }

        self.set_initial_object_position(&mut coin);

        self.coins.push(coin);
    }

    fn insert_camera(&mut self, mut camera: Camera<'a>) {
        if self.status == LevelStatus::ReLoadLevel {
            return;
        }

        let start_position = self.get_boder_start_position();

        let camera_position = camera.get_position();
        camera.set_position(camera_position + start_position);

        self.cameras.push(camera);
    }

    pub fn set_level(&mut self, level: u8) {
        assert!(level >= 1 && level <= 5, "level must be between 1 to 5 (include)");

        self.current_level = level;
    }

    pub fn next_level(&mut self) {
        assert!(self.current_level < 5, "level must be between 1 to 5 (include)");
        
        self.current_level += 1;
    }

    pub fn clear_level_objects(&mut self) {
        if self.status != LevelStatus::Lose {
            self.challenges = get_level_challenges(self.current_level).expect("Unable to get level challenges");
        }

        self.enemies.clear();
        self.walls.clear();
        self.doors.clear();
        self.door_collectables.clear();
        self.teleport_doors.clear();
        self.hide_places.clear();
        self.coins.clear();
        self.cameras.clear();

        self.exit_door = None;
    }

    pub fn load_level(&mut self, player: &mut Player<'a>) -> std::result::Result<(), String> {
        if self.status == LevelStatus::ReLoadLevel {
            player.add_inventory_amounts(self.add_amount_after_lost);
        } else {
            self.notoriety_level = 0;

            self.clear_level_objects();

            if self.status == LevelStatus::Lose || self.status == LevelStatus::Win {
                if self.status == LevelStatus::Win {
                    player.reset_level_tries();
                } else {
                    player.add_to_level_tries(1);
                }

                player.reset_props_for_new_level();
            }
        }

        player.reset_props();

        if self.status == LevelStatus::ReLoadLevel {
            self.enemies.clear();
        }
        
        match self.current_level {
            1 => {
                player.move_to(Position { x: 90.0, y: 180.0 }, true);

                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 140.0, y: 10.0 }, "18d/0 13l/3000 13r/0 18u/0 6r/3000 6l/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 100.0, y: 295.0 }, "9l/6000 20r/3000 11l/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 405.0 }, "11d/0 15r/0 10d/0 5l/2000 9r/2000 21u/1000 19l/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 340.0, y: 30.0 }, "26d/3500 26u/3000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 380.0, y: 420.0 }, "25r/0 6u/2000 16d/2000 25l/0 9d/0 5l/2000 5r/0 19u/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 550.0, y: 257.0 }, "8r/5000 15l/3000 7r/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 470.0, y: 157.0 }, "16r/4000 16l/5000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 470.0, y: 5.0 }, "16r/3000 5d/0 16l/0 5u/5000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 730.0, y: 520.0 }, "18u/3000 4r/0 18d/0 4l/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 915.0, y: 90.0 }, "15d/2000 4l/0 15u/0 15l/0 15d/2000 4r/0 15u/0 15r/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 865.0, y: 335.0 }, "18d/3000 4r/0 18u/5000 4l/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 840.0, y: 0.0 }, "20r/6500 20l/6500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1050.0, y: 100.0 }, "18d/4000 18u/5000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1020.0, y: 520.0 }, "18u/6000 18d/0 3r/4000 3l/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1190.0, y: 520.0 }, "42u/6000 42d/0 4l/4500 4r/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 730.0, y: 615.0 }, "20r/6000 20l/4000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1030.0, y: 615.0 }, "15r/6000 15l/5500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1310.0, y: 615.0 }, "18u/6000 18d/0 2l/4500 2r/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1310.0, y: 90.0 }, "20d/6000 20u/0 2r/5500 2l/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1450.0, y: 90.0 }, "24d/6000 24u/0 2l/4500 2r/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1450.0, y: 615.0 }, "18u/5500 18d/0 2r/6000 2l/2500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1140.0, y: 0.0 }, "25r/4000 25l/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1490.0, y: 0.0 }, "21r/4000 21l/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1580.0, y: 504.0 }, "12r/0 1d/4000 42u/3000 12l/0 41d/0", false));

                self.insert_wall(Wall::new(Position { x: 80.0, y: 0.0 }, Size { width: DEFAULT_SIZE, height: 100.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 78.0, y: 100.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 0.0, y: 160.0 }, Size { width: 110.0, height: DEFAULT_SIZE }, None, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 262.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 265.0, y: 70.0 }, Size { width: DEFAULT_SIZE, height: 536.0 }, None, None));
                self.insert_door(Door::new(1, DoorType::Coded, Position { x: 250.0, y: 610.0 }, Size { width: 60.0, height: 70.0 }, true, Some(1), None, None)?);
                self.insert_hide_place(HidePlace::new(Position { x: 215.0, y: 110.0 }, None));

                self.insert_wall(Wall::new(Position { x: 0.0, y: 250.0 }, Size { width: 210.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 210.0, y: 250.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_hide_place(HidePlace::new(Position { x: 130.0, y: 185.0 }, None));
                self.insert_coin(Coin::new(Position { x: 10.0, y: 200.0 }, None));

                self.insert_wall(Wall::new(Position { x: 53.0, y: 360.0 }, Size { width: 213.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 0.0, y: 360.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_hide_place(HidePlace::new(Position { x: 150.0, y: 295.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 70.0, y: 295.0 }, None));

                self.insert_hide_place(HidePlace::new(Position { x: 60.0, y: 390.0 }, None));
                self.insert_coin(Coin::new(Position { x: 225.0, y: 400.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 215.0, y: 480.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 160.0, y: 615.0 }, None));

                self.insert_hide_place(HidePlace::new(Position { x: 50.0, y: 512.0 }, None));
                self.insert_wall(Wall::new(Position { x: 0.0, y: 577.0 }, Size { width: 100.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 62.0, y: 608.0 }, Size { width: 40.0, height: 70.0 }, false, None, None, None)?);
                self.insert_door_collectable(DoorCollectable::new(1, DoorCollectableType::CodePaper, Position { x: 5.0, y: 625.0 }, vec![1], None));

                self.insert_wall(Wall::new(Position { x: 505.0, y: 580.0 }, Size { width: DEFAULT_SIZE, height: 96.0 }, None, None));
                self.insert_wall(Wall::new(Position { x: 535.0, y: 580.0 }, Size { width: 100.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 635.0, y: 580.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 690.0, y: 580.0 }, Size { width: DEFAULT_SIZE, height: 96.0 }, None, None));
                self.insert_door_collectable(DoorCollectable::new(2, DoorCollectableType::Key, Position { x: 545.0, y: 630.0 }, vec![2, 3], None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 687.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 690.0, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 360.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 687.0, y: 420.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 690.0, y: 480.0 }, Size { width: DEFAULT_SIZE, height: 100.0 }, None, None));

                self.insert_wall(Wall::new(Position { x: 435.0, y: 0.0 }, Size { width: DEFAULT_SIZE, height: 350.0 }, None, None));
                self.insert_wall(Wall::new(Position { x: 465.0, y: 320.0 }, Size { width: 165.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(2, DoorType::Locked, Position { x: 620.0, y: 320.0 }, Size { width: 80.0, height: DEFAULT_SIZE }, true, Some(2), None, None)?);
                
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 465.0, y: 220.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 520.0, y: 220.0 }, Size { width: 170.0, height: DEFAULT_SIZE }, None, None));
                self.insert_hide_place(HidePlace::new(Position { x: 550.0, y: 255.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 470.0, y: 255.0 }, None));

                self.insert_wall(Wall::new(Position { x: 465.0, y: 120.0 }, Size { width: 170.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 635.0, y: 120.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_hide_place(HidePlace::new(Position { x: 550.0, y: 155.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 640.0, y: 155.0 }, None));

                self.insert_hide_place(HidePlace::new(Position { x: 564.0, y: 55.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 500.0, y: 55.0 }, None));
                self.insert_coin(Coin::new(Position { x: 520.0, y: 15.0 }, None));

                self.insert_hide_place(HidePlace::new(Position { x: 384.0, y: 50.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 300.0, y: 220.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 300.0, y: 402.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 375.0, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 550.0, y: 515.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 570.0, y: 350.0 }, None));
                self.insert_coin(Coin::new(Position { x: 480.0, y: 440.0 }, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 797.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_hide_place(HidePlace::new(Position { x: 930.0, y: 0.0 }, None));
                
                self.insert_wall(Wall::new(Position { x: 720.0, y: 60.0 }, Size { width: 335.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1055.0, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                
                self.insert_wall(Wall::new(Position { x: 1110.0, y: 0.0 }, Size { width: DEFAULT_SIZE, height: 520.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1107.0, y: 520.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_coin(Coin::new(Position { x: 1145.0, y: 15.0 }, None));

                self.insert_wall(Wall::new(Position { x: 1000.0, y: 580.0 }, Size { width: 280.0, height: DEFAULT_SIZE }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 1065.0, y: 130.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1000.0, y: 200.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1065.0, y: 320.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1000.0, y: 430.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1030.0, y: 515.0 }, None));

                self.insert_wall(Wall::new(Position { x: 1140.0, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1195.0, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);

                self.insert_wall(Wall::new(Position { x: 1250.0, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 520.0 }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 1175.0, y: 515.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1205.0, y: 370.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1140.0, y: 250.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1205.0, y: 120.0 }, None));

                self.insert_wall(Wall::new(Position { x: 1280.0, y: 60.0 }, Size { width: 425.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1705.0, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 1285.0, y: 0.0 }, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1447.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                
                self.insert_hide_place(HidePlace::new(Position { x: 1575.0, y: 0.0 }, None));

                self.insert_hide_place(HidePlace::new(Position { x: 1640.0, y: 90.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1714.0, y: 230.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1560.0, y: 350.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1630.0, y: 514.0 }, None));
                self.insert_coin(Coin::new(Position { x: 1640.0, y: 300.0 }, None));

                self.insert_wall(Wall::new(Position { x: 720.0, y: 580.0 }, Size { width: 110.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 827.0, y: 90.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 830.0, y: 150.0 }, Size { width: DEFAULT_SIZE, height: 460.0 }, None, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 720.0, y: 300.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 780.0, y: 300.0 }, Size { width: 50.0, height: DEFAULT_SIZE }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 785.0, y: 400.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 720.0, y: 510.0 }, None));
                self.insert_coin(Coin::new(Position { x: 755.0, y: 520.0 }, None));

                self.insert_hide_place(HidePlace::new(Position { x: 785.0, y: 235.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 720.0, y: 110.0 }, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 860.0, y: 580.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 915.0, y: 580.0 }, Size { width: 80.0, height: DEFAULT_SIZE }, None, None));
                self.insert_wall(Wall::new(Position { x: 970.0, y: 90.0 }, Size { width: DEFAULT_SIZE, height: 490.0 }, None, None));

                self.insert_wall(Wall::new(Position { x: 860.0, y: 300.0 }, Size { width: 50.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 915.0, y: 300.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 925.0, y: 90.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 860.0, y: 235.0 }, None));

                self.insert_hide_place(HidePlace::new(Position { x: 925.0, y: 360.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 860.0, y: 430.0 }, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 990.0, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);
                self.insert_hide_place(HidePlace::new(Position { x: 900.0, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 845.0, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 780.0, y: 615.0 }, None));
                self.insert_coin(Coin::new(Position { x: 730.0, y: 625.0 }, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1247.0, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);
                self.insert_hide_place(HidePlace::new(Position { x: 1100.0, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1190.0, y: 615.0 }, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1387.0, y: 90.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 1390.0, y: 150.0 }, Size { width: DEFAULT_SIZE, height: 527.0 }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 1310.0, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1290.0, y: 510.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1344.0, y: 450.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1280.0, y: 330.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1344.0, y: 210.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1280.0, y: 120.0 }, None));
                self.insert_coin(Coin::new(Position { x: 1340.0, y: 360.0 }, None));

                self.insert_wall(Wall::new(Position { x: 1530.0, y: 90.0 }, Size { width: DEFAULT_SIZE, height: 517.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1527.0, y: 607.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 1485.0, y: 100.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1420.0, y: 250.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1485.0, y: 400.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1420.0, y: 520.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1450.0, y: 615.0 }, None));
                self.insert_coin(Coin::new(Position { x: 1430.0, y: 360.0 }, None));

                self.insert_wall(Wall::new(Position { x: 1560.0, y: 577.0 }, Size { width: 145.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(3, DoorType::Locked, Position { x: 1697.0, y: 575.0 }, Size { width: 70.0, height: DEFAULT_SIZE + 2.0 }, true, Some(2), None, None)?);
                self.insert_exit_door(ExitDoor::new(Position { x: 1697.0, y: 620.0 }, None));
                self.insert_coin(Coin::new(Position { x: 1600.0, y: 630.0 }, None));
            },

            2 => {
                player.move_to(Position { x: 1780.0, y: 790.0 }, false);

                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1070.0, y: 620.0 }, "52r/3500 52l/4000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 960.0, y: 620.0 }, "11u/0 21r/5500 21l/0 10d/0 1r/6000 1l/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1270.0, y: 240.0 }, "11l/7000 2r/1000 17d/4000 9r/0 17u/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 960.0, y: 240.0 }, "10r/4500 8d/6500 10l/0 8u/2500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 840.0, y: 615.0 }, "19u/0 2r/6000 2l/0 19d/0 2l/5500 2r/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 615.0 }, "35r/5000 35l/5000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 513.0 }, "21r/4500 21l/5500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 0.0 }, "4r/4000 4l/0 23d/6000 23u/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 205.0, y: 0.0 }, "5l/4500 14d/0 5r/5500 14u/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 550.0, y: 330.0 }, "30r/4500 30l/5500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 685.0, y: 240.0 }, "16r/4000 16l/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 475.0, y: 615.0 }, "6r/0 19u/4000 16r/0 19d/0 2r/3500 24l/4000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 5.0, y: 345.0 }, "36r/2500 7d/0 36l/0 7u/4000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 335.0, y: 0.0 }, "29d/0 12r/0 4u/3500 4d/0 12l/0 29u/0 2r/3500 2l/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 475.0, y: 150.0 }, "38r/3500 38l/5500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 960.0, y: 150.0 }, "30r/4500 30l/6500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 475.0, y: 0.0 }, "3d/0 73r/7500 73l/0 3u/0 1l/6500 1r/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1480.0, y: 517.0 }, "21r/6500 21l/5500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1372.0, y: 150.0 }, "1r/0 27d/0 1r/4000 1l/0 27u/0 1l/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1525.0, y: 160.0 }, "26d/0 3l/6000 3r/0 26u/4000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1360.0, y: 0.0 }, "17r/6500 6d/5500 17l/0 6u/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1705.0, y: 220.0 }, "8l/0 18d/0 8r/0 1d/6500 19u/4500", false));
                
                self.insert_wall(Wall::new(Position { x: 1360.0, y: 577.0 }, Size { width: 400.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1650.0, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 1330.0, y: 510.0 }, Size { width: DEFAULT_SIZE, height: 97.0 }, None, None));
                self.insert_wall(Wall::new(Position { x: 1230.0, y: 480.0 }, Size { width: 130.0, height: DEFAULT_SIZE }, None, None));
                self.insert_wall(Wall::new(Position { x: 1230.0, y: 510.0 }, Size { width: DEFAULT_SIZE, height: 97.0 }, None, None));
                self.insert_wall(Wall::new(Position { x: 1030.0, y: 577.0 }, Size { width: 200.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1027.0, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 1220.0, y: 620.0 }, None));
                self.insert_coin(Coin::new(Position { x: 1275.0, y: 520.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1450.0, y: 582.0 }, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1110.0, y: 582.0 }, false, None, None));

                self.insert_wall(Wall::new(Position { x: 920.0, y: 480.0 }, Size { width: DEFAULT_SIZE, height: 197.0 }, None, None));
                self.insert_wall(Wall::new(Position { x: 950.0, y: 480.0 }, Size { width: 225.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1175.0, y: 480.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 1330.0, y: 210.0 }, Size { width: DEFAULT_SIZE, height: 270.0 }, None, None));
                self.insert_wall(Wall::new(Position { x: 920.0, y: 210.0 }, Size { width: 410.0, height: DEFAULT_SIZE }, None, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1117.0, y: 240.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 1120.0, y: 300.0 }, Size { width: DEFAULT_SIZE, height: 180.0 }, None, None));

                self.insert_wall(Wall::new(Position { x: 920.0, y: 240.0 }, Size { width: DEFAULT_SIZE, height: 180.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 917.0, y: 420.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 950.0, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1050.0, y: 515.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1110.0, y: 485.0 }, true, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 1240.0, y: 415.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1290.0, y: 300.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1180.0, y: 240.0 }, None));
                self.insert_coin(Coin::new(Position { x: 1200.0, y: 340.0 }, None));

                self.insert_wall(Wall::new(Position { x: 950.0, y: 390.0 }, Size { width: 115.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0,DoorType::Regular, Position { x: 1065.0, y: 390.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 1050.0, y: 240.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 970.0, y: 240.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 980.0, y: 327.0 }, None));
                self.insert_coin(Coin::new(Position { x: 1050.0, y: 320.0 }, None));

                self.insert_hide_place(HidePlace::new(Position { x: 1030.0, y: 422.0 }, Some(0.98)));
                self.insert_camera(Camera::new_with_repeat(Position { x: 985.0, y: 395.0 }, false, None, None, Some(6000)));

                self.insert_wall(Wall::new(Position { x: 605.0, y: 390.0 }, Size { width: 315.0, height: DEFAULT_SIZE }, None, None));
                self.insert_wall(Wall::new(Position { x: 780.0, y: 420.0 }, Size { width: DEFAULT_SIZE, height: 187.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 777.0, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 840.0, y: 417.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 880.0, y: 530.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 795.0, y: 550.0 }, true, None, Some(350.0)));

                self.insert_teleport_door(TeleportDoor::new(1, Position { x: 475.0, y: 615.0 }, Position { x: 560.0, y: 790.0 }, 1, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 0.0, y: 577.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 65.0, y: 577.0 }, Size { width: 360.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 423.0, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 425.0, y: 477.0 }, Size { width: DEFAULT_SIZE, height: 130.0 }, None, None));
                self.insert_wall(Wall::new(Position { x: 0.0, y: 477.0 }, Size { width: 425.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 280.0, y: 507.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);
                self.insert_teleport_door(TeleportDoor::new(2, Position { x: 375.0, y: 510.0 }, Position { x: 560.0, y: 790.0 }, 1, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 330.0, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 85.0, y: 615.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 230.0, y: 582.0 }, false, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 120.0, y: 515.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 230.0, y: 482.0 }, true, None, None));
                self.insert_door_collectable(DoorCollectable::new(1, DoorCollectableType::CodePaper, Position { x: 323.0, y: 532.0 }, vec![1, 5], None));

                self.insert_teleport_door(TeleportDoor::new(3, Position { x: 5.0, y: 335.0 }, Position { x: 90.0, y: 510.0 }, 3, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 0.0, y: 300.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 65.0, y: 300.0 }, Size { width: 230.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 112.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 115.0, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 240.0 }, None, None));
                self.insert_wall(Wall::new(Position { x: 265.0, y: 0.0 }, Size { width: DEFAULT_SIZE, height: 300.0 }, None, None));
                self.insert_wall(Wall::new(Position { x: 145.0, y: 200.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 200.0, y: 200.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_teleport_door(TeleportDoor::new(4, Position { x: 150.0, y: 232.0 }, Position { x: 90.0, y: 510.0 }, 3, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 70.0, y: 235.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 0.0, y: 130.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 35.0, y: 0.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: -30.0, y: 80.0 }, true, None, Some(80.0)));

                self.insert_hide_place(HidePlace::new(Position { x: 220.0, y: 0.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 220.0, y: 70.0 }, None));
                self.insert_door_collectable(DoorCollectable::new(2, DoorCollectableType::Key, Position { x: 220.0, y: 250.0 }, vec![2, 3], None));

                self.insert_door(Door::new(2, DoorType::Locked, Position { x: 417.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 15.0, height: 60.0 }, true, Some(2), None, None)?);
                self.insert_wall(Wall::new(Position { x: 425.0, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 180.0 }, None, None));
                self.insert_door(Door::new(1, DoorType::Coded, Position { x: 438.0, y: 210.0 }, Size { width: 85.0, height: DEFAULT_SIZE }, true, Some(1), None, None)?);
                self.insert_wall(Wall::new(Position { x: 510.0, y: 210.0 }, Size { width: 410.0, height: DEFAULT_SIZE }, None, None));

                self.insert_wall(Wall::new(Position { x: 510.0, y: 240.0 }, Size { width: DEFAULT_SIZE, height: 180.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 540.0, y: 390.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_teleport_door(TeleportDoor::new(5, Position { x: 545.0, y: 420.0 }, Position { x: 620.0, y: 600.0 }, 5, None, None));
                
                self.insert_wall(Wall::new(Position { x: 540.0, y: 300.0 }, Size { width: 325.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 865.0, y: 300.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 630.0, y: 328.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 815.0, y: 328.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 695.0, y: 305.0 }, true, None, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 640.0, y: 240.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_teleport_door(TeleportDoor::new(6, Position { x: 545.0, y: 235.0 }, Position { x: 620.0, y: 600.0 }, 5, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 787.5, y: 238.0 }, None));
                self.insert_door_collectable(DoorCollectable::new(3, DoorCollectableType::Key, Position { x: 600.0, y: 255.0 }, vec![4], None));

                self.insert_hide_place(HidePlace::new(Position { x: 670.0, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 545.0, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 738.0, y: 500.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 625.0, y: 417.0 }, None));
                self.insert_coin(Coin::new(Position { x: 730.0, y: 430.0 }, None));

                self.insert_hide_place(HidePlace::new(Position { x: 450.0, y: 500.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 340.0, y: 415.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 160.0, y: 415.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 85.0, y: 327.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 470.0, y: 300.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 290.0, y: 220.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 385.0, y: 110.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 335.0, y: 0.0 }, None));
                self.insert_coin(Coin::new(Position { x: 5.0, y: 435.0 }, None));
                self.insert_coin(Coin::new(Position { x: 345.0, y: 340.0 },None));

                for num in 0..4 {
                    self.insert_wall(Wall::new(Position { x: 455.0 + (num as f32 * 226.25), y: 90.0 }, Size { width: 226.25, height: DEFAULT_SIZE }, None, None));
                }

                self.insert_hide_place(HidePlace::new(Position { x: 540.0, y: 147.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 800.0, y: 147.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 607.5, y: 95.0 }, true, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 858.5, y: 95.0 }, true, None, None));

                self.insert_wall(Wall::new(Position { x: 920.0, y: 120.0 }, Size { width: DEFAULT_SIZE, height: 30.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 917.0, y: 150.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 980.0, y: 147.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1110.0, y: 147.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1240.0, y: 147.0 }, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1110.0, y: 95.0 }, false, None, None, Some(4000)));
                self.insert_coin(Coin::new(Position { x: 1180.0, y: 160.0 }, None));
                
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1327.0, y: 150.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 475.0, y: 27.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 610.0, y: 27.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 750.0, y: 27.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1002.0, y: 27.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1140.0, y: 27.0 }, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 610.0, y: -25.0 }, false, None, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 812.5, y: -25.0 }, true, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1005.0, y: -25.0 }, false, None, None, Some(4000)));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1207.5, y: -25.0 }, true, None, None));
                self.insert_coin(Coin::new(Position { x: 940.0, y: 40.0 }, None));

                self.insert_teleport_door(TeleportDoor::new(7, Position { x: 1270.0, y: 23.0 }, Position{ x: 1450.0, y: 690.0 }, 8, None, None));
                self.insert_wall(Wall::new(Position { x: 1330.0, y: 0.0 }, Size { width: DEFAULT_SIZE, height: 150.0 }, None, None));

                self.insert_wall(Wall::new(Position { x: 1360.0, y: 480.0 }, Size { width: 345.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1705.0, y: 480.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);

                self.insert_teleport_door(TeleportDoor::new(8, Position { x: 1370.0, y: 510.0 }, Position { x: 1350.0, y: 200.0 }, 7, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1430.0, y: 510.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 67.0 }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 1470.0, y: 515.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1630.0, y: 515.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1510.0, y: 485.0 }, true, None, None));
                self.insert_coin(Coin::new(Position { x: 1560.0, y: 527.0 }, None));

                self.insert_wall(Wall::new(Position { x: 1360.0, y: 120.0 }, Size { width: 175.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1535.0, y: 120.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_door(Door::new(5, DoorType::Coded, Position { x: 1580.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 18.0, height: 60.0 }, true, Some(1), None, None)?);
                self.insert_wall(Wall::new(Position { x: 1590.0, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 420.0 }, None, None));
                self.insert_wall(Wall::new(Position { x: 1620.0, y: 180.0 }, Size { width: 85.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(3, DoorType::Locked, Position { x: 1697.0, y: 180.0 }, Size { width: 70.0, height: DEFAULT_SIZE }, true, Some(2), None, None)?);

                self.insert_wall(Wall::new(Position { x: 1460.0, y: 150.0 }, Size { width: DEFAULT_SIZE, height: 270.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1457.0, y: 420.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 1415.0, y: 150.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1360.0, y: 275.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1360.0, y: 415.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1440.0, y: 380.0 }, false, None, Some(25.0)));

                self.insert_hide_place(HidePlace::new(Position { x: 1490.0, y: 150.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1545.0, y: 275.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1520.0, y: 415.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1460.0, y: 250.0 }, true, None, Some(80.0)));

                self.insert_hide_place(HidePlace::new(Position { x: 1470.0, y: 55.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1360.0, y: 0.0 }, None));
                self.insert_coin(Coin::new(Position { x: 1365.0, y: 75.0 }, None));

                self.insert_hide_place(HidePlace::new(Position { x: 1650.0, y: 415.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1715.0, y: 315.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1640.0, y: 210.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1590.0, y: 320.0 }, true, None, Some(80.0)));

                self.insert_door(Door::new(4, DoorType::Locked, Position { x: 1613.0, y: 63.0 }, Size { width: 70.0, height: DEFAULT_SIZE }, true, Some(3), None, None)?);
                self.insert_wall(Wall::new(Position { x: 1677.0, y: 63.0 }, Size { width: DEFAULT_SIZE - 10.0, height: 117.0 }, None, None));
                self.insert_exit_door(ExitDoor::new(Position { x: 1700.0, y: 0.0 }, None));

                self.insert_coin(Coin::new(Position { x: 1627.0, y: 95.0 }, None));
                self.insert_coin(Coin::new(Position { x: 1627.0, y: 115.0 }, None));
                self.insert_coin(Coin::new(Position { x: 1627.0, y: 135.0 }, None));
            },

            3 => {
                player.move_to(Position { x: 1790.0, y: 170.0 }, false);

                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1525.0, y: 0.0 }, "13d/0 15r/0 1d/3500 1u/0 15l/0 13u/0 5r/6000 5l/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1680.0, y: 480.0 }, "25u/5500 25d/0 2l/3500 2r/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 580.0, y: 0.0 }, "13d/0 6r/3500 20r/0 13u/0 2l/3000 3r/3000 1l/0 13d/0 6l/3500 20l/0 13u/0 2r/3000 3l/3000 1r/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 0.0 }, "45r/2500 45l/5500", true));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 342.0, y: 130.0 }, "33l/0 3u/3000 3d/0 33r/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 950.0, y: 0.0 }, "3r/0 13d/0 2r/6500 2l/0 13u/0 3l/4500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1145.0, y: 0.0 }, "3l/0 13d/0 2l/6000 2r/0 13u/0 3r/5000", true));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1250.0, y: 0.0 }, "18r/0 13d/0 8l/4000 8r/0 13u/0 18l/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1400.0, y: 310.0 }, "1r/0 31d/0 1r/4500 1l/0 30u/0 1l/6500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1295.0, y: 310.0 }, "1l/0 31d/0 1l/4500 1r/0 31u/0 1r/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1074.0, y: 220.0 }, "10r/3500 20d/5500 10l/0 20u/0", true));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 970.0, y: 615.0 }, "20r/4500 20l/6500", true));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 970.0, y: 220.0 }, "29d/4500 29u/0 1l/6000 1r/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 835.0, y: 310.0 }, "2r/0 20d/7000 20u/0 2l/4500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 729.0, y: 310.0 }, "2l/0 20d/5500 20u/0 2r/5000", true));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 130.0, y: 220.0 }, "46r/3500 46l/5500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 604.0, y: 320.0 }, "20d/0 2l/4000 2r/0 20u/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 421.0, y: 520.0 }, "6r/4500 19u/3000 5l/2000 19d/0 1l/5000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 330.0, y: 615.0 }, "35r/4000 35l/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 221.0, y: 310.0 }, "9r/0 11d/3500 8l/0 11u/0 1l/5500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 110.0, y: 310.0 }, "10l/0 20d/5500 9r/0 20u/0 1r/3500", false));

                self.insert_wall(Wall::new(Position { x: 1639.0, y: 60.0 }, Size { width: 120.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1636.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);

                self.insert_wall(Wall::new(Position { x: 1489.0, y: 0.0 }, Size { width: DEFAULT_SIZE, height: 190.0 }, None, None));
                self.insert_wall(Wall::new(Position { x: 1489.0, y: 190.0 }, Size { width: 160.0, height: DEFAULT_SIZE }, None, None));
                self.insert_wall(Wall::new(Position { x: 1619.0, y: 220.0 }, Size { width: DEFAULT_SIZE, height: 230.0 }, None, None));
                self.insert_wall(Wall::new(Position { x: 1519.0, y: 450.0 }, Size { width: 130.0, height: DEFAULT_SIZE }, None, None));
                self.insert_wall(Wall::new(Position { x: 1489.0, y: 450.0 }, Size { width: DEFAULT_SIZE, height: 120.0 }, None, None));
                self.insert_wall(Wall::new(Position { x: 1519.0, y: 540.0 }, Size { width: 240.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1616.0, y: 480.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_teleport_door(TeleportDoor::new(1, Position { x: 1530.0, y: 475.0 }, Position { x: 790.0, y: 170.0 }, 2, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 1525.0, y: 0.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1574.0, y: 128.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1714.0, y: 210.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1649.0, y: 325.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1679.0, y: 478.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1709.0, y: 100.0 }, None));

                self.insert_wall(Wall::new(Position { x: 660.0, y: 60.0 }, Size { width: 150.0, height: DEFAULT_SIZE }, None, None));
                self.insert_teleport_door(TeleportDoor::new(2, Position { x: 710.0, y: -5.0 }, Position { x: 1610.0, y: 650.0 }, 1, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 656.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 777.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);

                for num in 0..5 {
                    self.insert_wall(Wall::new(Position { x: 0.0 + (num as f32 * 297.8), y: 190.0 }, Size { width: 297.8, height: DEFAULT_SIZE }, None, None));
                }

                self.insert_hide_place(HidePlace::new(Position { x: 840.0, y: 0.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 865.0, y: 125.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 585.0, y: 0.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 550.0, y: 125.0 }, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 715.0, y: 65.0 }, false, None, None, Some(4000)));
                self.insert_coin(Coin::new(Position { x: 695.0, y: 120.0 }, None));
                self.insert_coin(Coin::new(Position { x: 735.0, y: 120.0 }, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 907.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 910.0, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 130.0 }, None, None));
                
                self.insert_hide_place(HidePlace::new(Position { x: 980.0, y: 0.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 940.0, y: 125.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1035.0, y: 80.0 }, false, None, Some(45.0)));

                self.insert_wall(Wall::new(Position { x: 1060.0, y: 0.0 }, Size { width: DEFAULT_SIZE, height: 130.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1057.0, y: 130.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                
                self.insert_hide_place(HidePlace::new(Position { x: 1140.0, y: 125.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1090.0, y: 0.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1150.0, y: -25.0 }, true, None, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1207.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 1210.0, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 130.0 }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 1280.0, y: -2.0 }, None));
                self.insert_coin(Coin::new(Position { x: 1365.0, y: 10.0 }, None));

                self.insert_wall(Wall::new(Position { x: 1240.0, y: 60.0 }, Size { width: 190.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1430.0, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 1424.0, y: 125.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1380.0, y: 65.0 }, false, None, None));

                self.insert_wall(Wall::new(Position { x: 1310.0, y: 90.0 }, Size { width: DEFAULT_SIZE, height: 40.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1307.0, y: 130.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_teleport_door(TeleportDoor::new(3, Position { x: 1250.0, y: 125.0 }, Position { x: 1410.0, y: 390.0 }, 7, None, None));
                self.insert_door_collectable(DoorCollectable::new(1, DoorCollectableType::Key, Position { x: 1245.0, y: 90.0 }, vec![1, 5, 6, 7], None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 516.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 0.0, y: 60.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 65.0, y: 60.0 }, Size { width: 485.0, height: DEFAULT_SIZE }, None, None));
                self.insert_wall(Wall::new(Position { x: 520.0, y: 90.0 }, Size { width: DEFAULT_SIZE, height: 100.0 }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 440.0, y: -2.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 245.0, y: -2.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 65.0, y: -2.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 380.0, y: -25.0 }, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 200.0, y: -25.0 }, false, None, None));

                self.insert_wall(Wall::new(Position { x: 410.0, y: 90.0 }, Size { width: DEFAULT_SIZE, height: 40.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 407.0, y: 130.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_teleport_door(TeleportDoor::new(4, Position { x: 460.0, y: 125.0 }, Position { x: 1630.0, y: 560.0 }, 5, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 35.0, y: 127.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 190.0, y: 127.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 342.0, y: 127.0 }, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 190.0, y: 65.0 }, false, None, None, None));
                self.insert_coin(Coin::new(Position { x: 270.0, y: 140.0 }, None));

                self.insert_wall(Wall::new(Position { x: 1489.0, y: 220.0 }, Size { width: DEFAULT_SIZE, height: 230.0 }, None, None));
                self.insert_teleport_door(TeleportDoor::new(5, Position { x: 1550.0, y: 385.0 }, Position { x: 540.0, y: 300.0 }, 4, None, None));
                self.insert_teleport_door(TeleportDoor::new(6, Position { x: 1550.0, y: 215.0 }, Position { x: 1410.0, y: 390.0 }, 7, None, None));
                
                self.insert_camera(Camera::new_without_repeat(Position { x: 1490.0, y: 310.0 }, true, Some(0.99), Some(100.0)));
                self.insert_coin(Coin::new(Position { x: 1555.0, y: 340.0 }, None));
                self.insert_door_collectable(DoorCollectable::new(2, DoorCollectableType::CodePaper, Position { x: 1555.0, y: 290.0 }, vec![2, 3, 4, 8], None));

                self.insert_door(Door::new(1, DoorType::Locked, Position { x: 1229.0, y: 220.0 }, Size { width: DEFAULT_SIZE + 10.0, height: 60.0 }, true, Some(1), None, None)?);
                self.insert_wall(Wall::new(Position { x: 1234.0, y: 280.0 }, Size { width: 195.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(2, DoorType::Coded, Position { x: 1415.0, y: 280.0 }, Size { width: 80.0, height: DEFAULT_SIZE }, true, Some(2), None, None)?);
                self.insert_teleport_door(TeleportDoor::new(7, Position { x: 1330.0, y: 215.0 }, Position { x: 1410.0, y: 390.0 }, 7, None, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1357.0, y: 310.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 1360.0, y: 370.0 }, Size { width: DEFAULT_SIZE, height: 310.0 }, None, None));
                self.insert_wall(Wall::new(Position { x: 1489.0, y: 570.0 }, Size { width: DEFAULT_SIZE, height: 40.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1486.0, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);
                self.insert_teleport_door(TeleportDoor::new(8, Position { x: 1630.0, y: 615.0 }, Position { x: 1410.0, y: 390.0 }, 7, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 1444.0, y: 320.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1390.0, y: 450.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1420.0, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1550.0, y: 615.0 }, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1620.0, y: 545.0 }, false, None, None, Some(4000)));
                self.insert_coin(Coin::new(Position { x: 1715.0, y: 635.0 }, None));
                self.insert_coin(Coin::new(Position { x: 1715.0, y: 575.0 }, None));

                self.insert_wall(Wall::new(Position { x: 1234.0, y: 310.0 }, Size { width: DEFAULT_SIZE, height: 300.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1231.0, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 1264.0, y: 310.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1315.0, y: 450.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1315.0, y: 615.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1340.0, y: 555.0 }, false, None, None));

                self.insert_wall(Wall::new(Position { x: 1034.0, y: 220.0 }, Size { width: DEFAULT_SIZE, height: 360.0 }, None, None));
                self.insert_wall(Wall::new(Position { x: 1064.0, y: 490.0 }, Size { width: 115.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1179.0, y: 490.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_teleport_door(TeleportDoor::new(9, Position { x: 1070.0, y: 515.0 }, Position { x: 890.0, y: 790.0 }, 11, None, None));
                self.insert_wall(Wall::new(Position { x: 1014.0, y: 580.0 }, Size { width: 220.0, height: DEFAULT_SIZE }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 1140.0, y: 220.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1064.0, y: 320.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1104.0, y: 430.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1214.0, y: 400.0 }, false, None, Some(25.0)));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1140.0, y: 495.0 }, false, None, None));
                self.insert_coin(Coin::new(Position { x: 1150.0, y: 330.0 }, None));
                self.insert_coin(Coin::new(Position { x: 1135.0, y: 535.0 }, None));

                self.insert_hide_place(HidePlace::new(Position { x: 1175.0, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1065.0, y: 615.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1025.0, y: 585.0 }, false, None, None));
                
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 949.0, y: 580.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 916.0, y: 220.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 919.0, y: 280.0 }, Size { width: DEFAULT_SIZE, height: 400.0 }, None, None));
                self.insert_wall(Wall::new(Position { x: 669.0, y: 280.0 }, Size { width: 250.0, height: DEFAULT_SIZE }, None, None));
                self.insert_wall(Wall::new(Position { x: 669.0, y: 220.0 }, Size { width: DEFAULT_SIZE, height: 60.0 }, None, None));
                self.insert_teleport_door(TeleportDoor::new(10, Position { x: 720.0, y: 215.0 }, Position { x: 90.0, y: 390.0 }, 12, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 989.0, y: 505.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 949.0, y: 360.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 970.0, y: 220.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 851.0, y: 220.0 }, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 780.0, y: 195.0 }, false, None, None, None));

                self.insert_wall(Wall::new(Position { x: 669.0, y: 310.0 }, Size { width: DEFAULT_SIZE, height: 300.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 699.0, y: 580.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 764.0, y: 580.0 }, Size { width: 94.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 860.0, y: 580.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 764.0, y: 610.0 }, Size { width: DEFAULT_SIZE, height: 70.0 }, None, None));
                self.insert_teleport_door(TeleportDoor::new(11, Position { x: 810.0, y: 615.0 }, Position { x: 1150.0, y: 690.0 }, 9, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 824.0, y: 510.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 874.0, y: 380.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 870.0, y: 285.0 }, false, None, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 791.0, y: 310.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 794.0, y: 370.0 }, Size { width: DEFAULT_SIZE, height: 210.0 }, None, None));
                
                self.insert_hide_place(HidePlace::new(Position { x: 725.0, y: 310.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 749.0, y: 430.0 }, None));

                self.insert_wall(Wall::new(Position { x: 0.0, y: 280.0 }, Size { width: 610.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 610.0, y: 280.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 80.0, y: 220.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_teleport_door(TeleportDoor::new(12, Position { x: 10.0, y: 215.0 }, Position { x: 800.0, y: 390.0 }, 10, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 155.0, y: 220.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 335.0, y: 220.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 500.0, y: 220.0 }, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 250.0, y: 195.0 }, false, None, None, Some(4500)));
                self.insert_camera(Camera::new_without_repeat(Position { x: 560.0, y: 195.0 }, true, None, None));

                self.insert_wall(Wall::new(Position { x: 544.0, y: 310.0 }, Size { width: DEFAULT_SIZE, height: 210.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 541.0, y: 520.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                
                self.insert_hide_place(HidePlace::new(Position { x: 574.0, y: 350.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 620.0, y: 430.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 600.0, y: 515.0 }, None));

                self.insert_wall(Wall::new(Position { x: 379.0, y: 310.0 }, Size { width: DEFAULT_SIZE, height: 210.0 }, None, None));
                self.insert_door(Door::new(3, DoorType::Coded, Position { x: 363.0, y: 520.0 }, Size { width: DEFAULT_SIZE + 30.0, height: 60.0 }, true, Some(2), None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 469.0, y: 515.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 500.0, y: 420.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 409.0, y: 330.0 }, None));
                self.insert_coin(Coin::new(Position { x: 494.0, y: 320.0 }, None));
                self.insert_coin(Coin::new(Position { x: 494.0, y: 370.0 }, None));

                self.insert_wall(Wall::new(Position { x: 360.0, y: 580.0 }, Size { width: 310.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(5, DoorType::Locked, Position { x: 300.0, y: 580.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, true, Some(1), None, None)?);
                self.insert_wall(Wall::new(Position { x: 274.0, y: 580.0 }, Size { width: DEFAULT_SIZE, height: 97.0 }, None, None));
                self.insert_door(Door::new(4, DoorType::Coded, Position { x: 199.0, y: 580.0 }, Size { width: 85.0, height: DEFAULT_SIZE }, true, Some(2), None, None)?);
                self.insert_wall(Wall::new(Position { x: 179.0, y: 370.0 }, Size { width: DEFAULT_SIZE, height: 240.0 }, None, None));
                self.insert_wall(Wall::new(Position { x: 209.0, y: 490.0 }, Size { width: 97.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(6, DoorType::Locked, Position { x: 300.0, y: 490.0 }, Size { width: 80.0, height: DEFAULT_SIZE }, true, Some(1), None, None)?);
                self.insert_wall(Wall::new(Position { x: 370.0, y: 490.0 }, Size { width: 10.0, height: DEFAULT_SIZE }, None, None));

                self.insert_coin(Coin::new(Position { x: 260.0, y: 530.0 }, None));

                self.insert_hide_place(HidePlace::new(Position { x: 640.0, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 400.0, y: 615.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 560.0, y: 585.0 }, false, None, None));
                self.insert_coin(Coin::new(Position { x: 520.0, y: 630.0 }, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 176.0, y: 310.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 234.0, y: 425.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 330.0, y: 390.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 310.0, y: 310.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 245.0, y: 285.0 }, false, None, None));
                self.insert_coin(Coin::new(Position { x: 260.0, y: 360.0 }, None));

                self.insert_door(Door::new(7, DoorType::Locked, Position { x: -8.0, y: 580.0 }, Size { width: 75.0, height: DEFAULT_SIZE }, true, Some(1), None, None)?);
                self.insert_wall(Wall::new(Position { x: 60.0, y: 580.0 }, Size { width: 119.0, height: DEFAULT_SIZE }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 120.0, y: 310.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 0.0, y: 380.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 80.0, y: 515.0 }, None));
                self.insert_coin(Coin::new(Position { x: 120.0, y: 420.0 }, None));

                self.insert_door(Door::new(8, DoorType::Coded, Position { x: 80.0, y: 608.0 }, Size { width: DEFAULT_SIZE + 30.0, height: 70.0 }, true, Some(2), None, None)?);

                self.insert_camera(Camera::new_without_repeat(Position { x: 160.0, y: 585.0 }, false, None, None));

                self.insert_exit_door(ExitDoor::new(Position { x: 0.0, y: 620.0 }, None));
            },

            4 => {
                player.move_to(Position { x: 90.0, y: 790.0 }, true);

                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 132.0, y: 615.0 }, "34r/4000 34l/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 587.0, y: 615.0 }, "29r/4500 29l/6500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 990.0, y: 615.0 }, "51r/4000 51l/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 20.0, y: 520.0 }, "40r/5500 40l/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 0.0, y: 420.0 }, "10r/0 10u/4000 10l/0 10d/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 0.0, y: 90.0 }, "29r/0 13d/0 7l/3000 13u/0 12l/0 12d/3000 10l/0 12u/3000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 285.0, y: 430.0 }, "9l/0 11u/5500 8r/0 11d/0 1r/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 704.0, y: 301.0 }, "27l/0 13d/0 4l/6500 8r/0 13u/0 23r/4500", true));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 20.0, y: 0.0 }, "68r/4000 68l/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 584.0, y: 191.0 }, "17l/3500 10u/0 29r/3500 29l/0 10d/0 17r/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 842.0, y: 411.0 }, "11d/0 13l/5000 9u/0 11l/3000 24r/0 2u/4500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1610.0, y: 615.0 }, "5r/0 23u/3500 23d/0 5l/5500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1695.0, y: 100.0 }, "5d/0 5l/0 17d/5500 17u/0 5r/0 5u/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1685.0, y: 0.0 }, "32l/3500 32r/5500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 823.0, y: 0.0 }, "37r/3000 37l/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 812.5, y: 100.0 }, "1d/0 3r/0 8d/3500 20r/4000 9u/0 23l/4500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1490.0, y: 520.0 }, "46l/5500 46r/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1460.0, y: 90.0 }, "5r/0 33d/6000 33u/0 5l/4000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1355.0, y: 90.0 }, "10d/3000 21l/2500 7u/3000 4r/0 3u/0 17r/5000", true));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1095.0, y: 281.0 }, "15d/0 2l/3500 9u/0 8l/0 4u/0 1l/3500 2u/0 23r/0 15d/0 2r/3500 2l/0 15u/0 12l/0", true));

                self.insert_wall(Wall::new(Position { x: 0.0, y: 580.0 }, Size { width: 360.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 87.0, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 360.0, y: 580.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                
                for num in 0..2 {
                    self.insert_wall(Wall::new(Position { x: 415.0 + (num as f32 * 281.25), y: 580.0 }, Size { width: 281.25, height: DEFAULT_SIZE }, None, None));
                }
                
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 542.0, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);
               
                self.insert_hide_place(HidePlace::new(Position { x: 162.0, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 390.0, y: 615.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 227.0, y: 585.0 }, true, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 455.0, y: 585.0 }, true, None, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 944.5, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);
                
                self.insert_hide_place(HidePlace::new(Position { x: 610.0, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 745.0, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 870.0, y: 615.0 }, None));
                self.insert_coin(Coin::new(Position { x: 680.0, y: 630.0 }, None));

                self.insert_wall(Wall::new(Position { x: 977.5, y: 580.0 }, Size { width: 80.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1057.5, y: 580.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 1112.5, y: 580.0 }, Size { width: 482.5, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1565.0, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 1565.0, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 520.0 }, None, None));
                self.insert_wall(Wall::new(Position { x: 1127.5, y: 60.0 }, Size { width: 437.5, height: DEFAULT_SIZE }, None, None));
                self.insert_wall(Wall::new(Position { x: 977.5, y: 251.0 }, Size { width: 120.0, height: DEFAULT_SIZE }, None, None));
                self.insert_wall(Wall::new(Position { x: 1097.5, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 221.0 }, None, None));
                
                self.insert_hide_place(HidePlace::new(Position { x: 1039.5, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1224.5, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1480.5, y: 615.0 }, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1132.5, y: 585.0 }, false, None, None, Some(5000)));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1300.0, y: 585.0 }, true, None, None));
                self.insert_coin(Coin::new(Position { x: 1355.0, y: 630.0 }, None));

                self.insert_wall(Wall::new(Position { x: 1595.0, y: 60.0 }, Size { width: 110.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1705.0, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 1655.0, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1595.0, y: 480.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1715.0, y: 345.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1595.0, y: 240.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1715.0, y: 115.0 }, None));
                self.insert_coin(Coin::new(Position { x: 1605.0, y: 100.0 }, None));
                self.insert_coin(Coin::new(Position { x: 1710.0, y: 540.0 }, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 802.5, y: 60.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 867.5, y: 60.0 }, Size { width: 230.0, height: DEFAULT_SIZE }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 1630.0, y: 0.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1530.0, y: 0.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1288.0, y: 0.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1050.0, y: 0.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 926.0, y: 0.0 }, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1455.0, y: -25.0 }, false, None, None, Some(5000)));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1133.0, y: -25.0 }, false, None, None, Some(4000)));
                self.insert_camera(Camera::new_without_repeat(Position { x: 866.0, y: -25.0 }, false, None, None));
                self.insert_coin(Coin::new(Position { x: 1373.0, y: 10.0 }, None));
                self.insert_coin(Coin::new(Position { x: 990.0, y: 10.0 }, None));

                self.insert_hide_place(HidePlace::new(Position { x: 802.5, y: 110.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 922.5, y: 90.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1052.5, y: 110.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 937.5, y: 186.0 }, None));
                self.insert_door_collectable(DoorCollectable::new(1, DoorCollectableType::Key, Position { x: 1047.5, y: 201.0 }, vec![1], None));
                self.insert_coin(Coin::new(Position { x: 812.5, y: 201.0 }, None));
                self.insert_coin(Coin::new(Position { x: 997.5, y: 125.0 }, None));

                self.insert_wall(Wall::new(Position { x: 977.5, y: 490.0 }, Size { width: 532.5, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1510.0, y: 490.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 1132.5, y: 518.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1332.5, y: 518.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1520.0, y: 518.0 }, None));
                self.insert_coin(Coin::new(Position { x: 997.5, y: 530.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1200.0, y: 495.0 }, true, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1387.5, y: 495.0 }, true, None, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1417.0, y: 90.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 1420.0, y: 150.0 }, Size { width: DEFAULT_SIZE, height: 340.0 }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 1450.0, y: 425.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1520.0, y: 310.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1450.0, y: 215.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1485.0, y: 90.0 }, None));
                self.insert_coin(Coin::new(Position { x: 1460.0, y: 325.0 }, None));

                self.insert_wall(Wall::new(Position { x: 1127.5, y: 251.0 }, Size { width: 237.5, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1365.0, y: 251.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 1335.0, y: 90.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1230.0, y: 90.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1295.0, y: 186.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1200.0, y: 186.0 }, None));
                self.insert_door_collectable(DoorCollectable::new(2, DoorCollectableType::CodePaper, Position { x: 1137.5, y: 100.0 }, vec![2], None));
                self.insert_coin(Coin::new(Position { x: 1137.5, y: 201.0 }, None));

                self.insert_wall(Wall::new(Position { x: 1290.0, y: 281.0 }, Size { width: DEFAULT_SIZE, height: 149.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1287.0, y: 430.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);

                self.insert_camera(Camera::new_without_repeat(Position { x: 1400.0, y: 375.0 }, false, None, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1157.0, y: 281.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 1160.0, y: 351.0 }, Size { width: DEFAULT_SIZE, height: 139.0 }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 1215.0, y: 425.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1245.0, y: 281.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1210.0, y: 256.0 }, false, None, None));
                self.insert_coin(Coin::new(Position { x: 1225.0, y: 365.0 }, None));

                self.insert_wall(Wall::new(Position { x: 977.5, y: 400.0 }, Size { width: 90.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1032.5, y: 430.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 1085.0, y: 281.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1092.5, y: 426.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1002.5, y: 335.0 }, None));
                self.insert_coin(Coin::new(Position { x: 982.5, y: 440.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1032.5, y: 256.0 }, false, None, None));

                self.insert_wall(Wall::new(Position { x: 802.5, y: 251.0 }, Size { width: 40.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(1, DoorType::Locked, Position { x: 832.5, y: 251.0 }, Size { width: 83.0, height: DEFAULT_SIZE }, true, Some(1), None, None)?);
                self.insert_wall(Wall::new(Position { x: 907.5, y: 251.0 }, Size { width: 40.0, height: DEFAULT_SIZE }, None, None));

                self.insert_wall(Wall::new(Position { x: 772.5, y: 0.0 }, Size { width: DEFAULT_SIZE, height: 291.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 769.5, y: 291.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 772.5, y: 361.0 }, Size { width: DEFAULT_SIZE, height: 40.0 }, None, None));

                self.insert_wall(Wall::new(Position { x: 802.5, y: 371.0 }, Size { width: 40.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(3, DoorType::Locked, Position { x: 832.5, y: 371.0 }, Size { width: 83.0, height: DEFAULT_SIZE }, true, Some(3), None, None)?);
                self.insert_wall(Wall::new(Position { x: 907.5, y: 371.0 }, Size { width: 40.0, height: DEFAULT_SIZE }, None, None));

                self.insert_wall(Wall::new(Position { x: 947.5, y: 251.0 }, Size { width: DEFAULT_SIZE, height: 40.0 }, None, None));
                self.insert_door(Door::new(2, DoorType::Coded, Position { x: 930.5, y: 291.0 }, Size { width: DEFAULT_SIZE + 30.0, height: 70.0 }, true, Some(2), None, None)?);
                self.insert_wall(Wall::new(Position { x: 947.5, y: 361.0 }, Size { width: DEFAULT_SIZE, height: 219.0 }, None, None));

                self.insert_exit_door(ExitDoor::new(Position { x: 840.0, y: 295.0 }, None));

                self.insert_wall(Wall::new(Position { x: 545.0, y: 361.0 }, Size { width: DEFAULT_SIZE, height: 219.0 }, None, None));
                self.insert_wall(Wall::new(Position { x: 575.0, y: 361.0 }, Size { width: 197.5, height: DEFAULT_SIZE + 10.0 }, None, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 0.0, y: 490.0 }, Size { width: 60.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 60.0, y: 490.0 }, Size { width: 490.0, height: DEFAULT_SIZE }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 295.0, y: 518.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 65.0, y: 518.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 200.0, y: 495.0 }, false, None, None));
                self.insert_coin(Coin::new(Position { x: 465.0, y: 530.0 }, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 0.0, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                
                for num in 0..2 {
                    self.insert_wall(Wall::new(Position { x: 55.0 + (num as f32 * 331.25), y: 60.0 }, Size { width: 331.25, height: DEFAULT_SIZE }, None, None));
                }

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 717.5, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 75.0, y: 0.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 230.0, y: 0.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 445.0, y: 0.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 600.0, y: 0.0 }, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 152.5, y: -25.0 }, false, None, None, Some(4000)));
                self.insert_camera(Camera::new_without_repeat(Position { x: 305.0, y: -25.0 }, true, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 522.5, y: -25.0 }, false, None, None, Some(4000)));
                self.insert_camera(Camera::new_without_repeat(Position { x: 675.0, y: -25.0 }, true, None, None));
                self.insert_coin(Coin::new(Position { x: 377.0, y: 10.0 }, None));

                self.insert_wall(Wall::new(Position { x: 652.5, y: 151.0 }, Size { width: 120.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 649.5, y: 181.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);
                self.insert_teleport_door(TeleportDoor::new(1, Position { x: 704.5, y: 186.0 }, Position { x: 670.0, y: 690.0 }, 2, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 662.5, y: 91.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 522.0, y: 90.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 380.0, y: 90.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 510.0, y: 186.0 }, None));
                self.insert_door_collectable(DoorCollectable::new(3, DoorCollectableType::Key, Position { x: 390.0, y: 201.0 }, vec![3], None));
                self.insert_coin(Coin::new(Position { x: 450.0, y: 155.0 }, None));
                self.insert_coin(Coin::new(Position { x: 580.0, y: 155.0 }, None));

                self.insert_wall(Wall::new(Position { x: 575.0, y: 490.0 }, Size { width: 120.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 662.0, y: 520.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_teleport_door(TeleportDoor::new(2, Position { x: 595.0, y: 515.0 }, Position { x: 790.0, y: 360.0 }, 1, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 740.0, y: 515.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 640.0, y: 425.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 902.5, y: 450.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 770.0, y: 396.0 }, None));
                self.insert_coin(Coin::new(Position { x: 585.0, y: 430.0 }, None));
                self.insert_coin(Coin::new(Position { x: 850.0, y: 530.0 }, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 157.0, y: 90.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 160.0, y: 150.0 }, Size { width: DEFAULT_SIZE, height: 340.0 }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 95.0, y: 425.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 50.0, y: 305.0 }, None));

                self.insert_wall(Wall::new(Position { x: 0.0, y: 280.0 }, Size { width: 105.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 105.0, y: 280.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 40.0, y: 215.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 77.0, y: 85.0 }, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 194.0, y: 280.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 250.0, y: 280.0 }, Size { width: 100.0, height: DEFAULT_SIZE }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 230.0, y: 85.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 265.0, y: 215.0 }, None));                      
                self.insert_camera(Camera::new_without_repeat(Position { x: 185.0, y: 200.0 }, true, None, Some(330.0)));
                self.insert_coin(Coin::new(Position { x: 300.0, y: 160.0 }, None));

                self.insert_wall(Wall::new(Position { x: 350.0, y: 90.0 }, Size { width: DEFAULT_SIZE, height: 340.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 347.0, y: 430.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 255.0, y: 305.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 225.0, y: 425.0 }, None));
                self.insert_coin(Coin::new(Position { x: 230.0, y: 380.0 }, None));

                self.insert_wall(Wall::new(Position { x: 380.0, y: 251.0 }, Size { width: 392.5, height: DEFAULT_SIZE }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 435.0, y: 425.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 400.0, y: 276.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 550.0, y: 296.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 650.0, y: 296.0 }, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 600.0, y: 256.0 }, false, None, None, Some(5000)));
                self.insert_camera(Camera::new_without_repeat(Position { x: 695.0, y: 256.0 }, true, None, None));
                self.insert_coin(Coin::new(Position { x: 495.0, y: 440.0 }, None));
                self.insert_coin(Coin::new(Position { x: 505.0, y: 310.0 }, None));
                self.insert_coin(Coin::new(Position { x: 715.0, y: 310.0 }, None));
            },

            5 => {
                player.move_to(Position { x: 930.0, y: 460.0 }, false);

                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 883.0, y: 615.0 }, "3l/0 21u/5500 21d/0 3r/3500", true));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 990.0, y: 615.0 }, "27r/4000 27l/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1369.0, y: 615.0 }, "31r/5000 31l/4000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1490.0, y: 520.0 }, "2r/0 12u/0 19r/2000 11d/4000 11u/0 19l/0 12d/0 2l/4000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 977.0, y: 510.0 }, "1d/0 40r/6500 40l 1u/4500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1280.0, y: 420.0 }, "28l/5500 28r/3500", true));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 988.0, y: 301.0 }, "11r/0 14u/0 11l/0 7u/2000 11r/4000 11l/0 7d/0 11r/0 14d/0 11l/4000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1193.0, y: 101.0 }, "11r/2000 6d/0 11l/0 14d/0 11r/3000 11l/0 14u/0 11r/0 6u/0 11l/4000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1400.0, y: 301.0 }, "13r/0 10u/0 11l/2500 11u/0 11r/2500 21d/0 1r/3500 14l/4000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1650.0, y: 301.0 }, "2r/0 19u/0 3r/0 2u/4000 2d/0 3l/0 19d/0 2l/5000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1364.0, y: 0.0 }, "32r/5500 32l/4500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 990.0, y: 0.0 }, "28r/5000 28l/3000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 485.0, y: 0.0 }, "28r/5500 28l/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 155.0, y: 0.0 }, "3r/0 6d/0 14r/0 6u/0 6r/4000 6l/0 6d/0 14l/0 6u/0 3l/4000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 698.0, y: 291.0 }, "2l/0 22d/4000 22u/0 2r/5000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 488.0, y: 615.0 }, "20r/5000 20l/7000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 320.0, y: 615.0 }, "2r/0 16u/0 22r/2000 7d/0 8l/3500 7u/0 14l/0 16d/0 2l/5500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 615.0 }, "21r/5500 21l/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 0.0, y: 380.0 }, "13d/3500 5r/0 1d/0 1r/3500 13u/0 6l/0 1u/7000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 160.0, y: 520.0 }, "3r/0 15u/0 24r/3500 24l/0 15d/0 3l/5500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 815.0, y: 90.0 }, "3r/0 9d/7500 9u/0 3l/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 705.0, y: 90.0 }, "21l/5000 21r/6500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 483.0, y: 191.0 }, "21r/4000 21l/5000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 323.0, y: 150.0 }, "1r/0 3d/0 5r/3500 2l/0 10d/0 2r/3000 5l/2500 13u/0 1l/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 162.0, y: 280.0 }, "3r/0 13u/0 3r/5500 3l/0 13d/0 3l/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 0.0, y: 100.0 }, "17d/3500 5r/0 1d/0 1r/3500 1l/0 15u/0 5l/0 3u/3500", false));

                // top
                self.insert_wall(Wall::new(Position { x: 802.5, y: 251.0 }, Size { width: 40.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 842.5, y: 251.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 907.5, y: 251.0 }, Size { width: 40.0, height: DEFAULT_SIZE }, None, None));

                // left
                self.insert_wall(Wall::new(Position { x: 772.5, y: 251.0 }, Size { width: DEFAULT_SIZE, height: 40.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 769.5, y: 291.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 772.5, y: 361.0 }, Size { width: DEFAULT_SIZE, height: 40.0 }, None, None));

                // bottom
                self.insert_wall(Wall::new(Position { x: 802.5, y: 371.0 }, Size { width: 40.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 842.5, y: 371.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 907.5, y: 371.0 }, Size { width: 40.0, height: DEFAULT_SIZE }, None, None));

                // right
                self.insert_wall(Wall::new(Position { x: 947.5, y: 251.0 }, Size { width: DEFAULT_SIZE, height: 34.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 944.5, y: 285.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 75.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 947.5, y: 361.0 }, Size { width: DEFAULT_SIZE, height: 40.0 }, None, None));
                
                self.insert_wall(Wall::new(Position { x: 772.5, y: 401.0 }, Size { width: DEFAULT_SIZE, height: 279.0 }, None, None));
                
                for num in 0..2 {
                    self.insert_wall(Wall::new(Position { x: 977.5 + (num as f32 * 391.25), y: 361.0 }, Size { width: 391.25, height: DEFAULT_SIZE + 10.0 }, None, None));
                }

                self.insert_wall(Wall::new(Position { x: 947.5, y: 401.0 }, Size { width: DEFAULT_SIZE, height: 209.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 944.5, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 802.5, y: 411.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 902.5, y: 494.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 852.5, y: 615.0 }, None));
                self.insert_coin(Coin::new(Position { x: 812.5, y: 514.0 }, None));
                
                self.insert_wall(Wall::new(Position { x: 977.5, y: 580.0 }, Size { width: 727.5, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1705.0, y: 580.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                
                self.insert_hide_place(HidePlace::new(Position { x: 999.5, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1129.125, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1258.75, y: 615.0 }, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1070.0, y: 585.0 }, false, None, None, None));
                
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1323.75, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);
                
                self.insert_hide_place(HidePlace::new(Position { x: 1388.75, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1600.0, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1700.0, y: 615.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1453.75, y: 585.0 }, true, None, None));
                self.insert_coin(Coin::new(Position { x: 1533.75, y: 630.0 }, None));
                
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1602.0, y: 401.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 1605.0, y: 471.0 }, Size { width: DEFAULT_SIZE, height: 109.0 }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 1635.0, y: 515.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1675.0, y: 401.0 }, None));
                
                self.insert_wall(Wall::new(Position { x: 1450.0, y: 401.0 }, Size { width: DEFAULT_SIZE, height: 109.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1447.0, y: 510.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 1520.0, y: 401.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1560.0, y: 515.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1585.0, y: 475.0 }, false, None, None));
                self.insert_coin(Coin::new(Position { x: 1490.0, y: 455.0 }, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 977.5, y: 480.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 1032.5, y: 480.0 }, Size { width: 417.5, height: DEFAULT_SIZE }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 1382.0, y: 515.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1207.0, y: 515.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 997.0, y: 515.0 }, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1300.0, y: 485.0 }, false, None, None, Some(4000)));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1147.0, y: 485.0 }, false, None, None));
                self.insert_coin(Coin::new(Position { x: 1299.5, y: 530.0 }, None));

                self.insert_wall(Wall::new(Position { x: 1348.0, y: 401.0 }, Size { width: DEFAULT_SIZE, height: 10.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1345.0, y: 411.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);
                self.insert_teleport_door(TeleportDoor::new(1, Position { x: 1390.0, y: 415.0 }, Position { x: 960.0, y: 170.0 }, 2, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 1052.0, y: 415.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1280.0, y: 415.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1180.0, y: 415.0 }, None));
                self.insert_coin(Coin::new(Position { x: 1120.0, y: 430.0 }, None));

                self.insert_wall(Wall::new(Position { x: 947.5, y: 90.0 }, Size { width: DEFAULT_SIZE, height: 161.0 }, None, None));
                
                for num in 0..3 {
                    if num == 2 {
                        self.insert_wall(Wall::new(Position { x: 947.5 + (num as f32 * 252.5), y: 60.0 }, Size { width: 245.0, height: DEFAULT_SIZE }, None, None));
                    } else {
                        self.insert_wall(Wall::new(Position { x: 947.5 + (num as f32 * 252.5), y: 60.0 }, Size { width: 252.5, height: DEFAULT_SIZE }, None, None));
                    }
                }
                
                self.insert_hide_place(HidePlace::new(Position { x: 1007.5, y: 296.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1107.5, y: 261.0 }, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1055.0, y: 226.0 }, true, None, None, Some(4000)));

                self.insert_wall(Wall::new(Position { x: 977.5, y: 221.0 }, Size { width: 110.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1090.0, y: 221.0 }, Size { width: 60.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                
                self.insert_hide_place(HidePlace::new(Position { x: 1022.5, y: 156.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1094.5, y: 90.0 }, None));
                self.insert_coin(Coin::new(Position { x: 987.5, y: 100.0 }, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1149.5, y: 90.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 1152.5, y: 150.0 }, Size { width: DEFAULT_SIZE, height: 211.0 }, None, None));
                
                self.insert_hide_place(HidePlace::new(Position { x: 1194.5, y: 90.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1267.5, y: 156.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1259.5, y: 65.0 }, true, None, None));
                self.insert_door_collectable(DoorCollectable::new(1, DoorCollectableType::Key, Position { x: 1307.5, y: 100.0 }, vec![1], None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1182.5, y: 221.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 1250.0, y: 221.0 }, Size { width: 110.0, height: DEFAULT_SIZE }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 1297.5, y: 296.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1182.5, y: 296.0 }, None));

                self.insert_wall(Wall::new(Position { x: 1357.5, y: 90.0 }, Size { width: DEFAULT_SIZE, height: 201.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1354.5, y: 291.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);
            
                self.insert_wall(Wall::new(Position { x: 1387.5, y: 261.0 }, Size { width: 135.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(1, DoorType::Locked, Position { x: 1515.0, y: 261.0 }, Size { width: 85.0, height: DEFAULT_SIZE }, true, Some(1), None, None)?);
                self.insert_wall(Wall::new(Position { x: 1592.5, y: 261.0 }, Size { width: 47.5, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1605.0, y: 291.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);
                
                self.insert_hide_place(HidePlace::new(Position { x: 1399.5, y: 296.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1537.5, y: 296.0 }, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1467.5, y: 266.0 }, false, None, None, Some(4500)));

                self.insert_wall(Wall::new(Position { x: 1610.0, y: 90.0 }, Size { width: DEFAULT_SIZE, height: 171.0 }, None, None));
                
                self.insert_hide_place(HidePlace::new(Position { x: 1462.5, y: 196.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1442.5, y: 90.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1565.0, y: 160.0 }, None));
                self.insert_coin(Coin::new(Position { x: 1397.5, y: 211.0 }, None));
                self.insert_coin(Coin::new(Position { x: 1397.5, y: 100.0 }, None));
                self.insert_coin(Coin::new(Position { x: 1560.0, y: 100.0 }, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1695.0, y: 60.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None, None)?);
            
                self.insert_hide_place(HidePlace::new(Position { x: 1705.0, y: 296.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1715.0, y: 181.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1640.0, y: 90.0 }, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 1318.75, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 1640.0, y: 0.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1505.0, y: 0.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 1363.75, y: 0.0 }, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1580.0, y: -25.0 }, false, None, None, Some(4000)));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 944.5, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 1193.75, y: 0.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 989.5, y: 0.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1140.0, y: -25.0 }, false, None, None));
                self.insert_coin(Coin::new(Position { x: 1080.0, y: 10.0 }, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 829.5, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_teleport_door(TeleportDoor::new(2, Position { x: 884.5, y: -5.0 }, Position { x: 960.0, y: 170.0 }, 2, None, None));

                self.insert_wall(Wall::new(Position { x: 472.5, y: 60.0 }, Size { width: 475.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 439.5, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 764.5, y: 0.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 629.5, y: 0.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 489.5, y: 0.0 }, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 704.5, y: -25.0 }, false, None, None, Some(4000)));
                self.insert_coin(Coin::new(Position { x: 564.5, y: 10.0 }, None));

                self.insert_wall(Wall::new(Position { x: 442.5, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 90.0 }, None, None));
                self.insert_wall(Wall::new(Position { x: 150.0, y: 120.0 }, Size { width: 292.5, height: DEFAULT_SIZE }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 375.0, y: 0.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 320.0, y: 58.0 }, None));
                self.insert_coin(Coin::new(Position { x: 321.25, y: 10.0 }, None));

                self.insert_wall(Wall::new(Position { x: 281.25, y: 0.0 }, Size { width: DEFAULT_SIZE, height: 60.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 278.25, y: 60.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 230.0, y: 58.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 160.0, y: 0.0 }, None));
                self.insert_coin(Coin::new(Position { x: 231.25, y: 10.0 }, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 117.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 0.0, y: 60.0 }, Size { width: 60.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 60.0, y: 60.0 }, Size { width: 60.0, height: DEFAULT_SIZE }, None, None));
                self.insert_exit_door(ExitDoor::new(Position { x: 30.0, y: 0.0 }, None));

                self.insert_wall(Wall::new(Position { x: 652.5, y: 251.0 }, Size { width: 120.0, height: DEFAULT_SIZE }, None, None));
                self.insert_wall(Wall::new(Position { x: 622.5, y: 251.0 }, Size { width: DEFAULT_SIZE, height: 359.0 }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 687.5, y: 281.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 652.5, y: 396.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 652.5, y: 515.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 642.5, y: 470.0 }, true, None, None));
                self.insert_coin(Coin::new(Position { x: 722.5, y: 462.0 }, None));

                self.insert_wall(Wall::new(Position { x: 652.5, y: 580.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 717.5, y: 580.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 697.5, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 487.5, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 595.5, y: 615.0 }, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 545.0, y: 585.0 }, false, None, None, Some(4000)));
                
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 467.5, y: 580.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 522.5, y: 580.0 }, Size { width: 100.0, height: DEFAULT_SIZE }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 542.5, y: 518.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 479.5, y: 455.0 }, None));
                self.insert_door_collectable(DoorCollectable::new(2, DoorCollectableType::CodePaper, Position { x: 582.5, y: 460.0 }, vec![2], None));

                self.insert_wall(Wall::new(Position { x: 282.5, y: 430.0 }, Size { width: 340.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 434.5, y: 460.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 437.5, y: 520.0 }, Size { width: DEFAULT_SIZE, height: 160.0 }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 354.5, y: 455.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 387.5, y: 615.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 417.5, y: 565.0 }, false, None, None));
                self.insert_coin(Coin::new(Position { x: 322.5, y: 552.0 }, None));

                self.insert_wall(Wall::new(Position { x: 282.5, y: 460.0 }, Size { width: DEFAULT_SIZE, height: 150.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 279.5, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 214.5, y: 615.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 105.0, y: 615.0 }, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 65.0, y: 585.0 }, false, None, None));
                
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 0.0, y: 580.0 }, Size { width: 60.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 60.0, y: 580.0 }, Size { width: 222.0, height: DEFAULT_SIZE }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 60.0, y: 515.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 0.0, y: 440.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 75.0, y: 370.0 }, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 0.0, y: 340.0 }, Size { width: 60.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 60.0, y: 340.0 }, Size { width: 90.0, height: DEFAULT_SIZE }, None, None));
                self.insert_wall(Wall::new(Position { x: 150.0, y: 340.0 }, Size { width: 472.5, height: DEFAULT_SIZE }, None, None));

                self.insert_wall(Wall::new(Position { x: 120.0, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 220.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 117.0, y: 280.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 120.0, y: 340.0 }, Size { width: DEFAULT_SIZE, height: 170.0 }, None, None));
                self.insert_door(Door::new(2, DoorType::Coded, Position { x: 105.0, y: 510.0 }, Size { width: DEFAULT_SIZE + 30.0, height: 70.0 }, true, Some(2), None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 190.0, y: 515.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 150.0, y: 370.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 302.5, y: 370.0 }, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 226.25, y: 345.0 }, false, None, None, Some(4500)));
                self.insert_camera(Camera::new_without_repeat(Position { x: 367.5, y: 345.0 }, true, None, None));
                self.insert_coin(Coin::new(Position { x: 582.5, y: 370.0 }, None));
                self.insert_coin(Coin::new(Position { x: 582.5, y: 380.0 }, None));
                self.insert_coin(Coin::new(Position { x: 582.5, y: 390.0 }, None));
                
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 487.5, y: 370.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_teleport_door(TeleportDoor::new(3, Position { x: 532.5, y: 365.0 }, Position { x: 140.0, y: 450.0 }, 4, None, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 769.5, y: 90.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 772.5, y: 150.0 }, Size { width: DEFAULT_SIZE, height: 101.0 }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 902.5, y: 186.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 902.5, y: 90.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 814.5, y: 90.0 }, None));
                self.insert_coin(Coin::new(Position { x: 802.5, y: 191.0 }, None));
                
                self.insert_wall(Wall::new(Position { x: 472.0, y: 150.0 }, Size { width: 35.0, height: DEFAULT_SIZE }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 508.0, y: 150.0 }, Size { width: 70.0, height: DEFAULT_SIZE }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 580.0, y: 150.0 }, Size { width: 195.0, height: DEFAULT_SIZE }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 714.5, y: 85.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 580.0, y: 85.0 }, None));
                self.insert_coin(Coin::new(Position { x: 472.5, y: 100.0 }, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 664.5, y: 65.0 }, true, None, None, Some(4000)));

                self.insert_wall(Wall::new(Position { x: 442.5, y: 150.0 }, Size { width: DEFAULT_SIZE, height: 31.0 }, None, None));
                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 439.5, y: 181.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 442.5, y: 251.0 }, Size { width: 180.0, height: DEFAULT_SIZE - 2.0 }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 492.5, y: 186.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 607.5, y: 186.0 }, None));
                self.insert_door_collectable(DoorCollectable::new(3, DoorCollectableType::Key, Position { x: 722.5, y: 201.0 }, vec![3], None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 672.5, y: 155.0 }, true, None, None));

                self.insert_door(Door::new(3, DoorType::Locked, Position { x: 434.5, y: 277.0 }, Size { width: DEFAULT_SIZE + 20.0, height: 65.0 }, true, Some(3), None, None)?);

                self.insert_hide_place(HidePlace::new(Position { x: 353.25, y: 150.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 311.25, y: 230.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 379.5, y: 275.0 }, None));

                self.insert_hide_place(HidePlace::new(Position { x: 489.5, y: 280.0 }, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 534.5, y: 256.0 }, false, None, None, None));
                self.insert_coin(Coin::new(Position { x: 572.5, y: 280.0 }, None));
                self.insert_coin(Coin::new(Position { x: 572.5, y: 300.0 }, None));

                self.insert_door(Door::new(0, DoorType::Regular, Position { x: 278.25, y: 150.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)?);
                self.insert_wall(Wall::new(Position { x: 281.25, y: 210.0 }, Size { width: DEFAULT_SIZE, height: 130.0 }, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 193.25, y: 150.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 226.25, y: 275.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 160.0, y: 275.0 }, None));
                self.insert_coin(Coin::new(Position { x: 150.0, y: 225.0 }, None));
                
                self.insert_teleport_door(TeleportDoor::new(4, Position { x: 65.0, y: 275.0 }, Position { x: 140.0, y: 450.0 }, 4, None, None));

                self.insert_hide_place(HidePlace::new(Position { x: 0.0, y: 265.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 80.0, y: 180.0 }, None));
                self.insert_hide_place(HidePlace::new(Position { x: 60.0, y: 85.0 }, None));
                self.insert_coin(Coin::new(Position { x: 10.0, y: 205.0 }, None));
                self.insert_coin(Coin::new(Position { x: 10.0, y: 110.0 }, None));
                self.insert_coin(Coin::new(Position { x: 150.0, y: 225.0 }, None));
            },

            _ => ()
        }

        self.status = LevelStatus::NotDetermine;

        Ok(())
    }
}
