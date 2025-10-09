use std::{collections::HashMap, isize, usize};

use super::player::Player;

pub const COIN_COLLECTABLE_CHALLENGE_TYPE: &str = "coin_collectable";
pub const DETECTION_CHALLENGE_TYPE: &str = "detection";
pub const KILLING_ENEMIES_CHALLENGE_TYPE: &str = "killing_enemies";
pub const DOOR_COLLECTABLE_CHALLENGE_TYPE: &str = "door_collectable";
pub const TRICK_ENEMIES_CHALLENGE_TYPE: &str = "trick_enemies";
pub const NOTORIETY_LEVEL_CHALLENGE_TYPE: &str = "notoriety_level";
pub const DISTURB_CAMERAS_CHALLENGE_TYPE: &str = "disturb_cameras";
pub const ONLY_GUNS_CHALLENGE_TYPE: &str = "only_guns";
pub const LEVEL_TRIES_CHALLENGE_TYPE: &str = "level_tries";

#[derive(Debug, PartialEq, Eq)]
pub enum ChallengeStatus {
    NotDetermine,
    Completed,
    Failed
}

#[derive(Debug)]
pub struct Challenge {
    challenge_text: String,
    status: ChallengeStatus,
    metadata: HashMap<String, String>,
}

impl Challenge {
    pub fn new(challenge_text: String, metadata_string: String) -> Self {
        let metadata: HashMap<String, String> = metadata_string.split("&").into_iter().map(| param: &str | {
            let param_data: Vec<&str> = param.split("=").collect();

            (param_data[0].to_string(), param_data[1].to_string())
        }).collect(); 

        Self {
            challenge_text,
            status: ChallengeStatus::NotDetermine,
            metadata,
        }
    }
}

pub fn check_compare(compare_string: &String, lhs: isize, rhs: isize) -> bool {
    if compare_string == "at_most" {
        return lhs <= rhs;
    } else if compare_string == "at_least" {
        return lhs >= rhs;
    }

    lhs == rhs 
}

impl Challenge {
    pub fn get_challenge_text(&self) -> &String {
        &self.challenge_text
    }

    pub fn get_status(&self) -> &ChallengeStatus {
        &self.status
    }

    pub fn set_status(&mut self, new_val: ChallengeStatus) {
        self.status = new_val;
    }

    pub fn get_reward(&self) -> usize {
        assert!(self.metadata.get("reward") != None, "reward can not be none");

        self.metadata.get("reward").unwrap().parse::<usize>().unwrap()
    }

