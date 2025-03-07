use detective_game::{game::{camera::Camera, character::Character, collectable::{DoorCollectable, DoorCollectableType}, door::{Door, DoorType, TeleportDoor}, enemy::{Enemy, EnemyType}, hide_place::HidePlace, level::{GameObject, DEFAULT_SIZE}, player::{Player, PlayerStatus, DEFAULT_SIZE_FOR_INVENTORY_ITEM}, wall::Wall}, library::utils::get_attached_enemy_index, renderer::{color::Color, error::Result, render::{Render, Size}, vertice::Position}};
use glfw::{Action, Key};

pub type SimulationResult<T> = std::result::Result<T, SimulationError>;

#[derive(Debug)]
pub enum SimulationError {
    LoadSimulationError(SimulatorType, String),
}

impl std::fmt::Display for SimulationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error occurred, ").unwrap();
        
        match self {
            Self::LoadSimulationError(simulation_type, message) => write!(f, "Load Simulation Error: {}\nin: {:?}", message, simulation_type),
        }
    }
}

impl std::error::Error for SimulationError {}

#[derive(Debug, Clone)]
pub enum SimulatorType {
    EnemyLogic,
    PlayerInteractionWithHidePlace,
    CameraLogic,
    DoorLogic,
    PlayerLogic,
    Other
}

#[derive(PartialEq)]
pub enum SimulationStatus {
    Lose,
    Win,
    NotDetermine,
}

pub struct Simulator<'a> {
    walls: Vec<Wall<'a>>,
    doors: Vec<Door<'a>>,
    teleport_doors: Vec<TeleportDoor<'a>>,
    door_collectables: Vec<DoorCollectable<'a>>,
    hide_places: Vec<HidePlace<'a>>,
    enemies: Vec<Enemy<'a>>,
    attached_enemies_ids: Vec<(usize, Position)>,
    cameras: Vec<Camera<'a>>,
    status: SimulationStatus,
    notoriety_level: u64, 
}

impl Simulator<'_> {
    pub fn new() -> Self {
        Self {
            walls: Vec::new(),
            doors: Vec::new(),
            teleport_doors: Vec::new(),
            door_collectables: Vec::new(),
            hide_places: Vec::new(),
            enemies: Vec::new(),
            attached_enemies_ids: Vec::new(),
            cameras: Vec::new(),
            status: SimulationStatus::NotDetermine,
            notoriety_level: 0,
        }
    }
}

impl<'a> Simulator<'a> {
    pub fn draw(&mut self, player: &mut Player<'a>, render: &mut Render<'a>) -> Result<()> {
        render.fill_with_image("assets/game/background.jpg")?;
        
        render.display_text(&format!("status: {}", player.get_status()), Position { x: 400.0, y: 500.0 }, 1.0, None, Color::White).expect("Unable to display text"); 
        render.display_text(&format!("notoriety level: {}", self.notoriety_level), Position { x: 10.0, y: 560.0 }, 1.0, None, Color::White).expect("Unable to display text"); 
        
        // Should be in a function
        let holding = player.get_holding_item();

        let holding_info: (Option<&str>, String);
        if let Some(item) = holding {
            holding_info = (Some(item.get_image()), item.get_name());
        } else {
            holding_info = (None, String::from("nothing"));
        }

        if let Some(image_path) = holding_info.0 {
            render.load_image(image_path, Position { x: 150.0, y: 650.0 }, DEFAULT_SIZE_FOR_INVENTORY_ITEM, false, None, None, None, None)?;
        }

        render.display_text(&format!("holding: {}", holding_info.1), Position { x: 200.0, y: 650.0 }, 1.0, None, Color::White).expect("Unable to display text"); 
        // Should be in a function

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
            let want_to_teleport_enemies: Vec<&mut Enemy<'a>> = self.enemies.iter_mut().filter(| enemy | enemy.get_want_to_teleport() && enemy.collide(teleport_door)).collect();
            
            if want_to_teleport_enemies.len() > 0 {
                for enemy in want_to_teleport_enemies {
                    teleport_door.teleport(enemy);
                    enemy.set_want_to_teleport(false);
                }
            }           

            if player.is_colliding_with_object(teleport_door) {
                if let Some(player_interaction) = player.get_interaction() {
                    if !player.get_is_teleported() && player_interaction.key() == &Key::Space && player_interaction.action() == &Action::Press {
                        teleport_door.teleport(player);
                        player.set_is_teleported(true);
                    }
                }
            } else if player.get_is_teleported() {
                player.set_is_teleported(false);
            }

            teleport_door.draw(render)?;
        }

