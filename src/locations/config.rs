use std::{fs::File, io::Read};

use bevy::utils::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Deserialize, Debug)]
pub struct ButtonConfig {
    pub text: String,

    // The amount of a resource to add/remove from the player
    pub food: Option<i32>,
    pub water: Option<i32>,
    pub wood: Option<i32>,

    pub unlocks_location: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct EncounterLevel {
    pub encounter_text: String,
    pub button_config: Option<ButtonConfig>,
}

fn can_ignore_encounter_default() -> bool {
    true
}

fn should_regenerate_level_default() -> bool {
    true
}

#[derive(Deserialize, Debug)]
pub struct Encounter {
    pub starting_level: u32,
    pub levels: HashMap<String, EncounterLevel>,
    #[serde(default = "can_ignore_encounter_default")]
    pub can_ignore_encounter: bool,
    #[serde(default = "should_regenerate_level_default")]
    pub should_regenerate_level: bool,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    pub id: u32,
    pub position: Position,
    pub connected_locations: Vec<u32>,
    pub encounter: Encounter,
}

#[derive(Deserialize, Debug)]
pub struct Locations {
    pub data: Vec<Location>,
}

pub fn load_locations() -> Vec<Location> {
    let mut locations = String::new();

    File::open("config/locations.json")
        .unwrap()
        .read_to_string(&mut locations)
        .unwrap();

    let locations = serde_json::from_str::<Locations>(&locations).unwrap();
    locations.data
}
