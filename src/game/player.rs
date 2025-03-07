use glfw::{Action, Key};

use crate::{library::{constants::DEFAULT_MOVEMENT_VALUE, utils::{calculate_calc_position, round_position_to_full_numbers}}, renderer::{error::Result, render::{Render, Size}, vertice::Position}};

use super::{character::{Character, Direction, DEFAULT_CHARACTER_SIZE}, door::{Door, DoorType}, level::{EndStartPositions, GameObject}, level_object::{LevelObject, ObjectType}, wall::Wall};

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
pub struct DoorCollectableInventory {
    id: usize,
    opens: Vec<usize>,
}

#[derive(Debug, Clone)]
pub enum InventoryItemType {
    Weapon,
    TrickCan,
}

pub const DEFAULT_SIZE_FOR_INVENTORY_ITEM: Size<f32> = Size { width: 40.0, height: 40.0 };

#[derive(Debug, Clone)]
pub struct InventoryItem<'a> {
    item_type: InventoryItemType,
    amount: u32, 
    ammo: Option<usize>,
    image: &'a str,
    name: String,
}

impl<'a> InventoryItem<'a> {
    pub fn new(item_type: InventoryItemType, amount: u32, ammo: Option<usize>, image: &'a str, name: String) -> Self {
        Self {
            item_type,
            amount,
            ammo,
            image,
            name
        }
    }
}

impl<'a> InventoryItem<'a> {
    pub fn get_item_type(&self) -> &InventoryItemType {
        &self.item_type
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_amount(&self) -> u32 {
        self.amount
    }

    pub fn get_ammo(&self) -> Option<usize> {
        self.ammo
    }

    pub fn get_image(&self) -> &'a str {
        self.image
    }
}

#[derive(Debug)]
pub struct Player<'a> {
    position: Position,
    prev_position: Option<Position>,
    calc_position: EndStartPositions,
    size: Size,
    image: &'a str,
    flip: bool,
    movement_value: f32,
    status: PlayerStatus,
    interaction: Option<PlayerInteraction>,
    door_collectable_inventory: Vec<DoorCollectableInventory>,
    inventory: Vec<InventoryItem<'a>>,
    holding: Option<usize>,
    coins: u32,
    is_detected_by_enemy: bool,
    is_teleported: bool,
}

