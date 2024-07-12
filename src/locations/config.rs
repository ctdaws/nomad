use std::{fs::File, io::Read};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Deserialize, Debug)]
pub struct Interaction {
    pub text: String,
    // The amount of a resource to add/remove from the player
    pub food: Option<i32>,
    pub water: Option<i32>,
    pub wood: Option<i32>,
    pub unlocks_location: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct Encounter {
    pub text: String,
    pub interactions: Vec<Interaction>,
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
