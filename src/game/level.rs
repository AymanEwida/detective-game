use crate::{game::{enemy::EnemyType, player::PlayerStatus}, library::utils::get_level_challenges, renderer::{color::Color, error::Result, render::{Render, Size}, vertice::Position}};

use super::{camera::Camera, character::Character, enemy::Enemy, player::Player};

pub const DEFAULT_SIZE: f32 = 30.0;
pub const DEFAULT_SIZE_FOR_HIDE_PLACE: Size = Size { width: 45.0, height: 65.0 };
pub const DEFAULT_SIZE_FOR_COLLECTABLE: Size = Size { width: 40.0, height: 40.0 };
pub const DEFAULT_SIZE_FOR_TELEPORT_DOOR: Size = Size { width: DEFAULT_SIZE + 20.0, height: 70.0 };
pub const DEFAULT_SIZE_FOR_EXIT_DOOR: Size = Size { width: 70.0, height: 60.0 };

pub trait GameObject<'a> {
    fn draw(&self, render: &mut Render<'a>) -> Result<()>;
    fn get_position(&self) -> Position;
    fn set_position(&mut self, new_position: Position);
    fn get_size(&self) -> Size;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ObjectLevelType {
    Wall,
    RegularDoor,
    LockedDoor,
    CodedDoor,
    TeleportDoor,
    HidePlace,
    Coin,
    CodePaper,
    ExitDoor,
    Camera,
    Key,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectLevel<'a> {
    object_type: ObjectLevelType,
    position: Position,
    size: Size,
    image: &'a str,
    flip: bool,
    scale: Option<f32>,
    rotate: Option<f32>
}

impl ObjectLevel<'_> {
    pub fn new(object_type: ObjectLevelType, position: Position, size: Size, flip: bool, scale: Option<f32>, rotate: Option<f32>) -> Self {
        let image_path = match object_type {
            ObjectLevelType::Wall => "assets/game/wall.jpg",
            ObjectLevelType::RegularDoor => "assets/game/regular-close-door.png",
            ObjectLevelType::LockedDoor => "assets/game/locked-door.png",
            ObjectLevelType::CodedDoor => "assets/game/coded-door.png",
            ObjectLevelType::TeleportDoor => "assets/game/teleport-door.webp",
            ObjectLevelType::HidePlace => "assets/game/hide-place1.webp",
            ObjectLevelType::Coin => "assets/game/coin.png",
            ObjectLevelType::CodePaper => "assets/game/code-paper.webp",
            ObjectLevelType::ExitDoor => "assets/game/exit-door.png",
            ObjectLevelType::Key => "assets/game/key.png",
            _ => ""
        };
        
        Self {
            object_type, 
            position, 
            size, 
            image: image_path,
            flip,
            scale,
            rotate
        }
    }
}

impl ObjectLevel<'_> {
    pub fn get_type(&self) -> ObjectLevelType {
        self.object_type
    }

    pub fn open_door(&mut self) {
        assert!(self.object_type == ObjectLevelType::RegularDoor, "object must be a door");

        self.image = "assets/game/regular-open-door.png";
    }

    pub fn close_door(&mut self) {
        assert!(self.object_type == ObjectLevelType::RegularDoor, "object must be a door");

        self.image = "assets/game/regular-close-door.png";
    }
}

impl<'a> GameObject<'a> for ObjectLevel<'a> {
    fn draw(&self, render: &mut Render<'a>) -> Result<()> {
        render.load_image(self.image, self.position, self.size, self.flip, self.scale, None, self.rotate)?;

        Ok(())
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, new_position: Position) {
        self.position = new_position
    }

    fn get_size(&self) -> Size {
        self.size
    }
}

// TODO: remove ObjectLevel struct and add Wall, Door, HidePlace, ... instead, also provide move_enemy func a slice of wall and doors only
#[derive(Debug)]
pub struct GameLevel<'a> {
    border_top_left: Position,
    border_size: Size,
    background_image: &'a str,
    current_level: u8,
    enemies: Vec<Enemy<'a>>,
    level_objects: Vec<ObjectLevel<'a>>,
    cameras: Vec<Camera<'a>>,
    challenges: Vec<String>,
    notoriety_level: u64
}

impl Default for GameLevel<'_> {
    fn default() -> Self {
        // TODO: do tutorial here

        let enemies = vec![];

        let level_objects = vec![];

        Self {
            border_top_left: Position { x: 50.0, y: 140.0 },
            border_size: Size { width: 1820.0, height: 737.0 },
            background_image: "assets/game/background.jpg",
            current_level: 0,
            enemies,
            level_objects,
            cameras: Vec::new(),
            challenges: Vec::new(),
            notoriety_level: 0,
        }
    }
}

impl<'a> GameLevel<'a> {
    pub fn get_boder_size(&self) -> Size {
        Size { width: self.border_size.width - (DEFAULT_SIZE * 2.0), height: self.border_size.height - (DEFAULT_SIZE * 2.0) }
    }

    pub fn get_boder_start_position(&self) -> Position {
        self.border_top_left + DEFAULT_SIZE
    }

