use crate::{game::enemy::EnemyType, renderer::{error::Result, render::{Render, Size}, vertice::Position}};

use super::{character::Character, enemy::Enemy, player::Player};

pub const DEFAULT_SIZE: f32 = 30.0;
pub const DEFAULT_SIZE_FOR_HIDE_PLACE: Size = Size { width: 45.0, height: 65.0 };
pub const DEFAULT_SIZE_FOR_COLLECTABLE: Size = Size { width: 40.0, height: 40.0 };
pub const DEFAULT_SIZE_FOR_CAMERA: Size = Size { width: 30.0, height: 30.0 };

pub trait GameObject<'a> {
    fn draw(&self, render: &mut Render<'a>) -> Result<()>;
    fn get_position(&self) -> Position;
    fn get_size(&self) -> Size;
}

#[derive(PartialEq)]
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
    Key
}

pub struct ObjectLevel<'a> {
    object_type: ObjectLevelType,
    position: Position,
    size: Size,
    image: &'a str,
}

impl ObjectLevel<'_> {
    pub fn new(object_type: ObjectLevelType, position: Position, size: Size) -> Self {
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
            ObjectLevelType::Camera => "assets/game/camera.png",
            ObjectLevelType::Key => "assets/game/key.png",
        };
        
        Self {
            object_type, 
            position, 
            size, 
            image: image_path
        }
    }
}

impl ObjectLevel<'_> {
    fn open_door(&mut self) {
        assert!(self.object_type == ObjectLevelType::RegularDoor, "object must be a door");

        self.image = "assets/game/regular-open-door.png";
    }

    fn close_door(&mut self) {
        assert!(self.object_type == ObjectLevelType::RegularDoor, "object must be a door");

        self.image = "assets/game/regular-close-door.png";
    }
}

impl<'a> GameObject<'a> for ObjectLevel<'a> {
    fn draw(&self, render: &mut Render<'a>) -> Result<()> {
        render.load_image(self.image, self.position, self.size, None)?;

        Ok(())
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn get_size(&self) -> Size {
        self.size
    }
}

