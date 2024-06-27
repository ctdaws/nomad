use std::collections::VecDeque;

use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::With,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    math::{Vec2, Vec3},
    render::color::Color,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::HashMap,
};

use crate::{
    ui::{encounter::UpdateEncounter, resources::UpdateResources},
    PlayerResources,
};

use super::locations::location_configs;

#[derive(Resource, Default)]
pub struct Locations(HashMap<LocationId, Entity>);

#[derive(Resource)]
pub struct CurrentLocation(pub LocationId);

#[derive(Event)]
pub struct LocationSelected(pub LocationId);

#[derive(Component)]
pub struct Location;

#[derive(Component, Eq, PartialEq, Hash, Clone, Debug)]
pub struct LocationId(pub u32);

#[derive(Component)]
pub struct Encounter {
    pub text: String,
    // The amount of a resource to add/remove from the player
    pub food: Option<i32>,
    pub water: Option<i32>,
    pub wood: Option<i32>,
}

#[derive(Component)]
pub struct SquareCollider {
    pub half_width: f32,
    pub half_height: f32,
}

#[derive(Component)]
pub enum LocationState {
    Current,
    Selectable,
    NotSelectable,
}

#[derive(Component)]
pub struct ConnectedLocations(pub Vec<LocationId>);

#[derive(Bundle)]
pub struct LocationBundle {
    pub marker: Location,
    pub id: LocationId,

    pub encounter: Encounter,
    pub state: LocationState,
    pub connected_locations: ConnectedLocations,

    pub collider: SquareCollider,
    pub sprite: SpriteBundle,
}

impl LocationBundle {
    pub fn new(
        id: LocationId,
        position: Vec3,
        encounter: Encounter,
        connected_locations: ConnectedLocations,
    ) -> Self {
        LocationBundle {
            marker: Location,
            id,
            encounter,
            collider: SquareCollider {
                half_width: 25.,
                half_height: 25.,
            },
            state: LocationState::NotSelectable,
            connected_locations,
            sprite: SpriteBundle {
                transform: Transform::from_translation(position),
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(50., 50.)),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
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
    mut query: Query<
        (
            &mut LocationState,
            &ConnectedLocations,
            &mut Sprite,
            &Encounter,
        ),
        With<Location>,
    >,
    start_location: Res<CurrentLocation>,
    locations: Res<Locations>,
    mut location_moved_events: EventWriter<LocationSelected>,
    mut update_encounter_events: EventWriter<UpdateEncounter>,
) {
    if let Ok((mut location_state, connected_locations, mut sprite, encounter)) =
        query.get_mut(locations.0[&LocationId(0)])
    {
        *location_state = LocationState::Current;
        sprite.color = Color::GREEN;

        update_encounter_events.send(UpdateEncounter(encounter.text.clone()));
        location_moved_events.send(LocationSelected(start_location.0.clone()));

        for loc in connected_locations.0.clone() {
            if let Ok((mut location_state, _, mut sprite, _)) = query.get_mut(locations.0[&loc]) {
                *location_state = LocationState::Selectable;
                sprite.color = Color::YELLOW;
            }
        }
    }
}

pub fn location_selected(
    mut current_location: ResMut<CurrentLocation>,
    locations: Res<Locations>,
    mut player_resources: ResMut<PlayerResources>,
    mut evs: EventReader<LocationSelected>,
    mut update_resources_events: EventWriter<UpdateResources>,
    mut update_encounter_events: EventWriter<UpdateEncounter>,
    mut query: Query<
        (
            &mut Sprite,
            &ConnectedLocations,
            &Encounter,
            &mut LocationState,
        ),
        With<Location>,
    >,
) {
    for ev in evs.read() {
        if let Ok((_, _, _, state)) = query.get_mut(locations.0[&ev.0]) {
            if matches!(
                *state,
                LocationState::Current | LocationState::NotSelectable
            ) {
                return;
            }
        }

        player_resources.food -= 10;
        player_resources.water -= 10;
        player_resources.wood -= 10;

        if let Ok((mut sprite, connected_locations, _, mut state)) =
            query.get_mut(locations.0[&current_location.0])
        {
            sprite.color = Color::BLACK;
            *state = LocationState::NotSelectable;

            for loc in connected_locations.0.clone() {
                if let Ok((mut sprite, _, _, mut state)) = query.get_mut(locations.0[&loc]) {
                    sprite.color = Color::BLACK;
                    *state = LocationState::NotSelectable;
                }
            }
        }

        if let Ok((mut sprite, connected_locations, encounter, mut state)) =
            query.get_mut(locations.0[&ev.0])
        {
            sprite.color = Color::GREEN;
            *state = LocationState::Current;

            if let Some(food) = encounter.food {
                player_resources.food += food
            }

            if let Some(water) = encounter.water {
                player_resources.water += water
            }

            if let Some(wood) = encounter.wood {
                player_resources.wood += wood
            }

            update_resources_events.send(UpdateResources {
                food: Some(player_resources.food),
                water: Some(player_resources.water),
                wood: Some(player_resources.wood),
            });

            update_encounter_events.send(UpdateEncounter(encounter.text.clone()));

            for loc in connected_locations.0.clone() {
                if let Ok((mut sprite, _, _, mut state)) = query.get_mut(locations.0[&loc]) {
                    sprite.color = Color::YELLOW;
                    *state = LocationState::Selectable;
                }
            }
        }

        current_location.0 = ev.0.clone();
    }
}