impl Player<'_> {
    pub fn new(start_position: Position, flip: bool) -> Self {
        let size = DEFAULT_CHARACTER_SIZE;

        Self {
            position: start_position, 
            prev_position: None,
            calc_position: calculate_calc_position(start_position, size, DEFAULT_MOVEMENT_VALUE),
            size,
            image: "assets/game/detective.png",
            flip,
            movement_value: DEFAULT_MOVEMENT_VALUE,
            status: PlayerStatus::NotHidden,
            interaction: None,
            door_collectable_inventory: Vec::new(),
            inventory: vec![
                InventoryItem::new(InventoryItemType::TrickCan, 25, None, "assets/game/trick-can.png", String::from("Trick Can")),
                InventoryItem::new(InventoryItemType::Weapon, 1, Some(15), "assets/game/camera-gun.webp", String::from("Camera Gun"))
            ],
            holding: None,
            coins: 0,
            is_detected_by_enemy: false,
            is_teleported: false,
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
        self.set_calc_position();
    }

    fn get_size(&self) -> Size {
        self.size
    }

    fn get_calc_position(&self) -> EndStartPositions {
        self.calc_position
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
    
    pub fn get_door_collectable_inventory(&self) -> &[DoorCollectableInventory] {
        &self.door_collectable_inventory
    }

    pub fn add_door_collectable(&mut self, door_collectable_id: usize, opens: &Vec<usize>) {
        self.door_collectable_inventory.push(DoorCollectableInventory { id: door_collectable_id, opens: opens.clone() });
    }
    
    pub fn can_open_door(&self, door: &Door<'a>) -> bool {
        if door.get_door_type() == &DoorType::Regular {
            return true;
        }

        for door_collectable in self.door_collectable_inventory.iter() {
            if door_collectable.id == door.get_opens_by_id().unwrap() || door_collectable.opens.contains(&door.get_id()) {
                return true;
            }
        }

        false
    }

    pub fn get_coins(&self) -> u32 {
        self.coins
    }

    pub fn add_coin(&mut self) {
        self.coins = self.coins + 1;
    }

    pub fn get_movement_value(&self) -> f32 {
        self.movement_value
    }

    pub fn get_is_detected_by_enemy(&self) -> bool {
        self.is_detected_by_enemy
    }

    pub fn set_is_detected_by_enemy(&mut self, new_val: bool) {
        self.is_detected_by_enemy = new_val;
    }
    
    pub fn get_is_teleported(&self) -> bool {
        self.is_teleported
    }

    pub fn set_is_teleported(&mut self, new_val: bool) {
        self.is_teleported = new_val;
    }

    fn set_calc_position(&mut self) {
        self.calc_position = calculate_calc_position(self.position, self.size, self.movement_value);
    }

    pub fn get_holding_item(&self) -> Option<InventoryItem<'a>> {
        if let Some(holding) = self.holding {
            Some(self.inventory[holding].clone())
        } else {
            None
        }
    }

    fn get_next_holding_item_index(&self) -> usize {
        if let Some(holding) = self.holding {
            (holding + 1) % self.inventory.len()
        } else {
            0
        }
    }

    fn get_prev_holding_item_index(&self) -> usize {
        if let Some(holding) = self.holding {
            if holding == 0 {
                return self.inventory.len() - 1;   
            }

            (holding - 1) % self.inventory.len()
        } else {
            0
        }
    }

    pub fn switch_items(&mut self) {
        if let Some(interaction) = &self.interaction {
            match interaction.key() {
                &Key::I => {
                    if interaction.action() == &Action::Repeat {
                        self.holding = None;
                    }
                },

                &Key::K => {
                    if interaction.action() == &Action::Press {
                        self.holding = Some(self.get_next_holding_item_index());
                    }
                },

                &Key::J => {
                    if interaction.action() == &Action::Press {
                        self.holding = Some(self.get_prev_holding_item_index());
                    }
                }

                _ => ()
            }
        }
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
            self.set_calc_position();
        }
    }

    pub fn get_prev_position(&self) -> Option<Position> {
        self.prev_position
    }

    pub fn move_to(&mut self, new_position: Position, flip: bool) {
        self.flip = flip;

        self.position = new_position;
        self.set_calc_position();
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

    pub fn is_colliding_with_object(&self, object: &impl LevelObject<'a>) -> bool {
        if (self.get_calc_position().0 == object.get_calc_position().0) || (self.get_calc_position().1 == object.get_calc_position().1) {
            return true;
        }

        let start_object_position;
        let end_object_position;
        let start_player_position;
        let end_player_position;

        if object.get_type() == ObjectType::HidePlace {
            start_object_position = round_position_to_full_numbers(object.get_position(), self.movement_value, true, false);
            end_object_position = round_position_to_full_numbers(start_object_position + object.get_size(), self.movement_value, true, false);
            start_player_position = round_position_to_full_numbers(self.position, self.movement_value, true, false);
            end_player_position = self.position + self.size;
        } else {
            (start_object_position, end_object_position) = object.get_calc_position();
            (start_player_position, end_player_position) = self.get_calc_position(); 
        }

        let is_collide = | movement_val: f32 | {
            (start_player_position.y == start_object_position.y
                && (
                    (start_player_position.x >= start_object_position.x && start_player_position.x <= (start_object_position.x + movement_val))
                    || (end_player_position.x >= (end_object_position.x - movement_val) && end_player_position.x <= end_object_position.x)
                ))
            || (start_player_position.x == start_object_position.x
                && (
                    (start_player_position.y >= start_object_position.y && start_player_position.y <= (start_object_position.y + movement_val))
                    || (end_player_position.y >= (end_object_position.y - movement_val) && end_player_position.y <= end_object_position.y)
                )
            )
        };

        let is_colliding = is_collide(self.movement_value);

        if !is_colliding {
            return is_collide(self.movement_value * 2.0);
        }

        is_colliding
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
