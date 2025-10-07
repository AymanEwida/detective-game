use std::{time::Duration, usize};

use glfw::{Action, Key, MouseButton};

use crate::{library::{constants::DEFAULT_MOVEMENT_VALUE, utils::{calculate_calc_position, is_position_in_border, length_of_line, round_position_to_full_numbers}}, renderer::{color::Color, error::Result, render::Render, styles::Size, vertice::Position}};

use super::{bullet::{Bullet, BulletType}, can::Can, character::{Character, Direction, DEFAULT_CHARACTER_SIZE}, door::{Door, DoorType}, level::{EndStartPositions, GameObject}, level_object::{LevelObject, ObjectType}, wall::Wall};

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
pub struct PlayerMouseInteraction {
    mouse_button: MouseButton,
    action: Action,
    cursor_position: Position, 
}

impl PlayerMouseInteraction {
    pub fn new(mouse_button: MouseButton, action: Action, cursor_position: Position) -> Self {
        Self {
            mouse_button,
            action,
            cursor_position
        }
    }
}

impl PlayerMouseInteraction {
    pub fn get_mouse_button(&self) -> &MouseButton {
        &self.mouse_button
    }

    pub fn get_action(&self) -> &Action {
        &self.action
    }

    pub fn get_cursor_position(&self) -> Position {
        self.cursor_position
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
    original_amount: u32,
    amount: u32,
    original_ammo: Option<usize>,
    ammo: Option<usize>,
    image: &'a str,
    name: String,
}

impl<'a> InventoryItem<'a> {
    pub fn new(item_type: InventoryItemType, amount: u32, ammo: Option<usize>, image: &'a str, name: String) -> Self {
        Self {
            item_type,
            original_amount: amount,
            amount,
            original_ammo: ammo,
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

    pub fn increase_amount(&mut self, num: u32) {
        self.amount += num;
    }

    pub fn decrease_amount(&mut self, num: u32) {
        if self.amount < num {
            self.amount = 0;
        } else {
            self.amount -= num;
        }
    }

    pub fn increase_ammo(&mut self, num: usize) {
        if self.ammo.is_some() {
            self.ammo = Some(self.ammo.unwrap() + num);
        }
    }

    pub fn decrease_ammo(&mut self, num: usize) { 
        if self.ammo.is_some() {
            let ammo = self.ammo.unwrap();

            if ammo < num {
                self.ammo = Some(0);
            } else {
                self.ammo = Some(ammo - num);
            }
        }
    }

    pub fn set_amount_to_original(&mut self) {
        self.amount = self.original_amount;
    }

    pub fn set_ammo_to_original(&mut self) {
        self.ammo = self.original_ammo;
    }
}

#[derive(Debug)]
pub enum ShootObject<'a> {
    Can(Can<'a>),
    Bullet(Bullet<'a>),
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
    mouse_interaction: Option<PlayerMouseInteraction>,
    door_collectable_inventory: Vec<DoorCollectableInventory>,
    inventory: Vec<InventoryItem<'a>>,
    holding: Option<usize>,
    camera_disturb_lifttime: Duration,
    notoriety_camera_disturb_lifttime: Duration,
    coins: u32,
    is_detected_by_enemy: bool,
    seen_by_enemies: Vec<usize>,
    is_teleported: bool,
    can_detecting_radius: f32,
    ability_radius: f32,
    is_using_ability: bool,
    track_path_ability: bool,
    enemy_wait_time_on_trict_can: u64, // milliseconds
    lifes: u8,
    original_lifes: u8,
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
            mouse_interaction: None,
            door_collectable_inventory: Vec::new(),
            inventory: vec![
                InventoryItem::new(InventoryItemType::TrickCan, 30, None, "assets/game/trick-can.png", String::from("Trick Can")),
                InventoryItem::new(InventoryItemType::Weapon, 1, Some(30), "assets/game/camera-gun.webp", String::from("Camera Gun"))
            ],
            holding: None,
            camera_disturb_lifttime: Duration::from_secs(10),
            notoriety_camera_disturb_lifttime: Duration::from_secs(10),
            coins: 0,
            is_detected_by_enemy: false,
            seen_by_enemies: Vec::new(),
            is_teleported: false,
            can_detecting_radius: 100.0,
            ability_radius: 150.0,
            is_using_ability: false,
            track_path_ability: false,
            enemy_wait_time_on_trict_can: 6000,
            lifes: 5,
            original_lifes: 5,
        }
    }
}

impl<'a> GameObject<'a> for Player<'a> {
    fn draw(&self, render: &mut Render<'a>) -> Result<()> {
        if self.get_is_using_ability() {
            let center = Position {
                x: self.get_position().x + self.get_size().width / 2.0,
                y: self.get_position().y + self.get_size().height / 2.0,
            };

            render.draw_geometric_object(center, self.get_ability_radius(), Color::RGBA(0, 255, 0, 50), None, None, None, None);
        }

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

    pub fn get_mouse_interaction(&self) -> &Option<PlayerMouseInteraction> {
        &self.mouse_interaction
    }

    pub fn set_interaction(&mut self, new_value: Option<PlayerInteraction>) {
        self.interaction = new_value;
    }

    pub fn set_mouse_interaction(&mut self, new_value: Option<PlayerMouseInteraction>) {
        self.mouse_interaction = new_value;
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

    pub fn decrease_coins(&mut self, num: u32) {
        if self.get_coins() <= num {
            self.coins = 0;
        } else {
            self.coins = self.coins - num;
        }
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

    pub fn get_can_detecting_radius(&self) -> f32 {
        self.can_detecting_radius
    }

    pub fn set_can_detecting_radius(&mut self, new_radius: f32) {
        self.can_detecting_radius = new_radius;
    }

    pub fn get_camera_disturb_lifttime(&self) -> Duration {
        self.camera_disturb_lifttime
    }

    pub fn set_camera_disturb_lifttime(&mut self, new_secs: u64) {
        self.camera_disturb_lifttime = Duration::from_secs(new_secs);
    }

    pub fn get_notoriety_camera_disturb_lifttime(&self) -> Duration {
        self.notoriety_camera_disturb_lifttime
    }

    pub fn set_notoriety_camera_disturb_lifttime(&mut self, notoriety_level: u64) {
        if notoriety_level >= 4 {
            if notoriety_level == 4 {
                self.notoriety_camera_disturb_lifttime = Duration::from_secs(self.camera_disturb_lifttime.as_secs() - 1);
            } else {
                self.notoriety_camera_disturb_lifttime = Duration::from_secs(self.camera_disturb_lifttime.as_secs() - 2);
            }
        }
    }


    pub fn get_ability_radius(&self) -> f32 {
        self.ability_radius
    }

    pub fn set_ability_radius(&mut self, new_radius: f32) {
        self.ability_radius = new_radius;
    }

    pub fn get_is_using_ability(&self) -> bool {
        self.is_using_ability
    }

    pub fn set_is_using_ability(&mut self, new_val: bool) {
        self.is_using_ability = new_val;
    }
    
    pub fn get_track_path_ability(&self) -> bool {
        self.track_path_ability
    }

    pub fn set_track_path_ability(&mut self, new_val: bool) {
        self.track_path_ability = new_val;
    }

    pub fn get_seen_by_enemies(&self) -> &[usize] {
        self.seen_by_enemies.as_slice()
    }

    pub fn get_is_seen_by_enemy(&self) -> bool {
        self.seen_by_enemies.len() > 0
    }

    pub fn get_seen_by_enemy_id(&self, enemy_id: usize) -> bool {
        self.seen_by_enemies.contains(&enemy_id)
    }

    pub fn add_seen_enemy(&mut self, enemy_id: usize) {
        if !self.get_seen_by_enemy_id(enemy_id) {
            self.seen_by_enemies.push(enemy_id);
        }
    }

    pub fn remove_seen_enemy(&mut self, enemy_id: usize) {
        let mut idx = -1;
        
        for i in 0..self.seen_by_enemies.len() {
            let seen_enemy_id = self.seen_by_enemies[i];

            if seen_enemy_id == enemy_id {
                idx = i as i32;
            }
        }

        if idx != -1 {
            self.seen_by_enemies.swap_remove(idx as usize);
        }
    }

    pub fn get_enemy_wait_time_on_trict_can(&self) -> u64 {
        self.enemy_wait_time_on_trict_can
    }

    pub fn set_enemy_wait_time_on_trict_can(&mut self, new_val: u64) {
        self.enemy_wait_time_on_trict_can = new_val;
    }

    pub fn get_lifes(&self) -> u8 {
        self.lifes
    }

    pub fn set_lifes(&mut self, new_val: u8) {
        self.lifes = new_val;
    }

    pub fn decrease_life(&mut self) {
        if self.lifes != 0 {
            self.lifes -= 1;
        }
    }

    pub fn set_lifes_to_original_lifes(&mut self) {
        self.lifes = self.original_lifes;
    }

    pub fn get_original_lifes(&self) -> u8 {
        self.original_lifes
    }

    pub fn set_original_lifes(&mut self, new_val: u8) {
        self.original_lifes = new_val;
    }

    pub fn reset_props(&mut self) {
        self.seen_by_enemies = Vec::new();
        self.is_teleported = false;
        self.is_using_ability = false;
        self.is_detected_by_enemy = false;
        self.set_status(PlayerStatus::NotHidden);
    }

    pub fn reset_props_for_new_level(&mut self) {
        self.set_lifes_to_original_lifes();
        self.reset_inventory_amounts();

        self.door_collectable_inventory = Vec::new();
        self.holding = None;
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

    pub fn reset_inventory_amounts(&mut self) {
        for item in self.inventory.iter_mut() {
            item.set_amount_to_original();
            item.set_ammo_to_original();
        }
    }

    pub fn add_inventory_amounts(&mut self, num: usize) {
        for item in self.inventory.iter_mut() {
            match item.get_item_type() {
                InventoryItemType::TrickCan => {
                    item.increase_amount(num as u32);
                },
                
                InventoryItemType::Weapon => {
                    item.increase_ammo(num);
                }
            }
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

    pub fn shoot(&mut self, window_start: Position, window_size: Size, render: &mut Render<'a>) -> Option<ShootObject<'a>> {
        let holding_item = self.get_holding_item();

        if let Some(item) = holding_item {
            if let Some(mouse_interaction) = self.get_mouse_interaction() {
                let window_end = window_start + window_size;

                let start_position = if self.flip {
                    Position { x: self.position.x + self.size.width, y: self.position.y + 15.0 }
                } else {
                    Position { x: self.position.x, y: self.position.y + 15.0 }
                };

                let mut calc_start_position = Position { x: self.position.x + (self.size.width / 2.0), y: self.position.y };

                let mut end_position = mouse_interaction.get_cursor_position();

                let length = length_of_line(&start_position, &end_position);

                match item.get_item_type() {
                    InventoryItemType::TrickCan => {
                        match mouse_interaction.get_mouse_button() {
                            &MouseButton::Button1 => {
                                match mouse_interaction.get_action() {
                                    &Action::Press => {
                                        if length > 300.0 {
                                            let direction = (end_position - start_position).normalize(&window_start);

                                            end_position = start_position + (direction * 300.0);
                                        }

                                        let window_check = is_position_in_border(&window_start, &window_end, &end_position);

                                        if !window_check.0 {
                                            if end_position.x < window_start.x {
                                                end_position.x = window_start.x;
                                            } else {
                                                end_position.x = window_end.x;
                                            }
                                        }

                                        if !window_check.1 {
                                            if end_position.y < window_start.y {
                                                end_position.y = window_start.y;
                                            } else {
                                                end_position.y = window_end.y;
                                            }
                                        }

                                        if end_position.x > start_position.x {
                                            calc_start_position.x = self.position.x + self.size.width;
                                        } else if end_position.x < start_position.x {
                                            calc_start_position.x = self.position.x;
                                        }

                                        if end_position.y > start_position.y {
                                            calc_start_position.y = self.position.y + self.size.height;
                                        }

                                        render.draw_curved_line(start_position, end_position, Color::Green, None, None, None, None);
                                    },

                                    &Action::Release => {
                                        if item.amount == 0 {
                                            return None;
                                        }

                                        self.inventory[self.holding.unwrap()].decrease_amount(1);

                                        return Some(ShootObject::Can(Can::new(calc_start_position, end_position, "assets/game/trick-can.png", self.can_detecting_radius)));
                                    },

                                    _ => ()
                                }
                            },

                            _ => ()
                        }
                    },

                    InventoryItemType::Weapon => {
                        match mouse_interaction.get_mouse_button() {
                            &MouseButton::Button1 => {
                                if self.status != PlayerStatus::Hidden {
                                    match mouse_interaction.get_action() {
                                        &Action::Press => {
                                            if length > 150.0 {
                                                let direction = (end_position - start_position).normalize(&window_start);

                                                end_position = start_position + (direction * 150.0);
                                            }

                                            render.draw_line(start_position, end_position, Color::Green, None, None, None);
                                        },

                                        &Action::Release => {
                                            if item.ammo.is_none() {
                                                return None;
                                            }

                                            let ammo = item.ammo.unwrap();

                                            if ammo == 0 {
                                                return None;
                                            }

                                            self.inventory[self.holding.unwrap()].decrease_ammo(1);

                                            return Some(ShootObject::Bullet(Bullet::new(BulletType::CameraGunBullet, 10, "assets/game/bullet.png", None, start_position, end_position, DEFAULT_MOVEMENT_VALUE / 2.0)));
                                        },

                                        _ => ()
                                    }
                                }
                            },

                            _ => ()
                        }

                    },
                }
            }
        }

        None
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
            // (start_player_position.y == start_object_position.y
            //     && (
            //         (start_player_position.x >= start_object_position.x && start_player_position.x <= (start_object_position.x + movement_val))
            //         || (end_player_position.x >= (end_object_position.x - movement_val) && end_player_position.x <= end_object_position.x)
            //     ))
            // || (start_player_position.x == start_object_position.x
            //     && (
            //         (start_player_position.y >= start_object_position.y && start_player_position.y <= (start_object_position.y + movement_val))
            //         || (end_player_position.y >= (end_object_position.y - movement_val) && end_player_position.y <= end_object_position.y)
            //     )
            
            let covers_object_x = 
                    (start_player_position.x >= start_object_position.x && start_player_position.x <= (start_object_position.x + movement_val))
                    || (end_player_position.x >= (end_object_position.x - movement_val) && end_player_position.x <= end_object_position.x);

            let covers_object_y = 
                    (start_player_position.y >= start_object_position.y && start_player_position.y <= (start_object_position.y + movement_val))
                    || (end_player_position.y >= (end_object_position.y - movement_val) && end_player_position.y <= end_object_position.y);

            ((start_player_position.y == start_object_position.y || covers_object_y) && covers_object_x)
                || ((start_player_position.x == start_object_position.x || covers_object_x) && covers_object_y)
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
