use bevy::{math::Vec3, utils::HashMap};

use super::{
    location::LocationId,
    location_config::{EncounterConfig, EncounterOptionConfig, LocationConfig},
};

pub fn location_configs() -> HashMap<LocationId, LocationConfig> {
    let mut configs_map = HashMap::new();

    configs_map.insert(
        LocationId(0),
        LocationConfig {
            location_id: LocationId(0),
            position: Vec3::new(-656., -218., 2.),
            encounter: EncounterConfig {
                text: "Home, nestled between sheer cliffs and a meandering river".to_string(),
                options: vec![EncounterOptionConfig {
                    text: "Venture forth".to_string(),
                    food: None,
                    water: None,
                    wood: None,
                }],
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
                text: "You follow the river and find a dense thicket dotted with blackberries"
                    .to_string(),
                options: vec![
                    EncounterOptionConfig {
                        text: "Pick blackberries +5 food".to_string(),
                        food: Some(5),
                        water: None,
                        wood: None,
                    },
                    EncounterOptionConfig {
                        text: "Continue on".to_string(),
                        food: None,
                        water: None,
                        wood: None,
                    },
                ],
            },
            connected_locations: vec![LocationId(0), LocationId(2), LocationId(9), LocationId(10)],
        },
    );
    configs_map.insert(
        LocationId(2),
        LocationConfig {
            location_id: LocationId(2),
            position: Vec3::new(-596., 150., 2.),
            encounter: EncounterConfig {
                text: "You hear it before you see it, the chattering of insects leads you to a wildflower meadow teeming with life"
                    .to_string(),
                options: vec![EncounterOptionConfig {
                    text: "Continue on".to_string(),
                    food: None,
                    water: None,
                    wood: None,
                }],
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
                text: "You approach the mountain, and find a freshwater stream".to_string(),
                options: vec![
                    EncounterOptionConfig {
                        text: "Fill your waterskins +10 water".to_string(),
                        food: None,
                        water: Some(10),
                        wood: None,
                    },
                    EncounterOptionConfig {
                        text: "Continue on".to_string(),
                        food: None,
                        water: None,
                        wood: None,
                    },
                ],
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
                text: "A steep hill lies to the north-east. It will be a tiring climb, but the vantage point may be worthwhile".to_string(),
                options: vec![EncounterOptionConfig {
                    text: "Continue on".to_string(),
                    food: None,
                    water: None,
                    wood: None,
                }],
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
                text: "You push through a small forest".to_string(),
                options: vec![
                    EncounterOptionConfig {
                        text: "Pick up sticks +5 wood".to_string(),
                        food: None,
                        water: None,
                        wood: Some(5),
                    },
                    EncounterOptionConfig {
                        text: "Continue on".to_string(),
                        food: None,
                        water: None,
                        wood: None,
                    },
                ],
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
                text: "The forest is thick with fallen trees and navigation is difficult. At least there are plenty of sticks to collect".to_string(),
                options: vec![EncounterOptionConfig {
                    text: "Collect sticks -1 food -1 water +7 wood".to_string(),
                    food: Some(-1),
                    water: Some(-1),
                    wood: Some(7),
                }, EncounterOptionConfig {
                    text: "Continue on".to_string(),
                    food: None,
                    water: None,
                    wood: None,
                }],
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
                text: "You climb the hill, it's even steeper than it looked and you stop at the top for food and water. The view from here is incredible, to the south-west a stream can be seen running down the cliffside, and to the east you spot goats resting near the river"
                    .to_string(),
                options: vec![EncounterOptionConfig {
                    text: "Take a break and continue on -1 food -1 water".to_string(),
                    food: Some(-1),
                    water: Some(-1),
                    wood: None,
                }],
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
                text: "The base of a steep hill".to_string(),
                options: vec![EncounterOptionConfig {
                    text: "Continue on".to_string(),
                    food: None,
                    water: None,
                    wood: None,
                }],
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
                text: "You spot fish swimming in the gentle river waters".to_string(),
                options: vec![
                    EncounterOptionConfig {
                        text:
                            "Stop to gather some water and for some spear fishing +6 food +7 water"
                                .to_string(),
                        food: Some(6),
                        water: Some(7),
                        wood: None,
                    },
                    EncounterOptionConfig {
                        text: "Continue on".to_string(),
                        food: None,
                        water: None,
                        wood: None,
                    },
                ],
            },
            connected_locations: vec![LocationId(1), LocationId(8), LocationId(10), LocationId(24)],
        },
    );
    configs_map.insert(
        LocationId(10),
        LocationConfig {
            location_id: LocationId(10),
            position: Vec3::new(-210., -8., 2.),
            encounter: EncounterConfig {
                text: "The river is too wide to cross on foot".to_string(),
                options: vec![EncounterOptionConfig {
                    text: "Craft a makeshift bridge to cross the river -5 wood".to_string(),
                    food: None,
                    water: None,
                    wood: Some(-5),
                }],
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
                text: "You pass a large boulder, covered in moss".to_string(),
                options: vec![EncounterOptionConfig {
                    text: "Continue on".to_string(),
                    food: None,
                    water: None,
                    wood: None,
                }],
            },
            connected_locations: vec![LocationId(10), LocationId(12), LocationId(13)],
        },
    );
    configs_map.insert(
        LocationId(12),
        LocationConfig {
            location_id: LocationId(12),
            position: Vec3::new(-556., -308., 2.),
            encounter: EncounterConfig {
                text:
                    "You spot a cave in the cliff face, but fallen rocks make entrance impossible"
                        .to_string(),
                options: vec![EncounterOptionConfig {
                    text: "Continue on".to_string(),
                    food: None,
                    water: None,
                    wood: None,
                }],
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
                text: "The river bank is overgrown. You spot a kingfisher dart into the water. A small path leads to the river "
                    .to_string(),
                options: vec![EncounterOptionConfig {
                    text: "Fill your waterskins +10 water".to_string(),
                    food: None,
                    water: Some(10),
                    wood: None,
                },
                    EncounterOptionConfig {
                        text: "Continue on".to_string(),
                        food: None,
                        water: None,
                        wood: None,
                    },
                ],
            },
            connected_locations: vec![LocationId(10), LocationId(11), LocationId(14)],
        },
    );
    configs_map.insert(
        LocationId(14),
        LocationConfig {
            location_id: LocationId(14),
            position: Vec3::new(122., -176., 2.),
            encounter: EncounterConfig {
                text: "The outskirts of a forest, you spot mushrooms growing on the forest floor."
                    .to_string(),
                options: vec![
                    EncounterOptionConfig {
                        text: "Forage the mushrooms +4 food".to_string(),
                        food: Some(4),
                        water: None,
                        wood: None,
                    },
                    EncounterOptionConfig {
                        text: "Continue on".to_string(),
                        food: None,
                        water: None,
                        wood: None,
                    },
                ],
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
                text: "Deep in the forest, scattered sticks lie at your feet. ".to_string(),
                options: vec![
                    EncounterOptionConfig {
                        text: "Pick up sticks as you pass through +5 wood".to_string(),
                        food: None,
                        water: None,
                        wood: Some(5),
                    },
                    EncounterOptionConfig {
                        text: "Continue on".to_string(),
                        food: None,
                        water: None,
                        wood: None,
                    },
                ],
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
                text: "You spot large tracks running south-east but you can't identify what animal they come from".to_string(),
                options: vec![EncounterOptionConfig {
                    text: "Continue on".to_string(),
                    food: None,
                    water: None,
                    wood: None,
                }],
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
                text: "You follow the tracks and they lead to a large hole in the ground. Before you have time to think you see a bear emerge from its den. You scramble to escape.".to_string(),
                options: vec![EncounterOptionConfig {
                    text: "Throw some food to distract it -4 food".to_string(),
                    food: Some(-4),
                    water: None,
                    wood: None,
                }],
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
                text: "You pass an enormous charred tree stump, many times wider than the trees which surround it. Its fallen trunk is nowhere to be seen".to_string(),
                options: vec![EncounterOptionConfig {
                    text: "Continue on".to_string(),
                    food: None,
                    water: None,
                    wood: None,
                }],
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
                text: "You pass through a patch hazel trees".to_string(),
                options: vec![
                    EncounterOptionConfig {
                        text: "Harvest the hazelnuts +6 food".to_string(),
                        food: Some(6),
                        water: None,
                        wood: None,
                    },
                    EncounterOptionConfig {
                        text: "Continue on".to_string(),
                        food: None,
                        water: None,
                        wood: None,
                    },
                ],
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
                text: "You pass through a clearing between the forest and the river".to_string(),
                options: vec![EncounterOptionConfig {
                    text: "Continue on".to_string(),
                    food: None,
                    water: None,
                    wood: None,
                }],
            },
            connected_locations: vec![LocationId(22), LocationId(19), LocationId(21)],
        },
    );
    configs_map.insert(
        LocationId(21),
        LocationConfig {
            location_id: LocationId(21),
            position: Vec3::new(322., 226., 2.),
            encounter: EncounterConfig {
                text: "A tree lies fallen and splintered".to_string(),
                options: vec![
                    EncounterOptionConfig {
                        text: "Harvest wood from the tree +10 wood".to_string(),
                        food: None,
                        water: None,
                        wood: Some(10),
                    },
                    EncounterOptionConfig {
                        text: "Continue on".to_string(),
                        food: None,
                        water: None,
                        wood: None,
                    },
                ],
            },
            connected_locations: vec![LocationId(22), LocationId(19), LocationId(20)],
        },
    );
    configs_map.insert(
        LocationId(22),
        LocationConfig {
            location_id: LocationId(22),
            position: Vec3::new(472., 412., 2.),
            encounter: EncounterConfig {
                text:
                    "The river is fast flowing here, a simple makeshift bridge isn't strong enough"
                        .to_string(),
                options: vec![EncounterOptionConfig {
                    text: "Craft a sturdy bridge to cross -8 wood".to_string(),
                    food: None,
                    water: None,
                    wood: Some(-8),
                }],
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
                text: "A herd of goats in resting near the river".to_string(),
                options: vec![
                    EncounterOptionConfig {
                        text: "Sneak up to the herd and hunt a goat +10 food".to_string(),
                        food: Some(10),
                        water: None,
                        wood: None,
                    },
                    EncounterOptionConfig {
                        text: "Continue on".to_string(),
                        food: None,
                        water: None,
                        wood: None,
                    },
                ],
            },
            connected_locations: vec![LocationId(22), LocationId(24)],
        },
    );
    configs_map.insert(
        LocationId(24),
        LocationConfig {
            location_id: LocationId(24),
            position: Vec3::new(218., 372., 2.),
            encounter: EncounterConfig {
                text: "The river is narrower here than upstream, and reeds rustle as the water rushes by".to_string(),
                options: vec![EncounterOptionConfig {
                    text: "Continue on".to_string(),
                    food: None,
                    water: None,
                    wood: None,
                }],
            },
            connected_locations: vec![LocationId(9), LocationId(22), LocationId(23)],
        },
    );

    configs_map
}
