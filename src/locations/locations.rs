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
            position: Vec3::new(-656., -218., 2.),
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
            position: Vec3::new(-462., -34., 2.),
            encounter: EncounterConfig {
                text: "You find a berry bush by a stream. +10 food +20 water".to_string(),
                food: Some(10),
                water: Some(20),
                wood: None,
            },
            connected_locations: vec![LocationId(0), LocationId(2), LocationId(10)],
        },
    );
    configs_map.insert(
        LocationId(2),
        LocationConfig {
            location_id: LocationId(2),
            position: Vec3::new(-596., 150., 2.),
            encounter: EncounterConfig {
                text: "You start a fire and have a meal -10 food -10 water -10 wood".to_string(),
                food: Some(-10),
                water: Some(-10),
                wood: Some(-10),
            },
            connected_locations: vec![LocationId(1), LocationId(3), LocationId(4)],
        },
    );
    configs_map.insert(
        LocationId(3),
        LocationConfig {
            location_id: LocationId(3),
            position: Vec3::new(-846., 114., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(2)],
        },
    );
    configs_map.insert(
        LocationId(4),
        LocationConfig {
            location_id: LocationId(4),
            position: Vec3::new(-450., 320., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(5), LocationId(2), LocationId(7)],
        },
    );
    configs_map.insert(
        LocationId(5),
        LocationConfig {
            location_id: LocationId(5),
            position: Vec3::new(-684., 406., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(6), LocationId(4)],
        },
    );
    configs_map.insert(
        LocationId(6),
        LocationConfig {
            location_id: LocationId(6),
            position: Vec3::new(-894., 332., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(5)],
        },
    );
    configs_map.insert(
        LocationId(7),
        LocationConfig {
            location_id: LocationId(7),
            position: Vec3::new(-270., 488., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(4), LocationId(8)],
        },
    );
    configs_map.insert(
        LocationId(8),
        LocationConfig {
            location_id: LocationId(8),
            position: Vec3::new(-64., 418., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(7), LocationId(9)],
        },
    );
    configs_map.insert(
        LocationId(9),
        LocationConfig {
            location_id: LocationId(9),
            position: Vec3::new(-12., 206., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(8), LocationId(10), LocationId(24)],
        },
    );
    configs_map.insert(
        LocationId(10),
        LocationConfig {
            location_id: LocationId(10),
            position: Vec3::new(-210., -8., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(1), LocationId(9), LocationId(11), LocationId(13)],
        },
    );
    configs_map.insert(
        LocationId(11),
        LocationConfig {
            location_id: LocationId(11),
            position: Vec3::new(-296., -336., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(10), LocationId(12)],
        },
    );
    configs_map.insert(
        LocationId(12),
        LocationConfig {
            location_id: LocationId(12),
            position: Vec3::new(-556., -308., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(11)],
        },
    );
    configs_map.insert(
        LocationId(13),
        LocationConfig {
            location_id: LocationId(13),
            position: Vec3::new(60., 58., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(10), LocationId(14)],
        },
    );
    configs_map.insert(
        LocationId(14),
        LocationConfig {
            location_id: LocationId(14),
            position: Vec3::new(122., -176., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(13), LocationId(15)],
        },
    );
    configs_map.insert(
        LocationId(15),
        LocationConfig {
            location_id: LocationId(15),
            position: Vec3::new(442., -422., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(14), LocationId(16)],
        },
    );
    configs_map.insert(
        LocationId(16),
        LocationConfig {
            location_id: LocationId(16),
            position: Vec3::new(710., -360., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(15), LocationId(17), LocationId(18)],
        },
    );
    configs_map.insert(
        LocationId(17),
        LocationConfig {
            location_id: LocationId(17),
            position: Vec3::new(910., -504., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(16)],
        },
    );
    configs_map.insert(
        LocationId(18),
        LocationConfig {
            location_id: LocationId(18),
            position: Vec3::new(448., -88., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(19), LocationId(16)],
        },
    );
    configs_map.insert(
        LocationId(19),
        LocationConfig {
            location_id: LocationId(19),
            position: Vec3::new(696., 164., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(18), LocationId(20), LocationId(21)],
        },
    );
    configs_map.insert(
        LocationId(20),
        LocationConfig {
            location_id: LocationId(20),
            position: Vec3::new(682., 440., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(22), LocationId(19)],
        },
    );
    configs_map.insert(
        LocationId(21),
        LocationConfig {
            location_id: LocationId(21),
            position: Vec3::new(322., 226., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(22), LocationId(19)],
        },
    );
    configs_map.insert(
        LocationId(22),
        LocationConfig {
            location_id: LocationId(22),
            position: Vec3::new(472., 412., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![
                LocationId(20),
                LocationId(21),
                LocationId(23),
                LocationId(24),
            ],
        },
    );
    configs_map.insert(
        LocationId(23),
        LocationConfig {
            location_id: LocationId(23),
            position: Vec3::new(458., 504., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(22)],
        },
    );
    configs_map.insert(
        LocationId(24),
        LocationConfig {
            location_id: LocationId(24),
            position: Vec3::new(218., 372., 2.),
            encounter: EncounterConfig {
                text: "Home".to_string(),
                food: None,
                water: None,
                wood: None,
            },
            connected_locations: vec![LocationId(9), LocationId(22)],
        },
    );

    configs_map
}
