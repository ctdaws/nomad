use std::fs;

use bevy::{
    ecs::system::{ResMut, Resource},
    utils::HashMap,
};

use super::config::{LocationConfig, LocationConfigs};

#[derive(Hash, Eq, PartialEq)]
pub struct LocationId(pub u32);

#[derive(Resource)]
pub struct CurrentLocation(pub LocationId);

#[derive(Resource, Default)]
pub struct Locations(pub HashMap<LocationId, LocationConfig>);

pub fn load_location_configs() -> HashMap<String, LocationConfig> {
    let content = fs::read_to_string("config/locations.toml").unwrap();

    let location_configs: LocationConfigs = toml::from_str(&content).unwrap();
    location_configs.locations
}

pub fn setup_locations(mut locations: ResMut<Locations>) {
    let location_configs = load_location_configs();

    for (id, config) in location_configs {
        locations.0.insert(LocationId(id.parse().unwrap()), config);
    }
}
