use detective_game::{game::{camera::Camera, character::Character, door::Door, enemy::{Enemy, EnemyType}, level::{GameObject, ObjectLevel, ObjectLevelType, DEFAULT_SIZE, DEFAULT_SIZE_FOR_HIDE_PLACE}, player::Player, wall::Wall}, renderer::{color::Color, error::Result, render::{Render, Size}, vertice::Position}};

pub enum SimulatorType {
    EnemyLogic,
    Other
}

#[derive(PartialEq)]
pub enum SimulationStatus {
    Lose,
    Win,
    NotDetermine,
}

pub struct Simulator<'a> {
    objects: Vec<ObjectLevel<'a>>,
    walls: Vec<Wall<'a>>,
    doors: Vec<Door<'a>>,
    enemies: Vec<Enemy<'a>>,
    cameras: Vec<Camera<'a>>,
    status: SimulationStatus,
    notoriety_level: u64, 
}

impl Simulator<'_> {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            walls: Vec::new(),
            doors: Vec::new(),
            enemies: Vec::new(),
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
        render.display_text(&format!("notoriety level: {}", self.notoriety_level), Position { x: 400.0, y: 560.0 }, 1.0, None, Color::White).expect("Unable to display text"); 

        for object in self.objects.iter_mut() {
            match object.get_type() {
                ObjectLevelType::Wall => {
                    if player.collide(object) {
                        player.move_to_prev_position();
                    }
                },

                ObjectLevelType::RegularDoor => {
                    if player.collide(object) {
                        object.open_door();
                    } else {
                        object.close_door();
                    }
                }

                _ => ()
            }
            

            object.draw(render)?;
        }

        for camera in self.cameras.iter_mut() {
            camera.draw(render)?;
        }

        for enemy in self.enemies.iter_mut() {
            if enemy.collide_with_player(&player) {
                self.status = SimulationStatus::Lose;

                render.display_text("Lost", Position { x: 10.0, y: 500.0 }, 1.0, None, Color::White).expect("Unable to display text");
            } else {
                render.display_text("Still playing", Position { x: 10.0, y: 500.0 }, 1.0, None, Color::White).expect("Unable to display text");
            }

            enemy.draw(render)?;
            self.notoriety_level = enemy.detect_player(self.notoriety_level, player);
            
            enemy.move_enemy(
                player, 
                self.notoriety_level,
                Position { x: 0.0, y: 0.0 },
                Size { width: 800.0, height: 600.0 }, 
                &[
                    Wall::new(Position { x: 250.0, y: 170.0 }, Size { width: 250.0, height: DEFAULT_SIZE }, None, None),
                    Wall::new(Position { x: 500.0, y: 170.0 }, Size { width: DEFAULT_SIZE, height: 180.0 }, None, None),
                    Wall::new(Position { x: 250.0, y: 320.0 }, Size { width: 250.0, height: DEFAULT_SIZE }, None, None)
                ]
            );
        }

        player.draw(render)?;

        Ok(())
    }

    pub fn load_simulation(&mut self, simulator_type: SimulatorType) {
        self.clear_all();

        match simulator_type {
            SimulatorType::EnemyLogic => {
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 295.0, y: 260.0 }, "6u/0 15r/5500 6d/0 15l/3500", false));

                self.objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 250.0, y: 170.0 }, Size { width: 250.0, height: DEFAULT_SIZE }, false, None, None));
                // self.objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 250.0, y: 200.0 }, Size { width: DEFAULT_SIZE, height: 60.0 }, false, None, None));
                // self.objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 247.0, y: 260.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 500.0, y: 170.0 }, Size { width: DEFAULT_SIZE, height: 180.0 }, false, None, None));
                self.objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 250.0, y: 320.0 }, Size { width: 250.0, height: DEFAULT_SIZE }, false, None, None));
                
                self.objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 302.0, y: 255.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 375.0, y: 200.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 455.0, y: 240.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
            },

            SimulatorType::Other => () 
        }
    }

    pub fn clear_all(&mut self) {
        self.objects.clear();
        self.enemies.clear();
        self.cameras.clear();
    }

    pub fn get_status(&self) -> &SimulationStatus {
        &self.status
    }

    pub fn set_status(&mut self, new_value: SimulationStatus) {
        self.status = new_value;
    }
}