pub struct GameLevel<'a> {
    border_top_left: Position,
    border_size: Size,
    background_image: &'a str,
    current_level: u8,
    enemies: Vec<Enemy<'a>>,
    level_objects: Vec<ObjectLevel<'a>>,
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
        render.load_image(self.background_image, self.border_top_left, self.border_size, None)?;

        for num in 0..8 {
            // border top
            render.load_image("assets/game/wall.jpg", Position { x: self.border_top_left.x + num as f32 * (self.border_size.width / 8.0), y: self.border_top_left.y }, Size { width: self.border_size.width / 8.0, height: DEFAULT_SIZE }, None)?;
            
            // border bottom
            render.load_image("assets/game/wall.jpg", Position { x: self.border_top_left.x + num as f32 * (self.border_size.width / 8.0), y: self.border_top_left.y + self.border_size.height - DEFAULT_SIZE }, Size { width: self.border_size.width / 8.0, height: DEFAULT_SIZE }, None)?;
        }

        for num in 0..2 {
            // border right
            render.load_image("assets/game/wall.jpg", Position { x: self.border_top_left.x + self.border_size.width - DEFAULT_SIZE, y: self.border_top_left.y + ((num as f32 - 1.0) * DEFAULT_SIZE).abs() + num as f32 * (self.border_size.height / 2.0) }, Size { width: DEFAULT_SIZE, height: (self.border_size.height - 60.0) / 2.0 }, None)?;

            // border left
            render.load_image("assets/game/wall.jpg", Position { x: self.border_top_left.x, y: self.border_top_left.y + ((num as f32 - 1.0) * DEFAULT_SIZE).abs() + num as f32 * (self.border_size.height / 2.0) }, Size { width: DEFAULT_SIZE, height: (self.border_size.height - 60.0) / 2.0 }, None)?;
        }

        for level_object in self.level_objects.iter_mut() {
            match level_object.object_type {
                ObjectLevelType::RegularDoor => {
                    if player.collide(level_object) {
                        level_object.open_door();
                    } else {
                        level_object.close_door();
                    }
                },

                ObjectLevelType::CodedDoor => (),

                ObjectLevelType::LockedDoor => (),

                ObjectLevelType::HidePlace | ObjectLevelType::CodePaper | ObjectLevelType::ExitDoor | ObjectLevelType::Camera | ObjectLevelType::TeleportDoor => (),

                ObjectLevelType::Coin | ObjectLevelType::Key => (),

                _ => {
                    if player.collide(level_object) {
                        player.move_to_prev_position();
                    }
                }
            }

            level_object.draw(render)?;
        }

        for enemy in self.enemies.iter_mut() {
            enemy.draw(render)?;
            enemy.move_enemy(None);
        }

        player.draw(render)?;

        Ok(())
    }
    
    fn insert_object(&mut self, mut object: ObjectLevel<'a>) {
        let start_position = self.get_boder_start_position();

        object.position = object.position + start_position;

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

    pub fn next_level(&mut self, player: &mut Player) {
        assert!(self.current_level < 5, "level must be between 1 to 5 (include)");
        
        self.current_level += 1;
        
        self.enemies.clear();
        self.level_objects.clear();
        
        match self.current_level {
            1 => {
                player.move_to(Position { x: 90.0, y: 180.0 });

                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 140.0, y: 10.0 }, "18d/0 13l/3000 13r/0 18u/0 6r/3000 6l/0"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 100.0, y: 295.0 }, "9l/6000 20r/3000 11l/0"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 405.0 }, "11d/0 15r/0 10d/0 5l/2000 9r/2000 21u/1000 19l/0"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 340.0, y: 30.0 }, "26d/3500 26u/3000"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 380.0, y: 420.0 }, "25r/0 6u/2000 16d/2000 25l/0 9d/0 5l/2000 5r/0 19u/0"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 550.0, y: 257.0 }, "8r/6000 15l/4000 7r/0"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 470.0, y: 157.0 }, "16r/4000 16l/6000"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 470.0, y: 5.0 }, "16r/3000 5d/0 16l/0 5u/5000"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 730.0, y: 520.0 }, "18u/3000 4r/0 18d/0 4l/3500"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 915.0, y: 90.0 }, "15d/2000 4l/0 15u/0 15l/0 15d/2000 4r/0 15u/0 15r/0"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 865.0, y: 335.0 }, "18d/3000 4r/0 18u/5000 4l/0"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 840.0, y: 0.0 }, "20r/4000 20l/6500"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1050.0, y: 100.0 }, "15d/4000 15u/5000"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1020.0, y: 520.0 }, "18u/6000 18d/0 3r/4000 3l/0"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1175.0, y: 520.0 }, "18u/6000 18d/0 3l/4500 3r/0"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1190.0, y: 100.0 }, "15d/3500 15u/6000"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 730.0, y: 615.0 }, "20r/6000 20l/4000"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1030.0, y: 615.0 }, "15r/6000 15l/5500"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1310.0, y: 615.0 }, "18u/6000 18d/0 2l/4500 2r/0"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1310.0, y: 90.0 }, "20d/6000 20u/0 2r/5500 2l/0"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1450.0, y: 90.0 }, "20d/6000 20u/0 2l/4500 2r/0"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1450.0, y: 615.0 }, "18u/5500 18d/0 2r/6000 2l/0"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1140.0, y: 0.0 }, "25r/4000 25l/6000"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1490.0, y: 0.0 }, "21r/4000 21l/6000"));
                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1580.0, y: 504.0 }, "12r/0 1d/4000 42u/3000 12l/0 41d/0"));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 80.0, y: 0.0 }, Size { width: DEFAULT_SIZE, height: 100.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 78.0, y: 100.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 160.0 }, Size { width: 110.0, height: DEFAULT_SIZE }));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 262.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 15.0, height: 70.0 }));
                for num in 0..2 {
                    self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 265.0, y: 70.0 + (num as f32 * 268.5) }, Size { width: DEFAULT_SIZE + 10.0, height: 268.5 }));
                }
                self.insert_object(ObjectLevel::new(ObjectLevelType::CodedDoor, Position { x: 255.0, y: 607.0 }, Size { width: 60.0, height: 70.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 225.0, y: 110.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 250.0 }, Size { width: 210.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 210.0, y: 250.0 }, Size { width: 55.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 130.0, y: 185.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 10.0, y: 200.0 }, DEFAULT_SIZE_FOR_COLLECTABLE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 53.0, y: 360.0 }, Size { width: 213.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 0.0, y: 360.0 }, Size { width: 55.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 180.0, y: 295.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 70.0, y: 295.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 60.0, y: 390.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 225.0, y: 400.0 }, DEFAULT_SIZE_FOR_COLLECTABLE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 220.0, y: 480.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 160.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 50.0, y: 512.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 577.0 }, Size { width: 100.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 62.0, y: 608.0 }, Size { width: 40.0, height: 70.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::CodePaper, Position { x: 5.0, y: 625.0 }, DEFAULT_SIZE_FOR_COLLECTABLE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 505.0, y: 580.0 }, Size { width: DEFAULT_SIZE, height: 96.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 535.0, y: 580.0 }, Size { width: 100.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 635.0, y: 580.0 }, Size { width: 55.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 690.0, y: 580.0 }, Size { width: DEFAULT_SIZE, height: 96.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Key, Position { x: 545.0, y: 630.0 }, DEFAULT_SIZE_FOR_COLLECTABLE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 687.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 690.0, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 360.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 687.0, y: 420.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 690.0, y: 480.0 }, Size { width: DEFAULT_SIZE, height: 100.0 }));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 435.0, y: 0.0 }, Size { width: DEFAULT_SIZE, height: 350.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 465.0, y: 320.0 }, Size { width: 165.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: 620.0, y: 320.0 }, Size { width: 80.0, height: DEFAULT_SIZE }));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 465.0, y: 220.0 }, Size { width: 55.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 520.0, y: 220.0 }, Size { width: 170.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 550.0, y: 255.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 465.0, y: 120.0 }, Size { width: 170.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 635.0, y: 120.0 }, Size { width: 55.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 550.0, y: 155.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 580.0, y: 55.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 520.0, y: 15.0 }, DEFAULT_SIZE_FOR_COLLECTABLE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 390.0, y: 50.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 305.0, y: 220.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 305.0, y: 402.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 375.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 550.0, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 570.0, y: 350.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 480.0, y: 440.0 }, DEFAULT_SIZE_FOR_COLLECTABLE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 797.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 930.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 720.0, y: 60.0 }, Size { width: 335.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1055.0, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1110.0, y: 0.0 }, Size { width: DEFAULT_SIZE, height: 520.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1107.0, y: 520.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1145.0, y: 15.0 }, DEFAULT_SIZE_FOR_COLLECTABLE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1000.0, y: 580.0 }, Size { width: 280.0, height: DEFAULT_SIZE }));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1065.0, y: 130.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1000.0, y: 250.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1065.0, y: 370.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1030.0, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1140.0, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1195.0, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1250.0, y: 60.0 }, Size { width: DEFAULT_SIZE, height: 520.0 }));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1175.0, y: 515.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1205.0, y: 370.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1140.0, y: 250.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1205.0, y: 120.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1280.0, y: 60.0 }, Size { width: 425.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1705.0, y: 60.0 }, Size { width: 55.0, height: DEFAULT_SIZE }));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1285.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1447.0, y: 0.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }));
                
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1575.0, y: 0.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1640.0, y: 90.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1720.0, y: 230.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1555.0, y: 350.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1630.0, y: 514.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1640.0, y: 300.0 }, DEFAULT_SIZE_FOR_COLLECTABLE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 720.0, y: 580.0 }, Size { width: 110.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 827.0, y: 90.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 830.0, y: 150.0 }, Size { width: DEFAULT_SIZE, height: 460.0 }));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 720.0, y: 300.0 }, Size { width: 55.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 775.0, y: 300.0 }, Size { width: 55.0, height: DEFAULT_SIZE }));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 785.0, y: 400.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 755.0, y: 520.0 }, DEFAULT_SIZE_FOR_COLLECTABLE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 785.0, y: 235.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 720.0, y: 110.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 860.0, y: 580.0 }, Size { width: 55.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 915.0, y: 580.0 }, Size { width: 85.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 970.0, y: 90.0 }, Size { width: DEFAULT_SIZE, height: 490.0 }));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 860.0, y: 300.0 }, Size { width: 55.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 915.0, y: 300.0 }, Size { width: 55.0, height: DEFAULT_SIZE }));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 925.0, y: 90.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 860.0, y: 235.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 925.0, y: 400.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 990.0, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 900.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 780.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 730.0, y: 625.0 }, DEFAULT_SIZE_FOR_COLLECTABLE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1247.0, y: 610.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1100.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1387.0, y: 90.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 60.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1390.0, y: 150.0 }, Size { width: DEFAULT_SIZE, height: 527.0 }));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1310.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1350.0, y: 450.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1280.0, y: 350.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1350.0, y: 250.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1280.0, y: 120.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1530.0, y: 90.0 }, Size { width: DEFAULT_SIZE, height: 517.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1527.0, y: 607.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }));

                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1485.0, y: 100.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1420.0, y: 250.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1485.0, y: 350.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1420.0, y: 450.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 1450.0, y: 615.0 }, DEFAULT_SIZE_FOR_HIDE_PLACE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1430.0, y: 360.0 }, DEFAULT_SIZE_FOR_COLLECTABLE));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1560.0, y: 577.0 }, Size { width: 145.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: 1697.0, y: 575.0 }, Size { width: 70.0, height: DEFAULT_SIZE + 2.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::ExitDoor, Position { x: 1697.0, y: 620.0 }, Size { width: 70.0, height: 60.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1600.0, y: 630.0 }, DEFAULT_SIZE_FOR_COLLECTABLE));
            },

            2 => {
                player.move_to(Position { x: 1777.0, y: 780.0 });

                self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 1070.0, y: 615.0 }, "52r/3500 52l/4000"));
                // self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 680.0, y: 420.0 }, "2r/0 28l/0 13u/0 13d/0 26r/0"));
                // self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 10.0 }, "24d/0 5r/0 24u/0 5l/0"));
                // self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 200.0, y: 10.0 }, "28d/0 29u/0 12r/0 18d/0 9r/0 9l/0 18u/0 12l/0 1d/0"));
                // self.insert_enemy(Enemy::new(EnemyType::Regular, Position { x: 570.0, y: 15.0 }, "10d/0 2l/0 16d/0 12r/0 2u/0 6r/0 24u/0 16l/0"));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1360.0, y: 577.0 }, Size { width: 400.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1650.0, y: 607.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1330.0, y: 540.0 }, Size { width: DEFAULT_SIZE, height: 67.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1230.0, y: 510.0 }, Size { width: 130.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1230.0, y: 540.0 }, Size { width: DEFAULT_SIZE, height: 67.0 }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 1030.0, y: 577.0 }, Size { width: 200.0, height: DEFAULT_SIZE }));
                self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 1027.0, y: 607.0 }, Size { width: DEFAULT_SIZE + 5.0, height: 70.0 }));

                self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 1275.0, y: 550.0 }, DEFAULT_SIZE_FOR_COLLECTABLE));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 1450.0, y: 582.0 }, DEFAULT_SIZE_FOR_CAMERA));
                self.insert_object(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 1110.0, y: 582.0 }, DEFAULT_SIZE_FOR_CAMERA));

                // self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 260.0 , y: 70.0 }, Size { width: 40.0, height: 240.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 255.0 , y: 310.0 }, Size { width: 50.0, height: 70.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 240.0, y: 180.0 }, Size { width: 30.0, height: 30.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 300.0 , y: 240.0 }, Size { width: 220.0, height: 40.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 470.0 , y: 280.0 }, Size { width: 50.0, height: 135.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 420.0, y: 380.0 }, Size { width: 50.0, height: 40.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 425.0, y: 285.0 }, Size { width: 40.0, height: 40.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 330.0, y: 305.0 }, Size { width: 45.0, height: 80.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 520.0 , y: 335.0 }, Size { width: 220.0, height: 40.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 735.0, y: 335.0 }, Size { width: 70.0, height: 40.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 580.0, y: 405.0 }, Size { width: 45.0, height: 80.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 660.0, y: 355.0 }, Size { width: 30.0, height: 30.0 })); // rotate here 180

                // self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 190.0, y: 0.0 }, Size { width: 45.0, height: 80.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 150.0 , y: 0.0 }, Size { width: 40.0, height: 320.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 50.0 , y: 320.0 }, Size { width: 140.0, height: 40.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 0.0, y: 320.0 }, Size { width: 50.0, height: 40.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 80.0, y: 240.0 }, Size { width: 45.0, height: 80.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 0.0, y: 0.0 }, Size { width: 45.0, height: 80.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::Key, Position { x: 85.0, y: 10.0 }, Size { width: 60.0, height: 60.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 125.0, y: 130.0 }, Size { width: 30.0, height: 30.0 }));

                // self.insert_object(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 480.0 , y: 0.0 }, Size { width: 40.0, height: 180.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 478.0, y: 182.0 }, Size { width: 45.0, height: 57.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 370.0, y: 165.0 }, Size { width: 45.0, height: 80.0 }));

                // self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 610.0, y: 260.0 }, Size { width: 45.0, height: 80.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 520.0, y: 0.0 }, Size { width: 45.0, height: 80.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 620.0, y: 150.0 }, Size { width: 40.0, height: 40.0 }));
                // self.insert_object(ObjectLevel::new(ObjectLevelType::ExitDoor, Position { x: 730.0, y: 5.0 }, Size { width: 80.0, height: 100.0 }));
            },

            3 => {
                player.move_to(Position { x: 750.0, y: 5.0 });

                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 590.0, y: 10.0 }, "10d 12r 32d 32u 12l 10u"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 260.0, y: 10.0 }, "10d 14l 28r 14l 10u"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 310.0, y: 220.0 }, "1r 15l 14r"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 440.0, y: 325.0 }, "6d 3r 12d 12u 3l 6u"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 180.0, y: 540.0 }, "25r 25l"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 80.0, y: 330.0 }, "7l 30r 23l"));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 680.0, y: 65.0 }, Size { width: 120.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 676.5, y: 0.0 }, Size { width: 40.0, height: 65.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 540.0, y: 0.0 }, Size { width: 40.0, height: 170.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 495.0, y: 0.0 }, Size { width: 45.0, height: 80.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 520.0, y: 90.0 }, Size { width: 30.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 540.0, y: 170.0 }, Size { width: 160.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 595.0, y: 95.0 }, Size { width: 45.0, height: 80.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 660.0, y: 210.0 }, Size { width: 40.0, height: 170.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 755.0, y: 190.0 }, Size { width: 45.0, height: 80.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 680.0, y: 290.0 }, Size { width: 30.0, height: 30.0 })); // rotate here 270
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 540.0, y: 380.0 }, Size { width: 160.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 540.0, y: 420.0 }, Size { width: 40.0, height: 110.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 495.0, y: 450.0 }, Size { width: 45.0, height: 80.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 660.0, y: 420.0 }, Size { width: 40.0, height: 70.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 580.0, y: 430.0 }, Size { width: 50.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 580.0, y: 490.0 }, Size { width: 220.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 670.0, y: 505.0 }, Size { width: 30.0, height: 30.0 })); // rotate here 180

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: 535.0, y: 530.0 }, Size { width: 50.0, height: 70.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 610.0, y: 540.0 }, Size { width: 50.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 750.0, y: 550.0 }, Size { width: 40.0, height: 40.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Key, Position { x: 615.0, y: 210.0 }, Size { width: 40.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 610.0, y: 270.0 }, Size { width: 40.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 610.0, y: 318.0 }, Size { width: 50.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 540.0, y: 210.0 }, Size { width: 40.0, height: 170.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 560.0, y: 260.0 }, Size { width: 30.0, height: 30.0 })); // rotate here 45
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 495.0, y: 270.0 }, Size { width: 45.0, height: 80.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 65.0 }, Size { width: 140.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 104.0, y: 0.0 }, Size { width: 40.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 5.0, y: 3.5 }, Size { width: 50.0, height: 65.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 170.0 }, Size { width: 240.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 200.0, y: 95.0 }, Size { width: 45.0, height: 80.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 10.0, y: 130.0 }, Size { width: 40.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 240.0, y: 170.0 }, Size { width: 250.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 490.0, y: 170.0 }, Size { width: 50.0, height: 40.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 383.0, y: 210.0 }, Size { width: 40.0, height: 70.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 250.0, y: 210.0 }, Size { width: 45.0, height: 75.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 125.0, y: 210.0 }, Size { width: 40.0, height: 70.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 60.0, y: 220.0 }, Size { width: 50.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: 0.0, y: 278.0 }, Size { width: 55.0, height: 50.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 50.0, y: 280.0 }, Size { width: 370.0, height: 45.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 190.0, y: 300.0 }, Size { width: 30.0, height: 30.0 })); // rotate here 180

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 380.0, y: 325.0 }, Size { width: 40.0, height: 205.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 420.0, y: 450.0 }, Size { width: 45.0, height: 80.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 170.0, y: 490.0 }, Size { width: 210.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 240.0, y: 505.0 }, Size { width: 30.0, height: 30.0 })); // rotate here 180
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 310.0, y: 532.0 }, Size { width: 45.0, height: 70.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 180.0, y: 532.0 }, Size { width: 45.0, height: 70.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 490.0 }, Size { width: 120.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: 112.0, y: 490.0 }, Size { width: 65.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 78.0, y: 530.0 }, Size { width: 40.0, height: 70.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::ExitDoor, Position { x: 0.0, y: 530.0 }, Size { width: 60.0, height: 70.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 100.0, y: 325.0 }, Size { width: 45.0, height: 70.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 250.0, y: 325.0 }, Size { width: 45.0, height: 70.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 390.0 }, Size { width: 330.0, height: 35.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 330.0, y: 395.0 }, Size { width: 50.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 200.0, y: 400.0 }, Size { width: 30.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 70.0, y: 425.0 }, Size { width: 40.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 5.0, y: 450.0 }, Size { width: 40.0, height: 40.0 }));
            },

            4 => {
                player.move_to(Position { x: 0.0, y: 540.0 });

                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 140.0, y: 540.0 }, "22r 22l"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 532.0, y: 437.0 }, "8r 10d 10l 10r 10u 12r 20l"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 560.0, y: 332.0 }, "18r 18l"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 510.0, y: 170.0 }, "5d 20r 5u 20l"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 540.0, y: 5.0 }, "6d 15r 6u 15l"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 0.0, y: 190.0 }, "15d 15u"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 110.0, y: 190.0 }, "15d 15u"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 230.0, y: 190.0 }, "16d 12l 12r 16u"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 110.0, y: 0.0 }, "11r 11l"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 420.0, y: 370.0 }, "6d 6u"));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 500.0 }, Size { width: 120.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 83.0, y: 540.0 }, Size { width: 40.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 120.0, y: 500.0 }, Size { width: 120.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 170.0, y: 540.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 237.0, y: 500.0 }, Size { width: 60.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 297.0, y: 500.0 }, Size { width: 180.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 330.0, y: 540.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 438.0, y: 540.0 }, Size { width: 40.0, height: 65.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 477.0, y: 500.0 }, Size { width: 130.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 545.0, y: 515.0 }, Size { width: 30.0, height: 30.0 })); // rotate here 180
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 607.0, y: 500.0 }, Size { width: 60.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 495.0, y: 540.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 667.0, y: 540.0 }, Size { width: 30.0, height: 60.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 667.0, y: 500.0 }, Size { width: 80.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 747.0, y: 500.0 }, Size { width: 53.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Key, Position { x: 700.0, y: 560.0 }, Size { width: 40.0, height: 40.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 300.0, y: 125.0 }, Size { width: 80.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: 372.0, y: 125.0 }, Size { width: 65.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 430.0, y: 125.0 }, Size { width: 80.0, height: 40.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 470.0, y: 165.0 }, Size { width: 40.0, height: 50.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: 462.0, y: 214.0 }, Size { width: 55.0, height: 67.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 470.0, y: 281.0 }, Size { width: 40.0, height: 50.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 300.0, y: 165.0 }, Size { width: 40.0, height: 50.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 297.0, y: 215.5 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 300.0, y: 281.0 }, Size { width: 40.0, height: 50.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 300.0, y: 328.0 }, Size { width: 80.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::CodedDoor, Position { x: 367.0, y: 325.0 }, Size { width: 75.0, height: 43.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 430.0, y: 328.0 }, Size { width: 80.0, height: 40.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::ExitDoor, Position { x: 370.0, y: 210.0 }, Size { width: 70.0, height: 80.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 480.0, y: 368.0 }, Size { width: 30.0, height: 133.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 330.0, y: 420.0 }, Size { width: 80.0, height: 20.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 382.0, y: 440.0 }, Size { width: 30.0, height: 60.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 330.0, y: 443.0 }, Size { width: 45.0, height: 60.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 437.0, y: 440.0 }, Size { width: 45.0, height: 60.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 437.0, y: 366.0 }, Size { width: 45.0, height: 60.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 330.0, y: 380.0 }, Size { width: 40.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 300.0, y: 368.0 }, Size { width: 30.0, height: 133.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 507.0, y: 395.0 }, Size { width: 55.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 562.0, y: 395.0 }, Size { width: 238.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 720.0, y: 410.0 }, Size { width: 30.0, height: 30.0 })); // rotate here 180
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 562.0, y: 437.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 682.0, y: 437.0 }, Size { width: 45.0, height: 65.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 510.0, y: 281.0 }, Size { width: 120.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 630.0, y: 281.0 }, Size { width: 55.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 685.0, y: 281.0 }, Size { width: 115.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 580.0, y: 332.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 700.0, y: 332.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 760.0, y: 355.0 }, Size { width: 40.0, height: 40.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 510.0, y: 125.0 }, Size { width: 235.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 745.0, y: 125.0 }, Size { width: 55.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 680.0, y: 140.0 }, Size { width: 30.0, height: 30.0 })); // rotate here 180
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 580.0, y: 216.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 510.0, y: 165.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 755.0, y: 216.0 }, Size { width: 45.0, height: 65.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 470.0, y: 0.0 }, Size { width: 40.0, height: 60.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 466.0, y: 61.0 }, Size { width: 48.0, height: 64.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Key, Position { x: 520.0, y: 5.0 }, Size { width: 40.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 680.0, y: 62.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 550.0, y: 62.0 }, Size { width: 45.0, height: 65.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 300.0, y: 0.0 }, Size { width: 40.0, height: 125.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 380.0, y: 10.0 }, Size { width: 40.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 320.0, y: 60.0 }, Size { width: 30.0, height: 30.0 })); // rotate here 270 and then 45

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 50.0, y: 410.0 }, Size { width: 250.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 0.0, y: 410.0 }, Size { width: 50.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 170.0, y: 442.0 }, Size { width: 45.0, height: 60.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 100.0, y: 415.0 }, Size { width: 30.0, height: 30.0 })); // rotate here 270

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 70.0, y: 150.0 }, Size { width: 30.0, height: 260.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 50.0, y: 150.0 }, Size { width: 17.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 0.0, y: 150.0 }, Size { width: 50.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 50.0, y: 260.0 }, Size { width: 30.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 30.0, y: 330.0 }, Size { width: 45.0, height: 60.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 30.0, y: 190.0 }, Size { width: 45.0, height: 60.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 60.0 }, Size { width: 250.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 250.0, y: 60.0 }, Size { width: 50.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 175.0, y: 65.0 }, Size { width: 30.0, height: 30.0 })); // rotate here 270
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 260.0, y: 110.0 }, Size { width: 40.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::CodePaper, Position { x: 210.0, y: 110.0 }, Size { width: 40.0, height: 40.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 150.0, y: 150.0 }, Size { width: 150.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 100.0, y: 150.0 }, Size { width: 50.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 180.0, y: 180.0 }, Size { width: 40.0, height: 170.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 180.0, y: 350.0 }, Size { width: 40.0, height: 60.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 160.0, y: 220.0 }, Size { width: 30.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 100.0, y: 190.0 }, Size { width: 45.0, height: 60.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 135.0, y: 260.0 }, Size { width: 45.0, height: 60.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 100.0, y: 350.0 }, Size { width: 45.0, height: 60.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 200.0, y: 275.0 }, Size { width: 30.0, height: 30.0 })); // rotate here 45
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 255.0, y: 350.0 }, Size { width: 45.0, height: 60.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 220.0, y: 180.0 }, Size { width: 45.0, height: 60.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 260.0, y: 190.0 }, Size { width: 40.0, height: 40.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 50.0, y: 0.0 }, Size { width: 40.0, height: 60.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 0.0, y: 3.0 }, Size { width: 45.0, height: 60.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 200.0, y: 5.0 }, Size { width: 45.0, height: 57.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 110.0, y: 5.0 }, Size { width: 45.0, height: 57.0 }));
            },

            5 => {
                player.move_to(Position { x: 370.0, y: 275.0 });

                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 530.0, y: 540.0 }, "18r 34l 12u 12d 16r"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 600.0, y: 430.0 }, "12r 12l"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 110.0, y: 540.0 }, "16r 16l"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 120.0, y: 130.0 }, "17r 17l"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 60.0, y: 352.0 }, "17r 17l"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 505.0, y: 230.0 }, "12d 12u"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 615.0, y: 130.0 }, "11d 11u"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 740.0, y: 245.0 }, "12l 10d 10u 12r"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 540.0, y: 20.0 }, "14r 14l"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 310.0, y: 20.0 }, "12r 12l"));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 305.0, y: 190.0 }, Size { width: 190.0, height: 40.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 455.0, y: 230.0 }, Size { width: 40.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 452.0, y: 270.5 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 455.0, y: 336.0 }, Size { width: 40.0, height: 40.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 302.0, y: 230.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 305.0, y: 295.0 }, Size { width: 40.0, height: 80.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 305.0, y: 373.0 }, Size { width: 65.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 367.0, y: 373.0 }, Size { width: 60.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 425.0, y: 373.0 }, Size { width: 70.0, height: 40.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 330.0, y: 413.0 }, Size { width: 40.0, height: 187.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 435.0, y: 413.0 }, Size { width: 40.0, height: 120.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 432.0, y: 534.0 }, Size { width: 45.0, height: 67.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 415.0, y: 460.0 }, Size { width: 30.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 370.0, y: 420.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 370.0, y: 535.0 }, Size { width: 45.0, height: 65.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 475.0, y: 493.0 }, Size { width: 275.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 750.0, y: 493.0 }, Size { width: 50.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 620.0, y: 508.0 }, Size { width: 30.0, height: 30.0 })); // rotate here 270
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 520.0, y: 535.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 690.0, y: 535.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 760.0, y: 550.0 }, Size { width: 40.0, height: 40.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 475.0, y: 413.0 }, Size { width: 325.0, height: 10.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 540.0, y: 424.0 }, Size { width: 40.0, height: 70.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 610.0, y: 408.0 }, Size { width: 30.0, height: 30.0 })); // rotate here 270
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 680.0, y: 430.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 480.0, y: 436.0 }, Size { width: 45.0, height: 60.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 413.0 }, Size { width: 330.0, height: 10.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 240.0, y: 423.0 }, Size { width: 30.0, height: 112.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 238.0, y: 536.0 }, Size { width: 35.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 280.0, y: 423.0 }, Size { width: 45.0, height: 60.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 250.0, y: 480.0 }, Size { width: 30.0, height: 30.0 })); // rotate here 270
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 285.0, y: 535.0 }, Size { width: 45.0, height: 65.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 495.0 }, Size { width: 180.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 180.0, y: 495.0 }, Size { width: 55.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 50.0, y: 536.0 }, Size { width: 40.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 120.0, y: 535.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::CodePaper, Position { x: 5.0, y: 550.0 }, Size { width: 40.0, height: 40.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 42.0, y: 423.0 }, Size { width: 40.0, height: 72.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 0.0, y: 438.0 }, Size { width: 45.0, height: 60.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 110.0, y: 408.0 }, Size { width: 30.0, height: 30.0 })); // rotate here 270 (repeat)
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 195.0, y: 422.0 }, Size { width: 45.0, height: 65.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::CodedDoor, Position { x: 0.0, y: 185.0 }, Size { width: 55.0, height: 35.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 50.0, y: 190.0 }, Size { width: 255.0, height: 30.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 0.0, y: 80.0 }, Size { width: 50.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 50.0, y: 80.0 }, Size { width: 250.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 300.0, y: 80.0 }, Size { width: 250.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 550.0, y: 80.0 }, Size { width: 200.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 750.0, y: 80.0 }, Size { width: 50.0, height: 40.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 680.0, y: 15.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 610.0, y: 0.0 }, Size { width: 30.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 530.0, y: 15.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 480.0, y: 0.0 }, Size { width: 40.0, height: 80.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 425.0, y: 15.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 365.0, y: 30.0 }, Size { width: 40.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 300.0, y: 15.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 250.0, y: 0.0 }, Size { width: 40.0, height: 80.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 195.0, y: 15.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 140.0, y: 0.0 }, Size { width: 30.0, height: 30.0 })); // rotate here 270 (repeat)

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 455.0, y: 120.0 }, Size { width: 40.0, height: 70.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::TeleportDoor, Position { x: 410.0, y: 133.0 }, Size { width: 45.0, height: 60.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 360.0, y: 120.0 }, Size { width: 40.0, height: 70.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::CodedDoor, Position { x: 55.0, y: 120.0 }, Size { width: 55.0, height: 70.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 220.0, y: 95.0 }, Size { width: 30.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 280.0, y: 125.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 130.0, y: 125.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 210.0, y: 145.0 }, Size { width: 40.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 10.0, y: 135.0 }, Size { width: 40.0, height: 40.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: 70.0, y: 0.0 }, Size { width: 45.0, height: 80.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::ExitDoor, Position { x: 5.0, y: 10.0 }, Size { width: 70.0, height: 60.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 210.0, y: 220.0 }, Size { width: 30.0, height: 125.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 207.0, y: 345.0 }, Size { width: 35.0, height: 70.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 220.0, y: 280.0 }, Size { width: 30.0, height: 30.0 })); // rotate here 270 and 45
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 260.0, y: 350.0 }, Size { width: 45.0, height: 65.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 0.0, y: 315.0 }, Size { width: 50.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 50.0, y: 315.0 }, Size { width: 160.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 100.0, y: 320.0 }, Size { width: 30.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 145.0, y: 350.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 70.0, y: 360.0 }, Size { width: 40.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 5.0, y: 350.0 }, Size { width: 45.0, height: 65.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 60.0, y: 220.0 }, Size { width: 30.0, height: 34.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 57.0, y: 255.0 }, Size { width: 35.0, height: 60.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::CodePaper, Position { x: 165.0, y: 225.0 }, Size { width: 40.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 170.0, y: 275.0 }, Size { width: 40.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 110.0, y: 195.0 }, Size { width: 30.0, height: 30.0 })); // rotate here 270

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 495.0, y: 190.0 }, Size { width: 100.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 552.0, y: 120.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Key, Position { x: 500.0, y: 140.0 }, Size { width: 40.0, height: 40.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 562.0, y: 220.0 }, Size { width: 35.0, height: 72.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 565.0, y: 290.0 }, Size { width: 30.0, height: 50.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::LockedDoor, Position { x: 560.0, y: 340.0 }, Size { width: 40.0, height: 75.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 495.0, y: 220.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 495.0, y: 350.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 595.0, y: 280.0 }, Size { width: 45.0, height: 65.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 680.0, y: 315.0 }, Size { width: 30.0, height: 100.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 710.0, y: 315.0 }, Size { width: 40.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 750.0, y: 315.0 }, Size { width: 50.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 760.0, y: 365.0 }, Size { width: 40.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Key, Position { x: 715.0, y: 365.0 }, Size { width: 40.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 635.0, y: 350.0 }, Size { width: 45.0, height: 65.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 700.0, y: 250.0 }, Size { width: 45.0, height: 65.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 680.0, y: 120.0 }, Size { width: 30.0, height: 100.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 710.0, y: 190.0 }, Size { width: 40.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 750.0, y: 190.0 }, Size { width: 50.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 635.0, y: 120.0 }, Size { width: 45.0, height: 65.0 }));

            },

            _ => ()
        }
    }
}
