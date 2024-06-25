use std::collections::VecDeque;

use bevy::{
    asset::AssetServer,
    ecs::{
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::With,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    math::Vec3,
    render::color::Color,
    sprite::Sprite,
    utils::HashMap,
};

use super::{
    location::{ConnectedLocations, Encounter, Location, LocationBundle, LocationState},
    location_config::{EncounterConfig, LocationConfig, LocationId},
};

#[derive(Resource, Default)]
pub struct Locations(HashMap<LocationId, Entity>);

#[derive(Event)]
pub struct LocationMoved {
    pub prev: LocationId,
    pub new: LocationId,
}

fn location_configs() -> HashMap<LocationId, LocationConfig> {
    let mut configs_map = HashMap::new();

    configs_map.insert(
        LocationId(0),
        LocationConfig {
            location_id: LocationId(0),
            position: Vec3::new(-550., -150., 1.),
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
            position: Vec3::new(-300., 50., 1.),
            encounter: EncounterConfig {
                text: "You find a berry bush by a stream".to_string(),
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
            position: Vec3::new(0., 200., 1.),
            encounter: EncounterConfig {
                text: "You start a fire and have a meal".to_string(),
                food: Some(-10),
                water: Some(-10),
                wood: Some(-10),
            },
            connected_locations: vec![LocationId(1)],
        },
    );

    configs_map
}

pub fn setup_locations(mut commands: Commands, mut locations: ResMut<Locations>) {
    let configs = location_configs();
    let mut visited: Vec<LocationId> = Vec::<LocationId>::new();
    let mut to_visit = VecDeque::<LocationId>::new();
    to_visit.push_back(LocationId(0));

    while !to_visit.is_empty() {
        let location = to_visit.pop_front().unwrap();

        if !visited.contains(&location) {
            for connected_location in configs[&location].connected_locations.clone() {
                if !visited.contains(&connected_location) {
                    to_visit.push_back(connected_location);
                }
            }

            let entity_id = commands
                .spawn(LocationBundle::new(
                    location.clone(),
                    configs[&location].position,
                    Encounter {
                        text: configs[&location].encounter.text.clone(),
                        food: configs[&location].encounter.food,
                        water: configs[&location].encounter.water,
                        wood: configs[&location].encounter.wood,
                    },
                    ConnectedLocations(configs[&location].connected_locations.clone()),
                ))
                .id();

            locations.0.insert(location.clone(), entity_id);

            visited.push(location);
        }
    }
}

pub fn set_start_location(
    mut query: Query<(&mut LocationState, &ConnectedLocations), With<Location>>,
    locations: Res<Locations>,
    mut location_moved_events: EventWriter<LocationMoved>,
) {
    if let Ok((mut location_state, connected_locations)) =
        query.get_mut(locations.0[&LocationId(0)])
    {
        *location_state = LocationState::Current;

        for loc in connected_locations.0.clone() {
            if let Ok((mut location_state, _)) = query.get_mut(locations.0[&loc]) {
                *location_state = LocationState::Clickable;
            }
        }

        location_moved_events.send(LocationMoved {
            prev: LocationId(0),
            new: LocationId(0),
        });
    }
}

pub fn process_location_moved(
    mut location_moved_events: EventReader<LocationMoved>,
    mut query: Query<(&mut Sprite, &ConnectedLocations), With<Location>>,
    locations: Res<Locations>,
) {
    for ev in location_moved_events.read() {
        if let Ok((mut sprite, connected_locations)) = query.get_mut(locations.0[&ev.prev]) {
            sprite.color = Color::BLACK;

            for loc in connected_locations.0.clone() {
                if let Ok((mut sprite, _)) = query.get_mut(locations.0[&loc]) {
                    sprite.color = Color::BLACK;
                }
            }
        }

        if let Ok((mut sprite, connected_locations)) = query.get_mut(locations.0[&ev.new]) {
            sprite.color = Color::GREEN;

            for loc in connected_locations.0.clone() {
                if let Ok((mut sprite, _)) = query.get_mut(locations.0[&loc]) {
                    println!("GADSF");
                    sprite.color = Color::YELLOW;
                }
            }
        }
    }
}