    pub fn draw(&mut self, player: &mut Player<'a>, render: &mut Render<'a>) -> Result<()> {
        for (idx , challenge) in self.challenges.iter().enumerate() {
            render.display_text(challenge, Position { x: 50.0, y: 20.0 + (idx as f32 * 40.0) }, 0.5, None, Color::White).expect("Unable to display text");
        }

        render.display_text("notoriety level: 0", Position { x: 720.0, y: 60.0 }, 0.5, None, Color::White).expect("Unable to display text");

        render.display_text(&format!("level: {}", self.current_level), Position { x: 1580.0, y: 60.0 }, 0.5, None, Color::White).expect("Unable to display text");
        render.display_text(&format!("status: {}", player.get_status()), Position { x: 1580.0, y: 100.0 }, 0.5, None, Color::White).expect("Unable to display text");

        render.display_text("holding: nothing", Position { x: 50.0, y: 900.0 }, 0.5, None, Color::White).expect("Unable to display text");

        render.load_image(self.background_image, self.border_top_left, self.border_size, false, None, None, None)?;

        for num in 0..8 {
            // border top
            render.load_image("assets/game/wall.jpg", Position { x: self.border_top_left.x + num as f32 * (self.border_size.width / 8.0), y: self.border_top_left.y }, Size { width: self.border_size.width / 8.0, height: DEFAULT_SIZE }, false, None, None, None)?;
            
            // border bottom
            render.load_image("assets/game/wall.jpg", Position { x: self.border_top_left.x + num as f32 * (self.border_size.width / 8.0), y: self.border_top_left.y + self.border_size.height - DEFAULT_SIZE }, Size { width: self.border_size.width / 8.0, height: DEFAULT_SIZE }, false, None, None, None)?;
        }

        for num in 0..2 {
            // border right
            render.load_image("assets/game/wall.jpg", Position { x: self.border_top_left.x + self.border_size.width - DEFAULT_SIZE, y: self.border_top_left.y + ((num as f32 - 1.0) * DEFAULT_SIZE).abs() + num as f32 * (self.border_size.height / 2.0) }, Size { width: DEFAULT_SIZE, height: (self.border_size.height - 60.0) / 2.0 }, false, None, None, None)?;

            // border left
            render.load_image("assets/game/wall.jpg", Position { x: self.border_top_left.x, y: self.border_top_left.y + ((num as f32 - 1.0) * DEFAULT_SIZE).abs() + num as f32 * (self.border_size.height / 2.0) }, Size { width: DEFAULT_SIZE, height: (self.border_size.height - 60.0) / 2.0 }, false, None, None, None)?;
        }

        for level_object in self.level_objects.iter_mut() {
            let collided_enemies: Vec<&mut Enemy<'a>> = self.enemies.iter_mut().filter(| enemy | enemy.collide(level_object)).collect();

            match level_object.get_type() {
                ObjectLevelType::RegularDoor => {
                    if player.collide(level_object) || collided_enemies.len() > 0 {
                        level_object.open_door();
                    } else {
                        level_object.close_door();
                    }
                },

                ObjectLevelType::CodedDoor => (),

                ObjectLevelType::LockedDoor => (),

                ObjectLevelType::HidePlace | ObjectLevelType::CodePaper | ObjectLevelType::ExitDoor | ObjectLevelType::TeleportDoor => (),

                ObjectLevelType::Coin | ObjectLevelType::Key => (),

                _ => {
                    if player.collide(level_object) {
                        player.move_to_prev_position();
                    }

                    if collided_enemies.len() > 0 {
                        for collided_enemy in collided_enemies {
                            collided_enemy.move_to_prev_position();
                        }
                    }
                }
            }

            level_object.draw(render)?;
        }

        for camera in self.cameras.iter_mut() {
            camera.draw(render)?;
        }

        for enemy in self.enemies.iter_mut() {
            if enemy.is_off_window(render.get_size())
                || enemy.is_off_border(
                Some(self.border_top_left + DEFAULT_SIZE),
                Size { width: self.border_size.width - (DEFAULT_SIZE * 2.0), height: self.border_size.height - (DEFAULT_SIZE * 2.0) }
            ) {
                enemy.move_to_prev_position();
            }

            // TDOD: handle lose, win situation in level
            // if enemy.collide_with_player(&player) {
            //     self.status = LevelStatus::Lose;

            //     render.display_text("Lost", Position { x: 10.0, y: 500.0 }, 1.0, None, Color::White).expect("Unable to display text");
            // } else {
            //     render.display_text("Still playing", Position { x: 10.0, y: 500.0 }, 1.0, None, Color::White).expect("Unable to display text");
            // }

            enemy.draw(render)?;

            self.notoriety_level = enemy.move_enemy(
                player, 
                self.notoriety_level, 
                self.border_top_left + DEFAULT_SIZE, 
                Size { width: self.border_size.width - (DEFAULT_SIZE * 2.0), height: self.border_size.height - (DEFAULT_SIZE * 2.0) },
                &[],
                &[],
                &[]
            );
        }

        player.draw(render)?;

        Ok(())
    }
    
    fn insert_object(&mut self, mut object: ObjectLevel<'a>) {
        let start_position = self.get_boder_start_position();

        let object_position = object.get_position();
        object.set_position(object_position + start_position);

        self.level_objects.push(object);
    }

    fn insert_enemy(&mut self, mut enemy: Enemy<'a>) {
        let start_position = self.get_boder_start_position();

        let enemy_start_position = enemy.get_start_position();
        enemy._set_start_position(enemy_start_position + start_position);
        
        let enemy_position = enemy.get_position();
        enemy.set_position(enemy_position + start_position);

        self.enemies.push(enemy);
    }

