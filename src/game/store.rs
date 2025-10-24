use std::usize;

use derivative::Derivative;

use super::player::Player;

// TODO: see if to make a trait and different structs for items and upgrades
#[derive(Derivative)]
#[derivative(Debug)]
pub struct StoreItem<'a> {
    pub image_path: &'a str,
    pub title: String,
    pub description: String,
    pub price: usize,
    pub upgrade_info: Option<(usize, usize)>,
    pub error_message: String,

    #[derivative(Debug = "ignore")]
    pub buy_func: Box<dyn FnMut(&mut Player<'_>) -> Result<(), String>>,
}

impl<'a> StoreItem<'a> {
    pub fn get_image_path(&self) -> &'a str {
        self.image_path
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn get_price(&self) -> usize {
        self.price
    }

    pub fn get_upgrade_info(&self) -> Option<(usize, usize)> {
        return self.upgrade_info;
    }

    pub fn get_error_message(&self) -> &String {
        &self.error_message
    }

    pub fn set_error_message(&mut self, new_msg: String) {
        self.error_message = new_msg;
    }

    pub fn buy(&mut self, player: &mut Player<'_>) {
        if (player.get_coins() as usize) < self.get_price() {
            self.error_message = String::from("You do not have enough coins");

            return;
        }

        if let Some(upgrade_info) = self.upgrade_info {
            if upgrade_info.0 >= upgrade_info.1 {
                self.error_message = String::from("You have purchased all updrades");

                return;
            }
        }

        self.error_message = String::new();

        match (self.buy_func)(player) {
            Ok(_) => {
                player.decrease_coins(self.get_price() as u32);

                if let Some(upgrade_info) = self.upgrade_info {
                    self.upgrade_info = Some((upgrade_info.0 + 1, upgrade_info.1));
                }
            }
            Err(msg) => self.error_message = msg,
        };
    }
}
