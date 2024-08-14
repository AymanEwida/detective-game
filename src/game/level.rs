use crate::{game::enemy::EnemyType, renderer::{error::Result, render::{Render, Size}, vertice::Position}};

use super::{character::Character, enemy::Enemy, player::Player};

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
    current_level: u8,
    enemies: Vec<Enemy<'a>>,
    level_objects: Vec<ObjectLevel<'a>>
}

impl Default for GameLevel<'_> {
    fn default() -> Self {
        let enemies = vec![
            Enemy::new(EnemyType::Regular, Position { x: 190.0, y: 10.0 }, "4d 5u 17r 18d 18u 17l 1d"),
            Enemy::new(EnemyType::Regular, Position { x: 15.0, y: 330.0 }, "15r 15u 15d 16l 1r"),
            Enemy::new(EnemyType::Regular, Position { x: 590.0, y: 340.0 }, "21l 12u 30d 18u 26r 5l"),
            Enemy::new(EnemyType::Regular, Position { x: 600.0, y: 10.0 }, "15d 13r 15u 13l"),
        ];

        let level_objects = vec![
            ObjectLevel::new(ObjectLevelType::Wall, Position { x: 80.0, y: 0.0 }, Size { width: 50.0, height: 200.0 }),
            ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 75.0, y: 200.0 }, Size { width: 60.0, height: 70.0 }),
            ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 270.0 }, Size { width: 130.0, height: 50.0 }),

            ObjectLevel::new(ObjectLevelType::Wall, Position { x: 265.0, y: 70.0 }, Size { width: 60.0, height: 455.0 }),
            ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 225.0, y: 180.0 }, Size { width: 45.0, height: 100.0 }),
            ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 260.0, y: 525.0 }, Size { width: 70.0, height: 80.0 }),
            ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 325.0, y: 300.0 }, Size { width: 45.0, height: 100.0 }),

            ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 430.0 }, Size { width: 50.0, height: 70.0 }),
            ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 50.0, y: 430.0 }, Size { width: 55.0, height: 70.0 }),
            ObjectLevel::new(ObjectLevelType::Wall, Position { x: 104.0, y: 430.0 }, Size { width: 50.0, height: 170.0 }),
            ObjectLevel::new(ObjectLevelType::Coin, Position { x: 2.0, y: 505.0 }, Size { width: 40.0, height: 40.0 }),
            ObjectLevel::new(ObjectLevelType::CodePaper, Position { x: 50.0, y: 555.0 }, Size { width: 40.0, height: 40.0 }),

            ObjectLevel::new(ObjectLevelType::Wall, Position { x: 480.0, y: 420.0 }, Size { width: 111.25, height: 50.0 }),
            ObjectLevel::new(ObjectLevelType::Wall, Position { x: 591.25, y: 420.0 }, Size { width: 141.25, height: 50.0 }),
            ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 729.5, y: 420.0 }, Size { width: 70.0, height: 50.0 }),
            ObjectLevel::new(ObjectLevelType::Wall, Position { x: 480.0, y: 470.0 }, Size { width: 50.25, height: 130.0 }),
            ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 435.0, y: 500.0 }, Size { width: 45.0, height: 100.0 }),
            ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 755.0, y: 290.0 }, Size { width: 45.0, height: 100.0 }),
            ObjectLevel::new(ObjectLevelType::Coin, Position { x: 540.0, y: 555.0 }, Size { width: 40.0, height: 40.0 }),
            ObjectLevel::new(ObjectLevelType::ExitDoor, Position { x: 740.0, y: 500.0 }, Size { width: 80.0, height: 100.0 }),

            ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 445.0, y: 0.0 }, Size { width: 45.0, height: 100.0 }),
            ObjectLevel::new(ObjectLevelType::Wall, Position { x: 490.0, y: 0.0 }, Size { width: 50.0, height: 230.0 }),
            ObjectLevel::new(ObjectLevelType::Wall, Position { x: 490.0, y: 230.0 }, Size { width: 160.0, height: 50.0 }),
            ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 650.0, y: 230.0 }, Size { width: 70.0, height: 50.0 }),
            ObjectLevel::new(ObjectLevelType::Wall, Position { x: 720.0, y: 230.0 }, Size { width: 80.0, height: 50.0 }),
            ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 540.0, y: 130.0 }, Size { width: 45.0, height: 100.0 }),
            ObjectLevel::new(ObjectLevelType::Coin, Position { x: 730.0, y: 30.0 }, Size { width: 40.0, height: 40.0 }),
        ];

        Self {
            current_level: 1,
            enemies,
            level_objects,
        }
    }
}

