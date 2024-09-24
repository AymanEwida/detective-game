use crate::renderer::{error::Result, render::{Render, Size}, vertice::Position};

use super::{character::{Character, Direction}, level::GameObject};

#[derive(Debug)]
pub enum PlayerStatus {
    NotHidden,
    Hidden,
    Detectit
}

impl std::fmt::Display for PlayerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotHidden => write!(f, "not hidden"),
            Self::Hidden => write!(f, "hidden"),
            Self::Detectit => write!(f, "detectit")
        }
    }
}

#[derive(Debug)]
pub struct Player<'a> {
    position: Position,
    prev_position: Option<Position>,
    size: Size,
    image: &'a str,
    flip: bool,
    status: PlayerStatus
}

impl Player<'_> {
    pub fn new(start_position: Position, flip: bool) -> Self {
        Self {
            position: start_position, 
            prev_position: None,
            size: Size { width: 50.0, height: 60.0 },
            image: "assets/game/detective.png",
            flip,
            status: PlayerStatus::NotHidden
        }
    }
}

impl<'a> GameObject<'a> for Player<'a> {
    fn draw(&self, render: &mut Render<'a>) -> Result<()> {
        render.load_image(self.image, self.position, self.size, self.flip, None, None, None)?;

        Ok(())
    }

    fn get_position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, new_position: Position) {
        self.position = new_position;
    }

    fn get_size(&self) -> Size {
        self.size
    }
}

impl<'a> Character<'a> for Player<'a> {
    fn set_flip(&mut self, new_value: bool) {
        self.flip = new_value;
    }
}

impl<'a> Player<'a> {
    pub fn move_player(&mut self, direction: Direction, speed: Option<f32>) {
        self.prev_position = Some(self.get_position());

        self.move_character(direction, speed);
    }

    pub fn move_to_prev_position(&mut self) {
        if let Some(prev_position) = self.prev_position {
            self.position = prev_position;
        }
    }

    pub fn get_prev_position(&self) -> Option<Position> {
        self.prev_position
    }

    pub fn move_to(&mut self, new_position: Position, flip: bool) {
        self.flip = flip;

        self.position = new_position;
    }

    pub fn is_off_window(&self, window_size: Size) -> bool {
        self.position.x > window_size.width ||
        (self.position.x + self.size.width) > window_size.width ||
        self.position.x < 0.0 ||
        self.position.y > window_size.height ||
        (self.position.y + self.size.height) > window_size.height ||
        self.position.y < 0.0
    }

    pub fn is_off_border(&self, start_position: Option<Position>, size: Size) -> bool {
        let start_position = start_position.unwrap_or(Position { x: 0.0, y: 0.0 });

        self.position.x > (start_position.x + size.width) ||
        (self.position.x + self.size.width) > (start_position.x + size.width) ||
        self.position.x < start_position.x ||
        self.position.y > (start_position.y + size.height) ||
        (self.position.y + self.size.height) > (start_position.y + size.height) ||
        self.position.y < start_position.y
    }

    pub fn get_status(&self) -> &PlayerStatus {
        &self.status
    }

    pub fn set_status(&mut self, new_status: PlayerStatus) {
        self.status = new_status;
    }
}
