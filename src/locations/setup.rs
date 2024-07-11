use std::collections::VecDeque;

use bevy::{
    ecs::{
        event::EventWriter,
        system::{Commands, Query, Res, ResMut},
    },
    math::{Quat, Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::HashMap,
};

use crate::ui::encounter::UpdateEncounter;

use super::{
    config::load_locations,
    location::{
        ConnectedLocations, Encounter, Interaction, Location, LocationBundle, LocationId,
        LocationState, Locations, SquareCollider,
    },
    plugin::{
        CURRENT_LOCATION_COLOUR, LOCATION_CONNECTION_COLOUR, LOCATION_CONNECTION_Z,
        LOCATION_MARKER_Z, NOT_SELECTABLE_LOCATION_COLOUR, SELECTABLE_LOCATION_COLOUR,
    },
};

pub fn setup_locations(mut commands: Commands, mut locations: ResMut<Locations>) {
    let location_configs = load_locations();

    for config in location_configs {
        let connected_locations = config
            .connected_locations
            .iter()
            .map(|c| LocationId(*c))
            .collect::<Vec<_>>();

        let entity = commands
            .spawn(LocationBundle {
                marker: Location,
                id: LocationId(config.id),
                encounter: Encounter {
                    text: config.encounter.text,
                    interactions: config
                        .encounter
                        .interactions
                        .iter()
                        .map(|i| Interaction {
                            text: i.text.clone(),
                            food: i.food,
                            water: i.water,
                            wood: i.wood,
                        })
                        .collect(),
                },
                state: LocationState::NotSelectable,
                connected_locations: ConnectedLocations(connected_locations),
                collider: SquareCollider {
                    half_width: 15.,
                    half_height: 15.,
                },
                sprite: SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(
                        config.position.x as f32,
                        config.position.y as f32,
                        LOCATION_MARKER_Z,
                    )),
                    sprite: Sprite {
                        color: NOT_SELECTABLE_LOCATION_COLOUR,
                        custom_size: Some(Vec2::new(30., 30.)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            })
            .id();

        locations.0.insert(config.id, entity);
    }
}

pub fn setup_location_connections(
    mut commands: Commands,
    locations: Res<Locations>,
    query_connected_locations: Query<&ConnectedLocations>,
    query_transform: Query<&Transform>,
) {
    let mut visited: Vec<u32> = vec![];
    let mut to_visit = VecDeque::<u32>::new();
    to_visit.push_back(0);

    let mut initialised_connections = HashMap::<u32, Vec<u32>>::new();

    while !to_visit.is_empty() {
        let location_id = to_visit.pop_front().unwrap();

        if initialised_connections.get(&location_id).is_none() {
            initialised_connections.insert(location_id, vec![]);
        }

        if !visited.contains(&location_id) {
            let current_location_entity = *locations.0.get(&location_id).unwrap();
            let connected_locations = query_connected_locations
                .get(current_location_entity)
                .unwrap();

            for connected_location in connected_locations.0.clone() {
                if !visited.contains(&connected_location.0) {
                    to_visit.push_back(connected_location.0);

                    let connected_location_entity =
                        *locations.0.get(&connected_location.0).unwrap();

                    if !initialised_connections
                        .get(&connected_location.0)
                        .is_some_and(|locs| locs.contains(&location_id))
                    {
                        let current_location_transform =
                            query_transform.get(current_location_entity).unwrap();
                        let connected_location_transform =
                            query_transform.get(connected_location_entity).unwrap();

                        //init connection
                        // Needs to be dest - start
                        let start_pos = current_location_transform.translation;
                        let dest_pos = connected_location_transform.translation;

                        let between_vec = dest_pos - start_pos;

                        let mut connection_pos =
                            start_pos + between_vec.div_euclid(Vec3::new(2., 2., 1.));
                        connection_pos.z = LOCATION_CONNECTION_Z;

                        let connection_length = between_vec.length();

                        let angle = between_vec.y.atan2(between_vec.x);

                        commands.spawn(SpriteBundle {
                            transform: Transform::from_translation(connection_pos)
                                .with_rotation(Quat::from_rotation_z(angle)),
                            sprite: Sprite {
                                color: LOCATION_CONNECTION_COLOUR,
                                custom_size: Some(Vec2::new(connection_length, 10.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    }

                    if let Some(locs) = initialised_connections.get_mut(&location_id) {
                        locs.push(connected_location.0);
                    }
                }
            }
        }

        visited.push(location_id);
    }
}

pub fn set_start_location(
    mut query: Query<(
        &mut LocationState,
        &ConnectedLocations,
        &mut Sprite,
        &Encounter,
    )>,
    locations: Res<Locations>,
    mut update_encounter_events: EventWriter<UpdateEncounter>,
) {
    let (mut location_state, connected_locations, mut sprite, encounter) =
        query.get_mut(locations.0[&0]).unwrap();

    {
        *location_state = LocationState::Current;
        sprite.color = CURRENT_LOCATION_COLOUR;

        update_encounter_events.send(UpdateEncounter {
            text: encounter.text.clone(),
            interactions: encounter.interactions.clone(),
        });

        for loc in connected_locations.0.clone() {
            let (mut location_state, _, mut sprite, _) =
                query.get_mut(locations.0[&loc.0]).unwrap();

            *location_state = LocationState::Selectable;
            sprite.color = SELECTABLE_LOCATION_COLOUR;
        }
    }
}