    fn insert_camera(&mut self, mut camera: Camera<'a>) {
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

    pub fn load_level(&mut self, player: &mut Player<'a>) {
        self.challenges = get_level_challenges(self.current_level).expect("Unable to get level challenges");

        self.enemies.clear();
        self.level_objects.clear();
        self.cameras.clear();

        player.set_status(PlayerStatus::NotHidden);

        match self.current_level {
            1 => {
                player.move_to(Position { x: 90.0, y: 180.0 }, true);

                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 140.0, y: 10.0 }, "18d/0 13l/3000 13r/0 18u/0 6r/3000 6l/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 100.0, y: 295.0 }, "9l/6000 20r/3000 11l/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 405.0 }, "11d/0 15r/0 10d/0 5l/2000 9r/2000 21u/1000 19l/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 340.0, y: 30.0 }, "26d/3500 26u/3000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 380.0, y: 420.0 }, "25r/0 6u/2000 16d/2000 25l/0 9d/0 5l/2000 5r/0 19u/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 550.0, y: 257.0 }, "8r/6000 15l/4000 7r/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 470.0, y: 157.0 }, "16r/4000 16l/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 470.0, y: 5.0 }, "16r/3000 5d/0 16l/0 5u/5000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 730.0, y: 520.0 }, "18u/3000 4r/0 18d/0 4l/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 915.0, y: 90.0 }, "15d/2000 4l/0 15u/0 15l/0 15d/2000 4r/0 15u/0 15r/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 865.0, y: 335.0 }, "18d/3000 4r/0 18u/5000 4l/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 840.0, y: 0.0 }, "20r/4000 20l/6500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1050.0, y: 100.0 }, "15d/4000 15u/5000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1020.0, y: 520.0 }, "18u/6000 18d/0 3r/4000 3l/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1175.0, y: 520.0 }, "18u/6000 18d/0 3l/4500 3r/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1190.0, y: 100.0 }, "15d/3500 15u/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 730.0, y: 615.0 }, "20r/6000 20l/4000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1030.0, y: 615.0 }, "15r/6000 15l/5500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1310.0, y: 615.0 }, "18u/6000 18d/0 2l/4500 2r/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1310.0, y: 90.0 }, "20d/6000 20u/0 2r/5500 2l/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1450.0, y: 90.0 }, "20d/6000 20u/0 2l/4500 2r/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1450.0, y: 615.0 }, "18u/5500 18d/0 2r/6000 2l/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1140.0, y: 0.0 }, "25r/4000 25l/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1490.0, y: 0.0 }, "21r/4000 21l/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1580.0, y: 504.0 }, "12r/0 1d/4000 42u/3000 12l/0 41d/0", false));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 80.0, y: 0.0 }, Size { width: DEFAULT_SIZE, height: 100.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 78.0, y: 100.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 160.0 }, Size { width: 110.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 262.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 15.0, height: 70.0 }, false, None, None));
                for num in 0..2 {
                    self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 265.0, y: 70.0 + (num as f32 * 268.5) }, Size { width: DEFAULT_SIZE + 10.0, height: 268.5 }, false, None, None));
                }
                self.insert_object(ObjectLevel::new(ObjectLevelType::CodedDoor, Position { x: 255.0, y: 607.0 }, Size { width: 60.0, height: 70.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 225.0, y: 110.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 250.0 }, Size { width: 210.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 210.0, y: 250.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 130.0, y: 185.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 10.0, y: 200.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 53.0, y: 360.0 }, Size { width: 213.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 0.0, y: 360.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 180.0, y: 295.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 70.0, y: 295.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 60.0, y: 390.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 225.0, y: 400.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 220.0, y: 480.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 160.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 50.0, y: 512.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 577.0 }, Size { width: 100.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 62.0, y: 608.0 }, Size { width: 40.0, height: 70.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::CodePaper, Position { x: 5.0, y: 625.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 505.0, y: 580.0 }, Size { width: DEFAULT_SIZE, height: 96.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 535.0, y: 580.0 }, Size { width: 100.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 635.0, y: 580.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 690.0, y: 580.0 }, Size { width: DEFAULT_SIZE, height: 96.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Key, Position { x: 545.0, y: 630.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 687.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 690.0, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 360.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 687.0, y: 420.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 690.0, y: 480.0 }, Size { width: DEFAULT_SIZE, height: 100.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 435.0, y: 0.0 }, Size { width: DEFAULT_SIZE, height: 350.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 465.0, y: 320.0 }, Size { width: 165.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: 620.0, y: 320.0 }, Size { width: 80.0, height: DEFAULT_SIZE }, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 465.0, y: 220.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 520.0, y: 220.0 }, Size { width: 170.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 550.0, y: 255.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 465.0, y: 120.0 }, Size { width: 170.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 635.0, y: 120.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 550.0, y: 155.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 580.0, y: 55.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 520.0, y: 15.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 390.0, y: 50.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 305.0, y: 220.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 305.0, y: 402.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 375.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 550.0, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 570.0, y: 350.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 480.0, y: 440.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 797.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 930.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 720.0, y: 60.0 }, Size { width: 335.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1055.0, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1110.0, y: 0.0 }, Size { width: DEFAULT_SIZE, height: 520.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1107.0, y: 520.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1145.0, y: 15.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1000.0, y: 580.0 }, Size { width: 280.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1065.0, y: 130.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1000.0, y: 250.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1065.0, y: 370.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1030.0, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1140.0, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1195.0, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1250.0, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 520.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1175.0, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1205.0, y: 370.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1140.0, y: 250.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1205.0, y: 120.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1280.0, y: 60.0 }, Size { width: 425.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1705.0, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1285.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1447.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1575.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1640.0, y: 90.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1720.0, y: 230.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1555.0, y: 350.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1630.0, y: 514.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1640.0, y: 300.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 720.0, y: 580.0 }, Size { width: 110.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 827.0, y: 90.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 830.0, y: 150.0 }, Size { width: DEFAULT_SIZE, height: 460.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 720.0, y: 300.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 775.0, y: 300.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 785.0, y: 400.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 755.0, y: 520.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 785.0, y: 235.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 720.0, y: 110.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 860.0, y: 580.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 915.0, y: 580.0 }, Size { width: 85.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 970.0, y: 90.0 }, Size { width: DEFAULT_SIZE, height: 490.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 860.0, y: 300.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 915.0, y: 300.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 925.0, y: 90.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 860.0, y: 235.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 925.0, y: 400.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 990.0, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 900.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 780.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 730.0, y: 625.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1247.0, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1100.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1387.0, y: 90.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1390.0, y: 150.0 }, Size { width: DEFAULT_SIZE, height: 527.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1310.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1350.0, y: 450.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1280.0, y: 350.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1350.0, y: 250.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1280.0, y: 120.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1530.0, y: 90.0 }, Size { width: DEFAULT_SIZE, height: 517.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1527.0, y: 607.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1485.0, y: 100.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1420.0, y: 250.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1485.0, y: 350.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1420.0, y: 450.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1450.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1430.0, y: 360.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1560.0, y: 577.0 }, Size { width: 145.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: 1697.0, y: 575.0 }, Size { width: 70.0, height: DEFAULT_SIZE + 2.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::ExitDoor, Position { x: 1697.0, y: 620.0 }, DEFAULT_SIZE_FOR_EXIT_DOOR, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1600.0, y: 630.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
            },

            2 => {
                player.move_to(Position { x: 1777.0, y: 780.0 }, false);

                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1070.0, y: 615.0 }, "52r/3500 52l/4000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 960.0, y: 615.0 }, "10u/0 21r/5500 21l/0 10d/0 1r/6000 1l/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1270.0, y: 240.0 }, "11l/5500 2r/0 17d/4500 9r/0 17u/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 960.0, y: 240.0 }, "10r/4500 8d/6500 10l/0 8u/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 840.0, y: 615.0 }, "19u/0 2r/6000 2l/0 19d/0 2l/5500 2r/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 615.0 }, "35r/5000 35l/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 513.0 }, "21r/4500 21l/5500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 0.0 }, "4r/4000 4l/0 23d/6000 23u/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 205.0, y: 0.0 }, "5l/3500 13d/0 2r/5500 13u/0 3r/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 550.0, y: 330.0 }, "30r/4500 30l/5500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 685.0, y: 240.0 }, "16r/4000 16l/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 475.0, y: 615.0 }, "6r/0 19u/3000 16r/0 19d/0 2r/3500 24l/3000", false));
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
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1360.0, y: 577.0 }, Size { width: 400.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1650.0, y: 607.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1330.0, y: 510.0 }, Size { width: DEFAULT_SIZE, height: 97.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1230.0, y: 480.0 }, Size { width: 130.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1230.0, y: 510.0 }, Size { width: DEFAULT_SIZE, height: 97.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1030.0, y: 577.0 }, Size { width: 200.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1027.0, y: 607.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1275.0, y: 520.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1450.0, y: 582.0 }, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1110.0, y: 582.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 920.0, y: 480.0 }, Size { width: DEFAULT_SIZE, height: 197.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 950.0, y: 480.0 }, Size { width: 225.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1175.0, y: 480.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1330.0, y: 210.0 }, Size { width: DEFAULT_SIZE, height: 270.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 920.0, y: 210.0 }, Size { width: 410.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1117.0, y: 240.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1120.0, y: 300.0 }, Size { width: DEFAULT_SIZE, height: 180.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 920.0, y: 240.0 }, Size { width: DEFAULT_SIZE, height: 180.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 917.0, y: 420.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 950.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1050.0, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1110.0, y: 485.0 }, true, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1240.0, y: 415.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1290.0, y: 300.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1180.0, y: 240.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1200.0, y: 340.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 950.0, y: 390.0 }, Size { width: 115.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1065.0, y: 390.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1050.0, y: 240.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 980.0, y: 327.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 960.0, y: 250.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1105.0, y: 320.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1030.0, y: 422.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, Some(0.98), None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 985.0, y: 395.0 }, false, None, None, Some(4000)));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 605.0, y: 390.0 }, Size { width: 315.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 780.0, y: 420.0 }, Size { width: DEFAULT_SIZE, height: 187.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 777.0, y: 607.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 840.0, y: 417.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 880.0, y: 530.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 795.0, y: 550.0 }, true, None, Some(350.0)));

                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 475.0, y: 610.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 0.0, y: 577.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 65.0, y: 577.0 }, Size { width: 360.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 423.0, y: 607.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 425.0, y: 477.0 }, Size { width: DEFAULT_SIZE, height: 130.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 477.0 }, Size { width: 425.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 280.0, y: 507.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 375.0, y: 510.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 330.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 85.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 220.0, y: 582.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 120.0, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 230.0, y: 482.0 }, true, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::CodePaper, Position { x: 323.0, y: 532.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 5.0, y: 335.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 0.0, y: 300.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 65.0, y: 300.0 }, Size { width: 230.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 112.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 115.0, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 240.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 265.0, y: 0.0 }, Size { width: DEFAULT_SIZE, height: 300.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 145.0, y: 200.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 200.0, y: 200.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 150.0, y: 232.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 70.0, y: 235.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 0.0, y: 130.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 35.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: -20.0, y: 80.0 }, true, None, Some(80.0)));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 210.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 145.0, y: 135.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 245.0, y: 125.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Key, Position { x: 220.0, y: 250.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: 417.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 15.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 425.0, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 180.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::CodedDoor, Position { x: 438.0, y: 210.0 }, Size { width: 85.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 510.0, y: 210.0 }, Size { width: 410.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 510.0, y: 240.0 }, Size { width: DEFAULT_SIZE, height: 180.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 540.0, y: 390.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 545.0, y: 420.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 540.0, y: 300.0 }, Size { width: 325.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 865.0, y: 300.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 620.0, y: 328.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 800.0, y: 328.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 715.0, y: 305.0 }, true, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 640.0, y: 240.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 545.0, y: 235.0 }, Size { width: DEFAULT_SIZE + 20.0, height: 68.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 787.5, y: 238.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Key, Position { x: 600.0, y: 255.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 670.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 545.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 738.0, y: 500.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 625.0, y: 417.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 730.0, y: 430.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 450.0, y: 500.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 340.0, y: 415.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 160.0, y: 415.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 85.0, y: 327.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 470.0, y: 300.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 290.0, y: 220.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 385.0, y: 110.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 335.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 5.0, y: 435.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 345.0, y: 340.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                for num in 0..4 {
                    self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 455.0 + (num as f32 * 226.25), y: 90.0 }, Size { width: 226.25, height: DEFAULT_SIZE }, false, None, None));
                }

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 540.0, y: 147.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 675.0, y: 115.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 800.0, y: 147.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 607.5, y: 95.0 }, true, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 858.5, y: 95.0 }, true, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 920.0, y: 120.0 }, Size { width: DEFAULT_SIZE, height: 30.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 917.0, y: 150.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 980.0, y: 147.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1110.0, y: 147.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1240.0, y: 147.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1110.0, y: 95.0 }, false, None, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1180.0, y: 160.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1327.0, y: 150.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 475.0, y: 27.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 610.0, y: 27.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 745.0, y: 27.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 870.0, y: 27.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1005.0, y: 27.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1140.0, y: 27.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 610.0, y: -25.0 }, false, None, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 812.5, y: -25.0 }, true, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1005.0, y: -25.0 }, false, None, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1207.5, y: -25.0 }, true, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 940.0, y: 40.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 1270.0, y: 23.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1330.0, y: 0.0 }, Size { width: DEFAULT_SIZE, height: 150.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1360.0, y: 480.0 }, Size { width: 345.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1705.0, y: 480.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 1370.0, y: 510.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1430.0, y: 510.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 67.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1480.0, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1640.0, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1540.0, y: 485.0 }, true, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1590.0, y: 527.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1360.0, y: 120.0 }, Size { width: 175.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1535.0, y: 120.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::CodedDoor, Position { x: 1580.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 18.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1590.0, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 420.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1620.0, y: 180.0 }, Size { width: 85.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: 1697.0, y: 180.0 }, Size { width: 70.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1460.0, y: 150.0 }, Size { width: DEFAULT_SIZE, height: 270.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1457.0, y: 420.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1415.0, y: 150.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1360.0, y: 275.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1360.0, y: 415.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1440.0, y: 380.0 }, false, None, Some(25.0)));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1490.0, y: 150.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1545.0, y: 275.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1520.0, y: 415.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1460.0, y: 250.0 }, true, None, Some(80.0)));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1470.0, y: 55.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1360.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1365.0, y: 75.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1650.0, y: 415.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1715.0, y: 315.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1640.0, y: 210.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1590.0, y: 320.0 }, true, None, Some(80.0)));

                self.insert_object(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: 1613.0, y: 63.0 }, Size { width: 70.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1677.0, y: 63.0 }, Size { width: DEFAULT_SIZE - 10.0, height: 117.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::ExitDoor, Position { x: 1700.0, y: 0.0 }, DEFAULT_SIZE_FOR_EXIT_DOOR, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1627.0, y: 95.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1627.0, y: 115.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1627.0, y: 135.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
            },

            3 => {
                player.move_to(Position { x: 1787.0, y: 170.0 }, false);

                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1525.0, y: 0.0 }, "13d/0 15r/0 1d/3500 1u/0 15l/0 13u/0 5r/6000 5l/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1680.0, y: 480.0 }, "25u/5500 25d/0 2l/3500 2r/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 840.0, y: 0.0 }, "13d/0 4l/3500 4r/0 13u/0 2l/3000 3r/3000 1l/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 585.0, y: 0.0 }, "13d/0 4r/3500 4l/0 13u/0 1r/3000 3l/3000 2r/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 0.0 }, "45r/2500 45l/5500", true));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 342.0, y: 130.0 }, "33l/0 3u/3000 3d/0 33r/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 950.0, y: 0.0 }, "3r/0 13d/0 2r/6500 2l/0 13u/0 3l/4500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1145.0, y: 0.0 }, "3l/0 13d/0 2l/6000 2r/0 13u/0 3r/5000", true));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1250.0, y: 0.0 }, "18r/0 13d/0 8l/4000 8r/0 13u/0 18l/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1400.0, y: 310.0 }, "1r/0 30d/0 1r/4500 1l/0 30u/0 1l/6500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1295.0, y: 310.0 }, "1l/0 30d/0 1l/6000 1r/0 30u/0 1r/4000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1074.0, y: 220.0 }, "10r/3500 20d/5500 10l/0 20u/0", true));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 970.0, y: 615.0 }, "20r/4500 20l/6500", true));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 970.0, y: 220.0 }, "29d/3500 29u/0 1l/5500 1r/0", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 834.0, y: 310.0 }, "2r/0 20d/6000 20u/0 2l/4500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 729.0, y: 310.0 }, "2l/0 20d/4000 20u/0 2r/5500", true));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 130.0, y: 220.0 }, "46r/3500 46l/5500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 604.0, y: 320.0 }, "20d/0 2l/4000 2r/0 20u/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 421.0, y: 520.0 }, "6r/4500 17u/3000 5l/0 17d/0 1l/5000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 330.0, y: 615.0 }, "35r/4000 35l/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 221.0, y: 310.0 }, "9r/0 11d/3500 8l/0 11u/0 1l/5500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 110.0, y: 310.0 }, "10l/0 20d/5500 9r/0 20u/0 1r/3500", false));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1639.0, y: 60.0 }, Size { width: 120.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1636.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1489.0, y: 0.0 }, Size { width: DEFAULT_SIZE, height: 190.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1489.0, y: 190.0 }, Size { width: 160.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1619.0, y: 220.0 }, Size { width: DEFAULT_SIZE, height: 230.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1519.0, y: 450.0 }, Size { width: 130.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1489.0, y: 450.0 }, Size { width: DEFAULT_SIZE, height: 120.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1519.0, y: 540.0 }, Size { width: 240.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1616.0, y: 480.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 1530.0, y: 475.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1525.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1574.0, y: 128.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1714.0, y: 210.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1649.0, y: 325.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1679.0, y: 478.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1709.0, y: 100.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 660.0, y: 60.0 }, Size { width: 150.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 710.0, y: -5.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 656.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 777.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));

                for num in 0..5 {
                    self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0 + (num as f32 * 297.8), y: 190.0 }, Size { width: 297.8, height: DEFAULT_SIZE }, false, None, None));
                }

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 840.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 865.0, y: 125.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 585.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 550.0, y: 125.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 715.0, y: 65.0 }, false, None, None, Some(4000)));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 695.0, y: 120.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 735.0, y: 120.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 907.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 910.0, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 130.0 }, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 980.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 940.0, y: 125.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1035.0, y: 80.0 }, false, None, Some(45.0)));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1060.0, y: 0.0 }, Size { width: DEFAULT_SIZE, height: 130.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1057.0, y: 130.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1140.0, y: 125.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1090.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1150.0, y: -25.0 }, true, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1207.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1210.0, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 130.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1280.0, y: -2.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1365.0, y: 10.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1240.0, y: 60.0 }, Size { width: 194.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1434.0, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1424.0, y: 125.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1380.0, y: 65.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1310.0, y: 90.0 }, Size { width: DEFAULT_SIZE, height: 40.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1307.0, y: 130.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 1250.0, y: 125.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Key, Position { x: 1245.0, y: 90.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 516.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 0.0, y: 60.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 65.0, y: 60.0 }, Size { width: 485.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 520.0, y: 90.0 }, Size { width: DEFAULT_SIZE, height: 100.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 440.0, y: -2.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 260.0, y: -2.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 75.0, y: -2.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 380.0, y: -25.0 }, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 200.0, y: -25.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 410.0, y: 90.0 }, Size { width: DEFAULT_SIZE, height: 40.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 407.0, y: 130.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 460.0, y: 125.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 35.0, y: 127.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 190.0, y: 127.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 342.0, y: 127.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 190.0, y: 65.0 }, false, None, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 270.0, y: 140.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1489.0, y: 220.0 }, Size { width: DEFAULT_SIZE, height: 230.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 1550.0, y: 385.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 1550.0, y: 215.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));
                
                self.insert_camera(Camera::new_without_repeat(Position { x: 1490.0, y: 310.0 }, true, Some(0.99), Some(100.0)));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1555.0, y: 340.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::CodePaper, Position { x: 1555.0, y: 290.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: 1229.0, y: 220.0 }, Size { width: DEFAULT_SIZE + 10.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1234.0, y: 280.0 }, Size { width: 200.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::CodedDoor, Position { x: 1424.0, y: 280.0 }, Size { width: 75.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 1330.0, y: 215.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1357.0, y: 310.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1360.0, y: 370.0 }, Size { width: DEFAULT_SIZE, height: 310.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1489.0, y: 570.0 }, Size { width: DEFAULT_SIZE, height: 40.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1486.0, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 1630.0, y: 615.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1444.0, y: 320.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1390.0, y: 450.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1420.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1550.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1620.0, y: 545.0 }, false, None, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1715.0, y: 635.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1715.0, y: 575.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1234.0, y: 310.0 }, Size { width: DEFAULT_SIZE, height: 300.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1231.0, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1264.0, y: 310.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1315.0, y: 450.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1315.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1340.0, y: 555.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1034.0, y: 220.0 }, Size { width: DEFAULT_SIZE, height: 360.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1064.0, y: 490.0 }, Size { width: 115.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1179.0, y: 490.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 1084.0, y: 535.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, Some(0.92), None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1014.0, y: 580.0 }, Size { width: 220.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1140.0, y: 220.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1064.0, y: 320.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1104.0, y: 430.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1214.0, y: 400.0 }, false, None, Some(25.0)));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1140.0, y: 495.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1150.0, y: 330.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1135.0, y: 535.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1175.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1065.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1025.0, y: 585.0 }, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 949.0, y: 580.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 916.0, y: 220.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 919.0, y: 280.0 }, Size { width: DEFAULT_SIZE, height: 400.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 669.0, y: 280.0 }, Size { width: 250.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 669.0, y: 220.0 }, Size { width: DEFAULT_SIZE, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 699.0, y: 210.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, Some(0.92), None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 989.0, y: 505.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 949.0, y: 360.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 970.0, y: 220.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 851.0, y: 220.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 780.0, y: 195.0 }, false, None, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 669.0, y: 310.0 }, Size { width: DEFAULT_SIZE, height: 300.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 699.0, y: 580.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 764.0, y: 580.0 }, Size { width: 100.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 864.0, y: 580.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 764.0, y: 610.0 }, Size { width: DEFAULT_SIZE, height: 70.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 794.0, y: 640.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, Some(0.92), None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 824.0, y: 510.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 874.0, y: 380.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 870.0, y: 285.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 791.0, y: 310.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 794.0, y: 370.0 }, Size { width: DEFAULT_SIZE, height: 210.0 }, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 725.0, y: 310.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 749.0, y: 450.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 280.0 }, Size { width: 614.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 614.0, y: 280.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 80.0, y: 220.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 10.0, y: 215.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 155.0, y: 220.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 335.0, y: 220.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 500.0, y: 220.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 250.0, y: 195.0 }, false, None, None, Some(4500)));
                self.insert_camera(Camera::new_without_repeat(Position { x: 560.0, y: 195.0 }, true, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 544.0, y: 310.0 }, Size { width: DEFAULT_SIZE, height: 210.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 541.0, y: 520.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 574.0, y: 370.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 600.0, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 379.0, y: 310.0 }, Size { width: DEFAULT_SIZE, height: 210.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::CodedDoor, Position { x: 363.0, y: 520.0 }, Size { width: DEFAULT_SIZE + 30.0, height: 60.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 469.0, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 409.0, y: 330.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 494.0, y: 320.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 494.0, y: 370.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 359.0, y: 580.0 }, Size { width: 310.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: 299.0, y: 580.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 274.0, y: 580.0 }, Size { width: DEFAULT_SIZE, height: 97.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::CodedDoor, Position { x: 199.0, y: 580.0 }, Size { width: 85.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 179.0, y: 370.0 }, Size { width: DEFAULT_SIZE, height: 240.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 209.0, y: 490.0 }, Size { width: 97.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: 299.0, y: 490.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 359.0, y: 490.0 }, Size { width: 20.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 260.0, y: 530.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 640.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 384.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 560.0, y: 585.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 520.0, y: 630.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 176.0, y: 310.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 234.0, y: 425.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 280.0, y: 310.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 245.0, y: 285.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 329.0, y: 320.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: -8.0, y: 580.0 }, Size { width: 75.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 60.0, y: 580.0 }, Size { width: 119.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 120.0, y: 310.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 0.0, y: 380.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 80.0, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 120.0, y: 420.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::CodedDoor, Position { x: 80.0, y: 608.0 }, Size { width: DEFAULT_SIZE + 30.0, height: 70.0 }, false, None, None));

                self.insert_camera(Camera::new_without_repeat(Position { x: 160.0, y: 585.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::ExitDoor, Position { x: 0.0, y: 620.0 }, DEFAULT_SIZE_FOR_EXIT_DOOR, false, None, None));
            },

            4 => {
                player.move_to(Position { x: 90.0, y: 780.0 }, true);

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
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1695.0, y: 100.0 }, "5d/0 5l/0 15d/5500 15u/0 5r/0 5u/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1685.0, y: 0.0 }, "32l/3500 32r/5500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 823.0, y: 0.0 }, "37r/3000 37l/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 812.5, y: 100.0 }, "1d/0 3r/0 8d/3500 17r/2500 8u/0 20l/0 1u/4500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1490.0, y: 520.0 }, "46l/5500 46r/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1460.0, y: 90.0 }, "5r/0 33d/6000 33u/0 5l/4000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1355.0, y: 90.0 }, "10d/3000 21l/0 7u/3000 4r/0 3u/0 17r/5000", true));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1365.0, y: 291.0 }, "14d/0 4l/4500 4r/0 14u/6500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1200.0, y: 281.0 }, "1r/0 15d/0 2r/6000 2l/0 15u/0 1l/4000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1095.0, y: 281.0 }, "15d/0 2l/3500 9u/0 8l/0 4u/0 1l/3500 2u/0 11r/4500", true));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 580.0 }, Size { width: 360.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 87.0, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 360.0, y: 580.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                
                for num in 0..2 {
                    self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 415.0 + (num as f32 * 281.25), y: 580.0 }, Size { width: 281.25, height: DEFAULT_SIZE }, false, None, None));
                }
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 542.0, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));
               
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 162.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 390.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 227.0, y: 585.0 }, true, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 455.0, y: 585.0 }, true, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 944.5, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 610.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 745.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 870.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 750.0, y: 585.0 }, false, None, None, Some(4000)));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 680.0, y: 630.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 977.5, y: 580.0 }, Size { width: 80.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1057.5, y: 580.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1112.5, y: 580.0 }, Size { width: 482.5, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1565.0, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1565.0, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 520.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1127.5, y: 60.0 }, Size { width: 437.5, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 977.5, y: 251.0 }, Size { width: 120.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1097.5, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 221.0 }, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1039.5, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1224.5, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1480.5, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1132.5, y: 585.0 }, false, None, None, Some(5000)));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1300.0, y: 585.0 }, true, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1355.0, y: 630.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1595.0, y: 60.0 }, Size { width: 110.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1705.0, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1655.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1595.0, y: 450.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1715.0, y: 345.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1595.0, y: 240.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1715.0, y: 115.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1605.0, y: 100.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1710.0, y: 540.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 802.5, y: 60.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 867.5, y: 60.0 }, Size { width: 230.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1630.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1453.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1288.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1131.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 926.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1455.0, y: -25.0 }, false, None, None, Some(5500)));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1133.0, y: -25.0 }, false, None, None, Some(4000)));
                self.insert_camera(Camera::new_without_repeat(Position { x: 866.0, y: -25.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1373.0, y: 10.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1031.0, y: 10.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 802.5, y: 110.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 922.5, y: 90.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1052.5, y: 110.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 937.5, y: 186.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Key, Position { x: 1047.5, y: 201.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 812.5, y: 201.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 997.5, y: 125.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 977.5, y: 490.0 }, Size { width: 532.5, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1510.0, y: 490.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1132.5, y: 518.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1332.5, y: 518.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1492.5, y: 518.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 997.5, y: 530.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1217.5, y: 495.0 }, true, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1397.5, y: 495.0 }, true, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1417.0, y: 90.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1420.0, y: 150.0 }, Size { width: DEFAULT_SIZE, height: 340.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1450.0, y: 425.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1520.0, y: 310.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1450.0, y: 215.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1485.0, y: 90.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1460.0, y: 325.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1127.5, y: 251.0 }, Size { width: 237.5, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1365.0, y: 251.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1335.0, y: 90.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1230.0, y: 90.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1295.0, y: 186.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::CodePaper, Position { x: 1137.5, y: 100.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1137.5, y: 201.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1290.0, y: 281.0 }, Size { width: DEFAULT_SIZE, height: 149.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1287.0, y: 430.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1320.0, y: 295.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1375.0, y: 425.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1400.0, y: 375.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1157.0, y: 281.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1160.0, y: 351.0 }, Size { width: DEFAULT_SIZE, height: 139.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1215.0, y: 425.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1245.0, y: 281.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1225.0, y: 365.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 977.5, y: 400.0 }, Size { width: 90.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1032.5, y: 430.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1085.0, y: 281.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1092.5, y: 426.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1002.5, y: 335.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 982.5, y: 440.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1032.5, y: 256.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 802.5, y: 251.0 }, Size { width: 40.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: 832.5, y: 251.0 }, Size { width: 83.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 907.5, y: 251.0 }, Size { width: 40.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 772.5, y: 0.0 }, Size { width: DEFAULT_SIZE, height: 291.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 769.5, y: 291.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 772.5, y: 361.0 }, Size { width: DEFAULT_SIZE, height: 40.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 802.5, y: 371.0 }, Size { width: 40.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: 832.5, y: 371.0 }, Size { width: 83.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 907.5, y: 371.0 }, Size { width: 40.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 947.5, y: 251.0 }, Size { width: DEFAULT_SIZE, height: 40.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::CodedDoor, Position { x: 930.5, y: 291.0 }, Size { width: DEFAULT_SIZE + 30.0, height: 70.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 947.5, y: 361.0 }, Size { width: DEFAULT_SIZE, height: 219.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::ExitDoor, Position { x: 840.0, y: 295.0 }, DEFAULT_SIZE_FOR_EXIT_DOOR, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 545.0, y: 361.0 }, Size { width: DEFAULT_SIZE, height: 219.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 575.0, y: 361.0 }, Size { width: 197.5, height: DEFAULT_SIZE + 10.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 0.0, y: 490.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 55.0, y: 490.0 }, Size { width: 490.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 295.0, y: 518.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 65.0, y: 518.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 200.0, y: 495.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 465.0, y: 530.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 0.0, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                
                for num in 0..2 {
                    self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 55.0 + (num as f32 * 331.25), y: 60.0 }, Size { width: 331.25, height: DEFAULT_SIZE }, false, None, None));
                }

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 717.5, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 75.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 230.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 445.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 600.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 152.5, y: -25.0 }, false, None, None, Some(4000)));
                self.insert_camera(Camera::new_without_repeat(Position { x: 317.0, y: -25.0 }, true, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 522.5, y: -25.0 }, false, None, None, Some(4000)));
                self.insert_camera(Camera::new_without_repeat(Position { x: 675.0, y: -25.0 }, true, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 377.0, y: 10.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 652.5, y: 151.0 }, Size { width: 120.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 649.5, y: 181.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 704.5, y: 186.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 662.5, y: 91.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 522.0, y: 90.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 380.0, y: 90.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 510.0, y: 186.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Key, Position { x: 390.0, y: 201.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 450.0, y: 155.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 580.0, y: 155.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 575.0, y: 490.0 }, Size { width: 120.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 662.0, y: 520.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 595.0, y: 515.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 740.0, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 640.0, y: 425.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 902.5, y: 450.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 770.0, y: 396.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 585.0, y: 430.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 850.0, y: 530.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 157.0, y: 90.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 160.0, y: 150.0 }, Size { width: DEFAULT_SIZE, height: 340.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 75.0, y: 425.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 50.0, y: 305.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 280.0 }, Size { width: 105.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 105.0, y: 280.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 40.0, y: 215.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 77.0, y: 85.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 190.0, y: 280.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 245.0, y: 280.0 }, Size { width: 105.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 230.0, y: 85.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 265.0, y: 215.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));                      
                self.insert_camera(Camera::new_without_repeat(Position { x: 185.0, y: 200.0 }, true, None, Some(330.0)));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 300.0, y: 160.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 350.0, y: 90.0 }, Size { width: DEFAULT_SIZE, height: 340.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 347.0, y: 430.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 255.0, y: 305.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 225.0, y: 425.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 230.0, y: 380.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 380.0, y: 251.0 }, Size { width: 392.5, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 435.0, y: 425.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 400.0, y: 276.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 550.0, y: 296.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 650.0, y: 296.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 600.0, y: 256.0 }, false, None, None, Some(5000)));
                self.insert_camera(Camera::new_without_repeat(Position { x: 695.0, y: 256.0 }, true, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 495.0, y: 440.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 505.0, y: 310.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 715.0, y: 310.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
            },

            5 => {
                player.move_to(Position { x: 930.0, y: 460.0 }, false);

                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 883.0, y: 615.0 }, "3l/0 21u/5500 21d/0 3r/3500", true));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 990.0, y: 615.0 }, "27r/4000 27l/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1369.0, y: 615.0 }, "31r/3000 31l/5000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1647.0, y: 401.0 }, "5r/0 11d/4000 11u/0 5l/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1492.0, y: 520.0 }, "2r/0 12u/0 3r/6000 3l/0 12d/0 2l/4000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 977.0, y: 510.0 }, "1d/0 40r/6500 40l 1u/4500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1280.0, y: 420.0 }, "28l/5500 28r/3500", true));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1098.0, y: 261.0 }, "4d/0 11l/5000 11r/0 4u/3000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1193.0, y: 151.0 }, "10r/0 4u/2000 2l/0 2u/0 18l/0 6d/2500 10l/0 4u/2000 2r/0 2u/0 18r/0 6d/2500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1290.0, y: 301.0 }, "10l/0 4u/5500 4d/0 10r/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1400.0, y: 301.0 }, "13r/0 10u/0 11l/3000 11r/0 10d/0 1r/3500 14l/5500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1408.0, y: 90.0 }, "12r/4000 12l/4000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1650.0, y: 301.0 }, "2r/0 7u/4000 7d/0 2l/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1705.0, y: 100.0 }, "2d/0 3l/0 4d/4000 4u/0 3r/0 2u/6000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1364.0, y: 0.0 }, "32r/6500 32l/4500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 990.0, y: 0.0 }, "27r/5000 27l/3000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 485.0, y: 0.0 }, "28r/5500 28l/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 313.0, y: 60.0 }, "3r/0 6u/0 4r/6000 4l/0 6d/0 3l/5000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 152.0, y: 0.0 }, "3r/0 6d/0 4r/5500 4l/0 6u/0 3l/4500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 698.0, y: 291.0 }, "2l/0 16d/0 4r/0 6d/3500 6u/0 4l/0 16u/0 2r/5500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 488.0, y: 615.0 }, "20r/6000 20l/4000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 478.0, y: 510.0 }, "8r/0 3u/3000 3l/0 2u/0 17l/0 2d/3500 2u/0 12r/0 5d/3000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 325.0, y: 615.0 }, "2r/0 6u/3500 6d/0 2l/5500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 615.0 }, "21r/5500 21l/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 0.0, y: 380.0 }, "13d/3500 5r/0 1d/0 1r/3500 13u/0 6l/0 1u/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 160.0, y: 520.0 }, "3r/0 15u/0 24r/3500 24l/0 15d/0 3l/5500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 815.0, y: 90.0 }, "3r/0 9d/6000 9u/0 3l/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 705.0, y: 90.0 }, "21l/4500 21r/6500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 483.0, y: 191.0 }, "21r/4000 21l/5000", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 323.0, y: 150.0 }, "1r/0 3d/0 5r/3500 2l/0 10d/0 2r/3000 5l/0 13u/0 1l/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 162.0, y: 280.0 }, "3r/0 13u/0 3r/5500 3l/0 13d/0 3l/3500", false));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 0.0, y: 100.0 }, "17d/3500 5r/0 1d/0 1r/3500 1l/0 15u/0 5l/0 3u/3500", false));

                // top
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 802.5, y: 251.0 }, Size { width: 40.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 842.5, y: 251.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 907.5, y: 251.0 }, Size { width: 40.0, height: DEFAULT_SIZE }, false, None, None));

                // left
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 772.5, y: 251.0 }, Size { width: DEFAULT_SIZE, height: 40.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 769.5, y: 291.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 772.5, y: 361.0 }, Size { width: DEFAULT_SIZE, height: 40.0 }, false, None, None));

                // bottom
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 802.5, y: 371.0 }, Size { width: 40.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 842.5, y: 371.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 907.5, y: 371.0 }, Size { width: 40.0, height: DEFAULT_SIZE }, false, None, None));

                // right
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 947.5, y: 251.0 }, Size { width: DEFAULT_SIZE, height: 40.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 944.5, y: 291.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 947.5, y: 361.0 }, Size { width: DEFAULT_SIZE, height: 40.0 }, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 772.5, y: 401.0 }, Size { width: DEFAULT_SIZE, height: 279.0 }, false, None, None));
                
                for num in 0..2 {
                    self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 977.5 + (num as f32 * 391.25), y: 361.0 }, Size { width: 391.25, height: DEFAULT_SIZE + 10.0 }, false, None, None));
                }

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 947.5, y: 401.0 }, Size { width: DEFAULT_SIZE, height: 209.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 944.5, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 802.5, y: 411.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 902.5, y: 494.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 852.5, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 812.5, y: 514.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 977.5, y: 580.0 }, Size { width: 727.5, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1705.0, y: 580.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 999.5, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1129.125, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1258.75, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1131.125, y: 585.0 }, false, None, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1323.75, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1388.75, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1630.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1453.75, y: 585.0 }, true, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1533.75, y: 630.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1602.0, y: 401.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1605.0, y: 471.0 }, Size { width: DEFAULT_SIZE, height: 109.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1635.0, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1675.0, y: 401.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1450.0, y: 401.0 }, Size { width: DEFAULT_SIZE, height: 109.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1447.0, y: 510.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1520.0, y: 401.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1560.0, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1585.0, y: 475.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1490.0, y: 455.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 977.5, y: 480.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1032.5, y: 480.0 }, Size { width: 417.5, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1382.0, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1207.0, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 997.0, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1300.0, y: 485.0 }, false, None, None, Some(4000)));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1147.0, y: 485.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1299.5, y: 530.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1348.0, y: 401.0 }, Size { width: DEFAULT_SIZE, height: 10.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1345.0, y: 411.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 1390.0, y: 415.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1052.0, y: 415.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1280.0, y: 415.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1160.0, y: 376.0 }, false, None, None, Some(4500)));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1190.0, y: 430.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 947.5, y: 90.0 }, Size { width: DEFAULT_SIZE, height: 161.0 }, false, None, None));
                
                for num in 0..3 {
                    self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 947.5 + (num as f32 * 252.5), y: 60.0 }, Size { width: 252.5, height: DEFAULT_SIZE }, false, None, None));
                }
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1007.5, y: 296.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1107.5, y: 261.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 977.5, y: 221.0 }, Size { width: 120.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1097.5, y: 221.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1022.5, y: 156.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1094.5, y: 90.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 987.5, y: 100.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1149.5, y: 90.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1152.5, y: 150.0 }, Size { width: DEFAULT_SIZE, height: 211.0 }, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1194.5, y: 90.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1267.5, y: 156.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1259.5, y: 65.0 }, true, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Key, Position { x: 1307.5, y: 100.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1182.5, y: 221.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1247.5, y: 221.0 }, Size { width: 110.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1277.5, y: 251.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1182.5, y: 296.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1357.5, y: 90.0 }, Size { width: DEFAULT_SIZE, height: 201.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1354.5, y: 291.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));
            
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1387.5, y: 261.0 }, Size { width: 140.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: 1517.5, y: 261.0 }, Size { width: 85.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1592.5, y: 261.0 }, Size { width: 47.5, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1605.0, y: 291.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1399.5, y: 296.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1537.5, y: 296.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1467.5, y: 266.0 }, false, None, None, Some(4500)));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1610.0, y: 90.0 }, Size { width: DEFAULT_SIZE, height: 171.0 }, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1462.5, y: 196.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1442.5, y: 90.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1565.0, y: 160.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1397.5, y: 211.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1397.5, y: 100.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1560.0, y: 100.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1705.0, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
            
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1705.0, y: 296.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1715.0, y: 181.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1640.0, y: 90.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1318.75, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1640.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1505.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1363.75, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 1510.0, y: -25.0 }, false, None, None, Some(4000)));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 944.5, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1203.75, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 989.5, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 1140.0, y: -25.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1080.0, y: 10.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 829.5, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 884.5, y: -5.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 472.5, y: 60.0 }, Size { width: 475.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 439.5, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 764.5, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 629.5, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 489.5, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 634.5, y: -25.0 }, false, None, None, Some(4000)));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 564.5, y: 10.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 442.5, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 90.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 150.0, y: 120.0 }, Size { width: 292.5, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 373.25, y: 58.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 321.25, y: 10.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 281.25, y: 0.0 }, Size { width: DEFAULT_SIZE, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 278.25, y: 60.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 160.0, y: 58.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 231.25, y: 10.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 117.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 0.0, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 55.0, y: 60.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::ExitDoor, Position { x: 30.0, y: 0.0 }, DEFAULT_SIZE_FOR_EXIT_DOOR, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 652.5, y: 251.0 }, Size { width: 120.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 622.5, y: 251.0 }, Size { width: DEFAULT_SIZE, height: 359.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 687.5, y: 281.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 652.5, y: 396.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 652.5, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 642.5, y: 470.0 }, true, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 722.5, y: 462.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 652.5, y: 580.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 717.5, y: 580.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 697.5, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 487.5, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 595.5, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 597.5, y: 585.0 }, false, None, None, Some(3500)));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 467.5, y: 580.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 522.5, y: 580.0 }, Size { width: 100.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 542.5, y: 518.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 479.5, y: 455.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::CodePaper, Position { x: 582.5, y: 460.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 282.5, y: 430.0 }, Size { width: 340.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 434.5, y: 460.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 437.5, y: 520.0 }, Size { width: DEFAULT_SIZE, height: 160.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 354.5, y: 455.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 387.5, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 417.5, y: 565.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 322.5, y: 552.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 282.5, y: 460.0 }, Size { width: DEFAULT_SIZE, height: 150.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 279.5, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 214.5, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 105.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_without_repeat(Position { x: 65.0, y: 585.0 }, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 0.0, y: 580.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 55.0, y: 580.0 }, Size { width: 227.5, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 65.0, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 0.0, y: 440.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 75.0, y: 370.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 0.0, y: 340.0 }, Size { width: 55.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 55.0, y: 340.0 }, Size { width: 95.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 150.0, y: 340.0 }, Size { width: 472.5, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 120.0, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 220.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 117.0, y: 280.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 120.0, y: 340.0 }, Size { width: DEFAULT_SIZE, height: 170.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::CodedDoor, Position { x: 105.0, y: 510.0 }, Size { width: DEFAULT_SIZE + 30.0, height: 70.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 190.0, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 150.0, y: 370.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 302.5, y: 370.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 226.25, y: 345.0 }, false, None, None, Some(4500)));
                self.insert_camera(Camera::new_without_repeat(Position { x: 367.5, y: 345.0 }, true, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 582.5, y: 370.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 582.5, y: 380.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 582.5, y: 390.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 487.5, y: 370.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 532.5, y: 365.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 769.5, y: 90.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 772.5, y: 150.0 }, Size { width: DEFAULT_SIZE, height: 101.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 902.5, y: 186.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 902.5, y: 90.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 814.5, y: 90.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 802.5, y: 191.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 472.5, y: 150.0 }, Size { width: 40.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 512.5, y: 150.0 }, Size { width: 65.0, height: DEFAULT_SIZE }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 577.5, y: 150.0 }, Size { width: 195.0, height: DEFAULT_SIZE }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 714.5, y: 85.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 587.5, y: 85.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 472.5, y: 100.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 664.5, y: 65.0 }, true, None, None, Some(6000)));
                self.insert_camera(Camera::new_without_repeat(Position { x: 537.5, y: 65.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 442.5, y: 150.0 }, Size { width: DEFAULT_SIZE, height: 31.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 439.5, y: 181.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 442.5, y: 251.0 }, Size { width: 180.0, height: DEFAULT_SIZE - 2.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 492.5, y: 186.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 607.5, y: 186.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Key, Position { x: 722.5, y: 201.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 672.5, y: 155.0 }, false, None, None, Some(3500)));

                self.insert_object(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: 434.5, y: 277.0 }, Size { width: DEFAULT_SIZE + 20.0, height: 65.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 353.25, y: 150.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 311.25, y: 230.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 379.5, y: 275.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 489.5, y: 280.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_camera(Camera::new_with_repeat(Position { x: 534.5, y: 256.0 }, false, None, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 572.5, y: 280.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 572.5, y: 300.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 278.25, y: 150.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 281.25, y: 210.0 }, Size { width: DEFAULT_SIZE, height: 130.0 }, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 193.25, y: 150.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 226.25, y: 275.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 150.0, y: 225.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 65.0, y: 275.0 }, DEFAULT_SIZE_FOR_TELEPORT_DOOR, false, None, None));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 0.0, y: 265.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 80.0, y: 180.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 60.0, y: 85.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 10.0, y: 205.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 10.0, y: 110.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 150.0, y: 225.0 }, DEFAULT_SIZE_FOR_COLLECTABLE, false, None, None));
            },

            _ => ()
        }
    }
}
