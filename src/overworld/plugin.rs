use bevy::{
    app::{App, FixedUpdate, Plugin, Update},
    ecs::schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet},
    state::{condition::in_state, state::OnEnter},
};

use crate::{
    game_plugin::GameState,
    locations::{
        location::{change_location, despawn_location, spawn_location},
        setup::{setup_locations, spawn_initial_location},
    },
};

use super::{
    collisions::collisions,
    entities::{
        berry_bush::{pick_berry_bush, BerryBushPickedEvent},
        player::{player_interaction, player_movement},
        stick::{pick_up_stick, StickPickedUpEvent},
        water_pool::{collect_water, WaterCollectedEvent},
    },
    setup::setup_overworld,
    ui::plugin::OverworldUIPlugin,
};

#[derive(SystemSet, Hash, Debug, PartialEq, Eq, Clone)]
pub struct OverworldSet;

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OverworldUIPlugin)
            .add_event::<StickPickedUpEvent>()
            .add_event::<BerryBushPickedEvent>()
            .add_event::<WaterCollectedEvent>()
            .add_systems(
                OnEnter(GameState::Overworld),
                ((setup_locations, spawn_initial_location, setup_overworld).chain())
                    .in_set(OverworldSet),
            )
            .add_systems(
                Update,
                (
                    (change_location, despawn_location, spawn_location).chain(),
                    player_interaction,
                    pick_up_stick,
                    pick_berry_bush,
                    collect_water,
                )
                    .in_set(OverworldSet),
            )
            .add_systems(
                FixedUpdate,
                ((player_movement, collisions).chain()).in_set(OverworldSet),
            )
            .configure_sets(Update, OverworldSet.run_if(in_state(GameState::Overworld)))
            .configure_sets(
                FixedUpdate,
                OverworldSet.run_if(in_state(GameState::Overworld)),
            );
    }
}
