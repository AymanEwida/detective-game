use glfw::{Action, Key};

use crate::{library::utils::round_position_to_full_numbers, renderer::{error::Result, render::{Render, Size}, vertice::Position}};

use super::{character::{Character, Direction}, hide_place::HidePlace, level::GameObject, wall::Wall};

#[derive(Debug, PartialEq)]
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
pub struct PlayerInteraction {
    key: Key,
    action: Action
}

impl PlayerInteraction {
    pub fn new(key: Key, action: Action) -> Self {
        Self {
            key,
            action
        }
    }
}

impl PlayerInteraction {
    pub fn key(&self) -> &Key {
        &self.key
    }

    pub fn action(&self) -> &Action {
        &self.action
    }
}

#[derive(Debug)]
pub struct Player<'a> {
    position: Position,
    prev_position: Option<Position>,
    size: Size,
    image: &'a str,
    flip: bool,
    movement_value: f32,
    status: PlayerStatus,
    interaction: Option<PlayerInteraction>,
}

impl Player<'_> {
    pub fn new(start_position: Position, flip: bool) -> Self {
        Self {
            position: start_position, 
            prev_position: None,
            size: Size { width: 50.0, height: 60.0 },
            image: "assets/game/detective.png",
            flip,
            movement_value: 10.0,
            status: PlayerStatus::NotHidden,
            interaction: None
        }
    }
}

impl<'a> GameObject<'a> for Player<'a> {
    fn draw(&self, render: &mut Render<'a>) -> Result<()> {
        let opacity = if self.status == PlayerStatus::Hidden {
            Some(0.5)
        } else {
            None
        };

        render.load_image(self.image, self.position, self.size, self.flip, opacity, None, None, None)?;

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
    pub fn get_status(&self) -> &PlayerStatus {
        &self.status
    }

    pub fn set_status(&mut self, new_status: PlayerStatus) {
        self.status = new_status;
    }

    pub fn set_movement_value(&mut self, new_value: f32) {
        self.movement_value = new_value;
    }

    pub fn get_interaction(&self) -> &Option<PlayerInteraction> {
        &self.interaction
    }

    pub fn set_interaction(&mut self, new_value: Option<PlayerInteraction>) {
        self.interaction = new_value;
    }

    pub fn move_player(&mut self, direction: Direction) {
        if self.status != PlayerStatus::Hidden {
            self.prev_position = Some(self.get_position());
    
            self.move_character(direction, self.movement_value);
        }
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

    pub fn is_colliding_with_hide_place(&self, hide_place: &HidePlace) -> bool {
        let start_hide_place_position = round_position_to_full_numbers(hide_place.get_position(), self.movement_value, true, false);
        let end_hide_place_position = round_position_to_full_numbers(start_hide_place_position + hide_place.get_size(), self.movement_value, true, false);
        let start_player_position = round_position_to_full_numbers(self.position, self.movement_value, true, false);
        let end_player_position = self.position + self.size;

        (start_player_position.y == start_hide_place_position.y
        && (
            (start_player_position.x >= start_hide_place_position.x && start_player_position.x <= (start_hide_place_position.x + self.movement_value))
            || (end_player_position.x >= (end_hide_place_position.x - self.movement_value) && end_player_position.x <= end_hide_place_position.x)
        )) || (start_player_position.x == start_hide_place_position.x
        && (
            (start_player_position.y >= start_hide_place_position.y && start_player_position.y <= (start_hide_place_position.y + self.movement_value))
            || (end_player_position.y >= (end_hide_place_position.y - self.movement_value) && end_player_position.y <= end_hide_place_position.y)
        ))
    }

    pub fn throw_form_hide_place(&mut self, walls: &[Wall<'a>], enemy_movement_direction: &Direction) {
        if self.status == PlayerStatus::Hidden {
            let new_value = 60.0;

            let mut can_throw_left = false;
            let mut can_throw_right= false;

            let mut i = 0;

            while i < walls.len() {
                let wall = &walls[i];
                let wall_start = wall.get_position();
                let wall_end = wall_start + wall.get_size();

                let player_end = self.position + self.size;

                if self.position.x > wall_end.x && (self.position.x - wall_end.x) >= new_value {
                    can_throw_left = true;
                }

                if wall_start.x > player_end.x && (wall_start.x - player_end.x) >= new_value {
                    can_throw_right = true;
                }

                i = i + 1;
            }

            if can_throw_left && can_throw_right {
                if enemy_movement_direction == &Direction::Left {
                    self.set_position(Position { x: self.position.x - new_value, y: self.position.y });
                } else {
                    self.set_position(Position { x: self.position.x + new_value, y: self.position.y });
                }
            } else if can_throw_left {
                self.set_position(Position { x: self.position.x - new_value, y: self.position.y });
            } else {
                self.set_position(Position { x: self.position.x + new_value, y: self.position.y });
            }
        }
    }
}
