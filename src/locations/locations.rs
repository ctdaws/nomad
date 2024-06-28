use bevy::{math::Vec3, utils::HashMap};

use super::{
    location::LocationId,
    location_config::{EncounterConfig, LocationConfig},
};

pub fn location_configs() -> HashMap<LocationId, LocationConfig> {
    let mut configs_map = HashMap::new();

    configs_map.insert(
        LocationId(0),
        LocationConfig {
            location_id: LocationId(0),
            position: Vec3::new(-550., -150., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(1)],
        },
    );
    configs_map.insert(
        LocationId(1),
        LocationConfig {
            location_id: LocationId(1),
            position: Vec3::new(-300., 50., 2.),
            encounter: EncounterConfig {
                text: "You find a berry bush by a stream. +10 food +20 water".to_string(),
                food: Some(10),
                water: Some(20),
                wood: None,
            },
            connected_locations: vec![LocationId(0), LocationId(2)],
        },
    );
    configs_map.insert(
        LocationId(2),
        LocationConfig {
            location_id: LocationId(2),
            position: Vec3::new(0., 200., 2.),
            encounter: EncounterConfig {
                text: "You start a fire and have a meal -10 food -10 water -10 wood".to_string(),
                food: Some(-10),
                water: Some(-10),
                wood: Some(-10),
            },
            connected_locations: vec![LocationId(1)],
        },
    );

    configs_map
}
