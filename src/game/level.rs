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
    HidePlace,
    Coin,
    CodePaper,
    ExitDoor,
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
            ObjectLevelType::HidePlace => "assets/game/hide-place1.webp",
            ObjectLevelType::Coin => "assets/game/coin.png",
            ObjectLevelType::CodePaper => "assets/game/code-paper.webp",
            ObjectLevelType::ExitDoor => "assets/game/exit-door.png"
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
        render.load_image(self.image, self.position, self.size)?;

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

                ObjectLevelType::HidePlace | ObjectLevelType::CodePaper | ObjectLevelType::ExitDoor => (),

                ObjectLevelType::Coin => (),

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

            3 => {
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