    pub fn check_challenge(&self, player: &Player<'_>, is_check_at_complete: bool, notoriety_level: u64) -> ChallengeStatus {
        assert!(self.metadata.get("type") != None, "type can not be none");

        let challenge_type = self.metadata.get("type").unwrap();

        let at_complete_default_value = String::from("false");
        let at_complete = self.metadata.get("at_complete").unwrap_or(&at_complete_default_value);
        let count = self.metadata.get("count").unwrap_or(&String::from("0"))
            .parse::<isize>().unwrap_or(0);

        let compare_default_value = String::from("exact");
        let compare = self.metadata.get("compare").unwrap_or(&compare_default_value);

        match challenge_type.as_str() {
            DETECTION_CHALLENGE_TYPE => {
                if !is_check_at_complete && at_complete == "true" {
                    if !check_compare(compare, player.get_detect_count(), count) {
                        return ChallengeStatus::Failed;
                    }

                    return ChallengeStatus::NotDetermine;
                }

                if check_compare(compare, player.get_detect_count(), count) {
                    return ChallengeStatus::Completed;    
                } else if is_check_at_complete {
                    return ChallengeStatus::Failed;
                }

                return ChallengeStatus::NotDetermine;
            },

            ONLY_GUNS_CHALLENGE_TYPE => {
                if !is_check_at_complete && at_complete == "true" {
                    if !player.is_used_only_guns() {
                        return ChallengeStatus::Failed;
                    }

                    return ChallengeStatus::NotDetermine;
                }

                if player.is_used_only_guns() {
                    return ChallengeStatus::Completed;
                } else if is_check_at_complete {
                    return ChallengeStatus::Failed;
                }

                return ChallengeStatus::NotDetermine;
            },

            DISTURB_CAMERAS_CHALLENGE_TYPE => {
                if !is_check_at_complete && at_complete == "true" {
                    if !check_compare(compare, player.get_disturb_cameras_count(), count) {
                        return ChallengeStatus::Failed;
                    }

                    return ChallengeStatus::NotDetermine;
                }

                if check_compare(compare, player.get_disturb_cameras_count(), count) {
                    return ChallengeStatus::Completed;
                } else if is_check_at_complete {
                    return ChallengeStatus::Failed;
                }

                return ChallengeStatus::NotDetermine;
            },

            KILLING_ENEMIES_CHALLENGE_TYPE => {
                if !is_check_at_complete && at_complete == "true" {
                    if !check_compare(compare, player.get_enemies_killed_count(), count) {
                        return ChallengeStatus::Failed;
                    }

                    return ChallengeStatus::NotDetermine;
                }

                if check_compare(compare, player.get_enemies_killed_count(), count) {
                    return ChallengeStatus::Completed;
                } else if is_check_at_complete {
                    return ChallengeStatus::Failed;
                }

                return ChallengeStatus::NotDetermine;
            },

            COIN_COLLECTABLE_CHALLENGE_TYPE => {
                if !is_check_at_complete && at_complete == "true" {
                    if !check_compare(compare, player.get_coins() as isize, count) {
                        return ChallengeStatus::Failed;
                    }

                    return ChallengeStatus::NotDetermine;
                }

                if check_compare(compare, player.get_detect_count(), count) {
                    return ChallengeStatus::Completed;    
                } else if is_check_at_complete {
                    return ChallengeStatus::Failed;
                }

                return ChallengeStatus::NotDetermine;
            },

            DOOR_COLLECTABLE_CHALLENGE_TYPE => {
                let collectable_type = self.metadata.get("collectable_type").unwrap();

                if !is_check_at_complete && at_complete == "true" {
                    if !check_compare(compare, player.get_door_collectable_count(collectable_type), count) {
                        return ChallengeStatus::Failed;
                    } 

                    return ChallengeStatus::NotDetermine;
                }

                if check_compare(compare, player.get_door_collectable_count(collectable_type), count) {
                    return ChallengeStatus::Completed;
                } else if is_check_at_complete {
                    return ChallengeStatus::Failed;
                }

                return ChallengeStatus::NotDetermine;
            },

            LEVEL_TRIES_CHALLENGE_TYPE => { 
                if !is_check_at_complete && at_complete == "true" {
                    if !check_compare(compare, player.get_level_tries(), count) {
                        return ChallengeStatus::Failed;
                    }


                    return ChallengeStatus::NotDetermine;
                }

                if check_compare(compare, player.get_level_tries(), count) {
                    return ChallengeStatus::Completed;
                } else if is_check_at_complete {
                    return ChallengeStatus::Failed;
                }
                
                return ChallengeStatus::NotDetermine;
            },

            TRICK_ENEMIES_CHALLENGE_TYPE => {
                if !is_check_at_complete && at_complete == "true" {
                    if !check_compare(compare, player.get_enemies_trick_count(), count) {
                        return ChallengeStatus::Failed;
                    }

                    return ChallengeStatus::NotDetermine;
                }

                if check_compare(compare, player.get_enemies_trick_count(), count) {
                    return ChallengeStatus::Completed;
                } else if is_check_at_complete {
                    return ChallengeStatus::Failed;
                }

                return ChallengeStatus::NotDetermine;
            },

            NOTORIETY_LEVEL_CHALLENGE_TYPE => {
                if !is_check_at_complete && at_complete == "true" {
                    if !check_compare(compare, notoriety_level as isize, count) {
                        return ChallengeStatus::Failed;
                    }

                    return ChallengeStatus::NotDetermine;
                } 

                if check_compare(compare, notoriety_level as isize, count) {
                    return ChallengeStatus::Completed;
                } else if is_check_at_complete {
                    return ChallengeStatus::Failed;
                }

                return ChallengeStatus::NotDetermine;
            },

            _ => ()
        }

        ChallengeStatus::NotDetermine
    }
}

