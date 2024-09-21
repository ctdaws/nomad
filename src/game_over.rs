use bevy::{
    color::{Color, LinearRgba},
    ecs::{
        event::EventWriter,
        system::{Commands, Res, ResMut},
    },
    state::state::NextState,
    text::{TextSection, TextStyle},
    ui::node_bundles::TextBundle,
    utils::default,
};

use crate::{
    game_plugin::GameState,
    locations::location::{CurrentLocation, DespawnLocationEvent},
    party_resources::PartyResources,
};

pub fn game_over_setup(
    mut commands: Commands,
    current_location: Res<CurrentLocation>,
    mut despawn_location_events: EventWriter<DespawnLocationEvent>,
) {
    despawn_location_events.send(DespawnLocationEvent(current_location.0.clone()));

    commands.spawn(TextBundle::from_sections([TextSection::new(
        "Game Over",
        TextStyle {
            font_size: 30.,
            color: Color::LinearRgba(LinearRgba::WHITE),
            ..default()
        },
    )]));
}

pub fn check_for_game_over(
    party_resources: Res<PartyResources>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if party_resources.food == 0 || party_resources.water == 0 {
        next_game_state.set(GameState::GameOver)
    }
}
