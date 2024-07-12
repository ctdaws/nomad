use bevy::{
    ecs::{
        event::EventWriter,
        system::{Commands, Query, Res, ResMut},
    },
    math::{Vec2, Vec3},
    render::view::Visibility,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};

use crate::{
    plugin::{GAME_START_INTERACTION_TEXT, GAME_START_TEXT},
    ui::encounter::UpdateEncounter,
};

use super::{
    config::load_locations,
    events::{ShowConnectedLocations, SpawnLocationConnections},
    location::{
        ConnectedLocations, Encounter, Interaction, Location, LocationBundle, LocationId,
        LocationState, Locations, SquareCollider,
    },
    plugin::{
        CURRENT_LOCATION_COLOUR, LOCATION_MARKER_Z, NOT_SELECTABLE_LOCATION_COLOUR,
        SELECTABLE_LOCATION_COLOUR,
    },
};

pub fn setup_locations(
    mut commands: Commands,
    mut locations: ResMut<Locations>,
    mut spawn_location_connections_events: EventWriter<SpawnLocationConnections>,
    mut show_connected_locations_events: EventWriter<ShowConnectedLocations>,
) {
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
                            unlocks_location: i.unlocks_location,
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
                    visibility: Visibility::Hidden,
                    ..Default::default()
                },
            })
            .id();

        locations.0.insert(config.id, entity);
    }

    show_connected_locations_events.send(ShowConnectedLocations(0));
    spawn_location_connections_events.send(SpawnLocationConnections(0));
}

pub fn set_start_location(
    mut query: Query<(
        &mut LocationState,
        &ConnectedLocations,
        &mut Sprite,
        &mut Visibility,
    )>,
    locations: Res<Locations>,
    mut update_encounter_events: EventWriter<UpdateEncounter>,
) {
    let (mut location_state, connected_locations, mut sprite, mut visibility) =
        query.get_mut(locations.0[&0]).unwrap();

    {
        *location_state = LocationState::Current;
        sprite.color = CURRENT_LOCATION_COLOUR;
        *visibility = Visibility::Visible;

        update_encounter_events.send(UpdateEncounter {
            text: GAME_START_TEXT.to_string(),
            interactions: vec![Interaction {
                text: GAME_START_INTERACTION_TEXT.to_string(),
                food: None,
                water: None,
                wood: None,
                unlocks_location: None,
            }],
        });

        for loc in connected_locations.0.clone() {
            let (mut location_state, _, mut sprite, mut visibility) =
                query.get_mut(locations.0[&loc.0]).unwrap();

            *location_state = LocationState::Selectable;
            sprite.color = SELECTABLE_LOCATION_COLOUR;
            *visibility = Visibility::Visible;
        }
    }
}