impl<'a> GameLevel<'a> {
    pub fn draw(&mut self, player: &mut Player<'a>, render: &mut Render<'a>) -> Result<()> {
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
    
    pub fn next_level(&mut self, player: &mut Player) {
        assert!(self.current_level >= 1 && self.current_level < 5, "level must be between 1 to 5 (include)");
        
        self.current_level += 1;
        
        self.enemies.clear();
        self.level_objects.clear();
        
        match self.current_level {
            2 => {
                player.move_to(Position { x: 750.0, y: 540.0 });

                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 200.0, y: 540.0 }, "39r 39l"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 680.0, y: 420.0 }, "2r 28l 13u 13d 26r"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 10.0, y: 10.0 }, "24d 5r 24u 5l"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 200.0, y: 10.0 }, "28d 29u 12r 18d 9r 9l 18u 12l 1d"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 570.0, y: 15.0 }, "10d 2l 16d 12r 2u 6r 24u 16l"));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 650.0, y: 480.0 }, Size { width: 150.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 645.0, y: 520.0 }, Size { width: 50.0, height: 85.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 420.0, y: 480.0 }, Size { width: 230.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 530.0, y: 520.0 }, Size { width: 45.0, height: 80.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 450.0, y: 500.0 }, Size { width: 30.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 380.0, y: 380.0 }, Size { width: 40.0, height: 140.0 }));           
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 300.0, y: 380.0 }, Size { width: 80.0, height: 40.0 }));         
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 260.0, y: 380.0 }, Size { width: 40.0, height: 140.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 320.0, y: 430.0 }, Size { width: 40.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 130.0, y: 480.0 }, Size { width: 130.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 125.0, y: 520.0 }, Size { width: 50.0, height: 80.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 170.0, y: 405.0 }, Size { width: 45.0, height: 80.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 260.0 , y: 70.0 }, Size { width: 40.0, height: 240.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 255.0 , y: 310.0 }, Size { width: 50.0, height: 70.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 240.0, y: 180.0 }, Size { width: 30.0, height: 30.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 300.0 , y: 240.0 }, Size { width: 220.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 470.0 , y: 280.0 }, Size { width: 50.0, height: 135.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 420.0, y: 380.0 }, Size { width: 50.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 425.0, y: 285.0 }, Size { width: 40.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 330.0, y: 305.0 }, Size { width: 45.0, height: 80.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 520.0 , y: 335.0 }, Size { width: 220.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 735.0, y: 335.0 }, Size { width: 70.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 580.0, y: 405.0 }, Size { width: 45.0, height: 80.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 660.0, y: 355.0 }, Size { width: 30.0, height: 30.0 })); // rotate here 180

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 190.0, y: 0.0 }, Size { width: 45.0, height: 80.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 150.0 , y: 0.0 }, Size { width: 40.0, height: 320.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 50.0 , y: 320.0 }, Size { width: 140.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 0.0, y: 320.0 }, Size { width: 50.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 80.0, y: 240.0 }, Size { width: 45.0, height: 80.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 0.0, y: 0.0 }, Size { width: 45.0, height: 80.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Key, Position { x: 85.0, y: 10.0 }, Size { width: 60.0, height: 60.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Camera, Position { x: 125.0, y: 130.0 }, Size { width: 30.0, height: 30.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 480.0 , y: 0.0 }, Size { width: 40.0, height: 180.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 478.0, y: 182.0 }, Size { width: 45.0, height: 57.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 370.0, y: 165.0 }, Size { width: 45.0, height: 80.0 }));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 610.0, y: 260.0 }, Size { width: 45.0, height: 80.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::HidePlace, Position { x: 520.0, y: 0.0 }, Size { width: 45.0, height: 80.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Coin, Position { x: 620.0, y: 150.0 }, Size { width: 40.0, height: 40.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::ExitDoor, Position { x: 730.0, y: 5.0 }, Size { width: 80.0, height: 100.0 }));
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
                player.move_to(Position { x: 10.0, y: 10.0 });

                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 300.0, y: 10.0 }, "4d 4u 2r 3d 3u 2l"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 15.0, y: 300.0 }, "3r 2u 2d 4l 1r"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 600.0, y: 400.0 }, "3l 3u 3d 5r 2l"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 600.0, y: 10.0 }, "2d 2r 2u 2l"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 400.0, y: 500.0 }, "2r 2d 2u 3l 1r"));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 40.0, y: 0.0 }, Size { width: 50.0, height: 100.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 40.0, y: 100.0 }, Size { width: 50.0, height: 100.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 40.0, y: 200.0 }, Size { width: 50.0, height: 50.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 35.0, y: 250.0 }, Size { width: 45.0, height: 50.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 250.0 }, Size { width: 35.0, height: 50.0 }));
            },

            5 => {
                player.move_to(Position { x: 10.0, y: 10.0 });

                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 300.0, y: 10.0 }, "4d 4u 2r 3d 3u 2l"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 15.0, y: 300.0 }, "3r 2u 2d 4l 1r"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 600.0, y: 400.0 }, "3l 3u 3d 5r 2l"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 600.0, y: 10.0 }, "2d 2r 2u 2l"));
                self.enemies.push(Enemy::new(EnemyType::Regular, Position { x: 400.0, y: 500.0 }, "2r 2d 2u 3l 1r"));

                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 40.0, y: 0.0 }, Size { width: 50.0, height: 100.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 40.0, y: 100.0 }, Size { width: 50.0, height: 100.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::RegularDoor, Position { x: 40.0, y: 200.0 }, Size { width: 50.0, height: 50.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 35.0, y: 250.0 }, Size { width: 45.0, height: 50.0 }));
                self.level_objects.push(ObjectLevel::new(ObjectLevelType::Wall, Position { x: 0.0, y: 250.0 }, Size { width: 35.0, height: 50.0 }));
            },

            _ => ()
        }
    }
}
