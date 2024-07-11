use bevy::{
    ecs::{
        event::{EventReader, EventWriter},
        query::With,
        system::{Query, Res, ResMut},
    },
    render::view::Visibility,
    sprite::Sprite,
};

use crate::{
    plugin::PlayerResources,
    ui::{
        encounter::{EncounterUI, UpdateEncounter},
        resources::UpdateResources,
    },
};

use super::{
    location::{
        ConnectedLocations, CurrentLocation, Encounter, Location, LocationSelected, LocationState,
        Locations,
    },
    plugin::{CURRENT_LOCATION_COLOUR, NOT_SELECTABLE_LOCATION_COLOUR, SELECTABLE_LOCATION_COLOUR},
};

pub fn location_selected(
    mut current_location: ResMut<CurrentLocation>,
    locations: Res<Locations>,
    mut player_resources: ResMut<PlayerResources>,
    mut location_selected_events: EventReader<LocationSelected>,
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
    mut encounter_ui_query: Query<&mut Visibility, With<EncounterUI>>,
) {
    for ev in location_selected_events.read() {
        let (_, _, _, state) = query.get_mut(locations.0[&ev.0]).unwrap();

        if matches!(
            *state,
            LocationState::Current | LocationState::NotSelectable
        ) {
            return;
        }

        player_resources.food -= 1;
        player_resources.water -= 1;

        update_resources_events.send(UpdateResources {
            player_food: Some(player_resources.food),
            player_water: Some(player_resources.water),
            ..Default::default()
        });

        let (mut sprite, connected_locations, _, mut state) =
            query.get_mut(locations.0[&current_location.0]).unwrap();

        sprite.color = NOT_SELECTABLE_LOCATION_COLOUR;
        *state = LocationState::NotSelectable;

        for loc in connected_locations.0.clone() {
            let (mut sprite, _, _, mut state) = query.get_mut(locations.0[&loc.0]).unwrap();
            sprite.color = NOT_SELECTABLE_LOCATION_COLOUR;
            *state = LocationState::NotSelectable;
        }

        let (mut sprite, connected_locations, encounter, mut state) =
            query.get_mut(locations.0[&ev.0]).unwrap();

        sprite.color = CURRENT_LOCATION_COLOUR;
        *state = LocationState::Current;

        update_encounter_events.send(UpdateEncounter {
            text: encounter.text.clone(),
            interactions: encounter.interactions.clone(),
        });

        for loc in connected_locations.0.clone() {
            let (mut sprite, _, _, mut state) = query.get_mut(locations.0[&loc.0]).unwrap();
            sprite.color = SELECTABLE_LOCATION_COLOUR;
            *state = LocationState::Selectable;
        }

        *encounter_ui_query.single_mut() = Visibility::Visible;

        current_location.0 = ev.0.clone();
    }
}