        for door_collectable in self.door_collectables.iter_mut() {
            if !door_collectable.is_collected() && player.collide(door_collectable) {
                player.add_door_collectable(door_collectable.get_id(), door_collectable.opens());

                door_collectable.set_is_collected(true);
            }

            door_collectable.draw(render)?;
        }

        for hide_place in self.hide_places.iter() {
            if player.is_colliding_with_object(hide_place) {
                if let Some(player_interaction) = player.get_interaction() {
                    let player_status = player.get_status();

                    if (!player.get_is_detected_by_enemy() || player_status != &PlayerStatus::Detectit) && player_interaction.key() == &Key::Space && player_interaction.action() == &Action::Press {
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

            if let Some(enemy_id) = enemy_id {
                self.attached_enemies_ids.push((enemy_id, detected_player_position.unwrap()));
            }
        }

        for enemy in self.enemies.iter_mut() {
            let idx = get_attached_enemy_index(&self.attached_enemies_ids, enemy.get_id());
            if idx != -1 {
                let (.., detected_player_position) = self.attached_enemies_ids[idx as usize];

                enemy.attach_camera(detected_player_position);

                self.attached_enemies_ids.remove(idx as usize);
            }

            let mut is_enemy_colliding_with_a_wall = false;

            for wall in self.walls.iter() {
                if enemy.collide(wall) {
                    is_enemy_colliding_with_a_wall = true;

                    break;
                }
            }

            if enemy.is_off_window(render.get_size()) || is_enemy_colliding_with_a_wall {
                enemy.set_is_colliding(true);
                enemy.move_to_prev_position();
            } else {
                enemy.set_is_colliding(false);
            }

            if enemy.collide_with_player(&player) {
                self.status = SimulationStatus::Lose;

                render.display_text("Lost", Position { x: 10.0, y: 500.0 }, 1.0, None, Color::White).expect("Unable to display text");
            } else {
                render.display_text("Still playing", Position { x: 10.0, y: 500.0 }, 1.0, None, Color::White).expect("Unable to display text");
            }

            render.display_text(&format!("enemy mode: {}", enemy.get_mode()), Position { x: 400.0, y: 560.0 }, 1.0, None, Color::White)?;

            enemy.draw(render)?;
            
            self.notoriety_level = enemy.move_enemy(
                player, 
                self.notoriety_level,
                Position { x: 0.0, y: 0.0 },
                render.get_size(),  
                &self.walls,
                &self.doors,
                &self.hide_places
            );
        }

        player.draw(render)?;
        player.switch_items();

        Ok(())
    }

    pub fn load_simulation(&mut self, simulator_type: SimulatorType) -> SimulationResult<()> {
        self.clear_all();

        match simulator_type {
            SimulatorType::EnemyLogic => {
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 290.0, y: 260.0 }, "6u/5500 6r/0 6d/5500 6u/0 9r/5500 6d/5500 15l/5500", false));

                self.walls.push(Wall::new(Position { x: 250.0, y: 170.0 }, Size { width: 250.0, height: DEFAULT_SIZE }, None, None));
                self.walls.push(Wall::new(Position { x: 250.0, y: 200.0 }, Size { width: DEFAULT_SIZE, height: 60.0 }, None, None));
                self.doors.push(
                    Door::new(0, DoorType::Regular, Position { x: 247.0, y: 260.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)
                        .map_err(| error | SimulationError::LoadSimulationError(simulator_type.clone(), error.to_string()) )?
                );
                self.walls.push(Wall::new(Position { x: 500.0, y: 170.0 }, Size { width: DEFAULT_SIZE, height: 180.0 }, None, None));
                self.walls.push(Wall::new(Position { x: 250.0, y: 320.0 }, Size { width: 195.0, height: DEFAULT_SIZE }, None, None));
                self.doors.push(
                    Door::new(1, DoorType::Regular, Position { x: 445.0, y: 320.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)
                        .map_err(| error | SimulationError::LoadSimulationError(simulator_type, error.to_string()) )?
                );
                
                self.hide_places.push(HidePlace::new(Position { x: 302.0, y: 255.0 }, None));
                self.hide_places.push(HidePlace::new(Position { x: 375.0, y: 200.0 }, None));
                self.hide_places.push(HidePlace::new(Position { x: 455.0, y: 240.0 }, None));
            },

            SimulatorType::PlayerInteractionWithHidePlace => {
                self.hide_places.push(HidePlace::new(Position { x: 300.0, y: 400.0 }, None));
            },

            SimulatorType::CameraLogic => {
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 290.0, y: 260.0 }, "6u/5500 6r/0 6d/5500 6u/0 9r/5500 6d/5500 15l/5500", false));
                // self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 500.0, y: 20.0 }, "4r/2500 4l/2500", false));

                self.walls.push(Wall::new(Position { x: 250.0, y: 170.0 }, Size { width: 250.0, height: DEFAULT_SIZE }, None, None));
                self.walls.push(Wall::new(Position { x: 250.0, y: 200.0 }, Size { width: DEFAULT_SIZE, height: 60.0 }, None, None));
                self.doors.push(
                    Door::new(0, DoorType::Regular, Position { x: 247.0, y: 260.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None, None)
                        .map_err(| error | SimulationError::LoadSimulationError(simulator_type.clone(), error.to_string()) )?
                );
                self.walls.push(Wall::new(Position { x: 500.0, y: 170.0 }, Size { width: DEFAULT_SIZE, height: 180.0 }, None, None));
                self.walls.push(Wall::new(Position { x: 250.0, y: 320.0 }, Size { width: 195.0, height: DEFAULT_SIZE }, None, None));
                self.doors.push(
                    Door::new(1, DoorType::Regular, Position { x: 445.0, y: 320.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None, None)
                        .map_err(| error | SimulationError::LoadSimulationError(simulator_type, error.to_string()) )?
                );
                
                self.hide_places.push(HidePlace::new(Position { x: 302.0, y: 255.0 }, None));
                self.hide_places.push(HidePlace::new(Position { x: 455.0, y: 240.0 }, None));
                // self.cameras.push(Camera::new_without_repeat(Position { x: 370.0, y: 175.0 }, false, None, None));
                self.cameras.push(Camera::new_with_repeat(Position { x: 370.0, y: 175.0 }, false, None, None, Some(5000)));
                // self.cameras.push(Camera::new_without_repeat(Position { x: 295.0, y: 180.0 }, false, None, Some(90.0)));
            },

            SimulatorType::DoorLogic => {
                // self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 290.0, y: 260.0 }, "6u/5500 6r/0 6d/5500 6u/0 9r/5500 6d/5500 15l/5500", false));

                self.door_collectables.push(DoorCollectable::new(1, DoorCollectableType::Key, Position { x: 400.0, y: 20.0 }, vec![1], None));
                self.door_collectables.push(DoorCollectable::new(2, DoorCollectableType::CodePaper, Position { x: 20.0, y: 400.0 }, vec![2], None));

                self.teleport_doors.push(TeleportDoor::new(Position { x: 200.0, y: 100.0 }, Position { x: 360.0, y: 260.0 }, None, None));

                self.walls.push(Wall::new(Position { x: 250.0, y: 170.0 }, Size { width: 250.0, height: DEFAULT_SIZE }, None, None));
                self.walls.push(Wall::new(Position { x: 250.0, y: 200.0 }, Size { width: DEFAULT_SIZE, height: 60.0 }, None, None));
                self.doors.push(
                    Door::new(1, DoorType::Locked, Position { x: 243.0, y: 260.0 }, Size { width: DEFAULT_SIZE + 15.0, height: 60.0 }, true, Some(1), None, None)
                        .map_err(| error | SimulationError::LoadSimulationError(simulator_type.clone(), error.to_string()) )?
                );
                self.walls.push(Wall::new(Position { x: 500.0, y: 170.0 }, Size { width: DEFAULT_SIZE, height: 180.0 }, None, None));
                self.walls.push(Wall::new(Position { x: 250.0, y: 320.0 }, Size { width: 195.0, height: DEFAULT_SIZE }, None, None));
                self.doors.push(
                    Door::new(2, DoorType::Coded, Position { x: 430.0, y: 320.0 }, Size { width: 85.0, height: DEFAULT_SIZE }, true, Some(2), None, None)
                        .map_err(| error | SimulationError::LoadSimulationError(simulator_type, error.to_string()) )?
                );
                self.teleport_doors.push(TeleportDoor::new(Position { x: 360.0, y: 255.0 }, Position { x: 200.0, y: 100.0 }, None, None));
                
                self.hide_places.push(HidePlace::new(Position { x: 302.0, y: 255.0 }, None));
            },

            SimulatorType::PlayerLogic => {

            }

            SimulatorType::Other => () 
        }

        Ok(())
    }

    pub fn clear_all(&mut self) {
        self.doors.clear();
        self.teleport_doors.clear();
        self.door_collectables.clear();
        self.walls.clear();
        self.hide_places.clear();
        self.enemies.clear();
        self.cameras.clear();
        self.attached_enemies_ids.clear();
    }

    pub fn get_status(&self) -> &SimulationStatus {
        &self.status
    }

    pub fn set_status(&mut self, new_value: SimulationStatus) {
        self.status = new_value;
    }
}
