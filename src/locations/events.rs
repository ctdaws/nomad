use bevy::{
    ecs::{
        event::{Event, EventReader, EventWriter},
        system::{Commands, Query, Res, ResMut},
    },
    math::{Quat, Vec2, Vec3},
    render::view::Visibility,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};

use crate::{
    plugin::PlayerResources,
    ui::{encounter::UpdateEncounter, resources::UpdateResources},
};

use super::{
    location::{
        ConnectedLocations, CurrentLocation, Encounter, LocationState, Locations,
        SpawnedConnections,
    },
    plugin::{
        CURRENT_LOCATION_COLOUR, LOCATION_CONNECTION_COLOUR, LOCATION_CONNECTION_Z,
        NOT_SELECTABLE_LOCATION_COLOUR, SELECTABLE_LOCATION_COLOUR,
    },
};

#[derive(Event)]
pub struct LocationClicked(pub u32);

#[derive(Event)]
pub struct SpawnLocationConnections(pub u32);

#[derive(Event)]
pub struct ShowConnectedLocations(pub u32);

fn clear_old_location_state(
    location_id: u32,
    locations: &Res<Locations>,
    connected_locations_query: &Query<&ConnectedLocations>,
    sprite_and_state_query: &mut Query<(&mut Sprite, &mut LocationState)>,
) {
    let location_entity = locations.0[&location_id];

    let (mut sprite, mut state) = sprite_and_state_query.get_mut(location_entity).unwrap();

    sprite.color = NOT_SELECTABLE_LOCATION_COLOUR;
    *state = LocationState::NotSelectable;

    let connected_locations = connected_locations_query.get(location_entity).unwrap();

    for c in connected_locations.0.clone() {
        let (mut sprite, mut state) = sprite_and_state_query.get_mut(locations.0[&c.0]).unwrap();
        sprite.color = NOT_SELECTABLE_LOCATION_COLOUR;
        *state = LocationState::NotSelectable;
    }
}

fn set_new_location_state(
    location_id: u32,
    locations: &Res<Locations>,
    connected_locations_query: &Query<&ConnectedLocations>,
    sprite_and_state_query: &mut Query<(&mut Sprite, &mut LocationState)>,
) {
    let location_entity = locations.0[&location_id];

    let (mut sprite, mut state) = sprite_and_state_query.get_mut(location_entity).unwrap();

    sprite.color = CURRENT_LOCATION_COLOUR;
    *state = LocationState::Current;

    let connected_locations = connected_locations_query.get(location_entity).unwrap();

    for c in connected_locations.0.clone() {
        let (mut sprite, mut state) = sprite_and_state_query.get_mut(locations.0[&c.0]).unwrap();
        sprite.color = SELECTABLE_LOCATION_COLOUR;
        *state = LocationState::Selectable;
    }
}

pub fn location_clicked(
    locations: Res<Locations>,
    mut current_location: ResMut<CurrentLocation>,
    mut player_resources: ResMut<PlayerResources>,
    mut location_selected_events: EventReader<LocationClicked>,
    mut update_resources_events: EventWriter<UpdateResources>,
    mut update_encounter_events: EventWriter<UpdateEncounter>,
    mut spawn_location_connections_events: EventWriter<SpawnLocationConnections>,
    mut show_connected_locations_events: EventWriter<ShowConnectedLocations>,
    connected_locations_query: Query<&ConnectedLocations>,
    mut sprite_and_state_query: Query<(&mut Sprite, &mut LocationState)>,
    encounter_query: Query<&Encounter>,
) {
    for ev in location_selected_events.read() {
        let (_, state) = sprite_and_state_query.get_mut(locations.0[&ev.0]).unwrap();

        if matches!(
            *state,
            LocationState::Current | LocationState::NotSelectable
        ) {
            return;
        }

        show_connected_locations_events.send(ShowConnectedLocations(ev.0));
        spawn_location_connections_events.send(SpawnLocationConnections(ev.0));

        clear_old_location_state(
            current_location.0,
            &locations,
            &connected_locations_query,
            &mut sprite_and_state_query,
        );

        set_new_location_state(
            ev.0,
            &locations,
            &connected_locations_query,
            &mut sprite_and_state_query,
        );

        player_resources.food -= 1;
        player_resources.water -= 1;

        update_resources_events.send(UpdateResources {
            player_food: Some(player_resources.food),
            player_water: Some(player_resources.water),
            ..Default::default()
        });

        let encounter = encounter_query.get(locations.0[&ev.0]).unwrap();
        update_encounter_events.send(UpdateEncounter {
            text: encounter.text.clone(),
            interactions: encounter.interactions.clone(),
        });

        current_location.0 = ev.0.clone();
    }
}

fn get_connection_params(start: Vec3, end: Vec3) -> (Vec3, f32, f32) {
    // Needs to be end - start
    let between = end - start;

    let mut position = start + between.div_euclid(Vec3::new(2., 2., 1.));
    position.z = LOCATION_CONNECTION_Z;

    let angle = between.y.atan2(between.x);

    (position, between.length(), angle)
}

pub fn spawn_location_connections(
    mut commands: Commands,
    locations: Res<Locations>,
    mut spawned_connections: ResMut<SpawnedConnections>,
    mut spawn_location_connections_events: EventReader<SpawnLocationConnections>,
    connected_locations_query: Query<&ConnectedLocations>,
    transform_query: Query<&Transform>,
) {
    for ev in spawn_location_connections_events.read() {
        let connected_locations = connected_locations_query.get(locations.0[&ev.0]).unwrap();
        let base_location_transform = transform_query.get(locations.0[&ev.0]).unwrap();

        for c in connected_locations.0.clone() {
            if !(spawned_connections.0.contains(&(ev.0, c.0))
                || spawned_connections.0.contains(&(c.0, ev.0)))
            {
                let connected_location_transform = transform_query.get(locations.0[&c.0]).unwrap();
                let (position, length, angle) = get_connection_params(
                    base_location_transform.translation,
                    connected_location_transform.translation,
                );

                commands.spawn(SpriteBundle {
                    transform: Transform::from_translation(position)
                        .with_rotation(Quat::from_rotation_z(angle)),
                    sprite: Sprite {
                        color: LOCATION_CONNECTION_COLOUR,
                        custom_size: Some(Vec2::new(length, 10.)),
                        ..Default::default()
                    },
                    ..Default::default()
                });

                spawned_connections.0.push((ev.0, c.0));
            }
        }
    }
}

pub fn show_connected_locations(
    locations: Res<Locations>,
    mut show_connected_locations_events: EventReader<ShowConnectedLocations>,
    connected_locations_query: Query<&ConnectedLocations>,
    mut visibility_query: Query<&mut Visibility>,
) {
    for ev in show_connected_locations_events.read() {
        let connected_locations = connected_locations_query.get(locations.0[&ev.0]).unwrap();

        for c in connected_locations.0.clone() {
            let mut visibility = visibility_query.get_mut(locations.0[&c.0]).unwrap();
            *visibility = Visibility::Visible;
        }
    }
}
