use bevy::{
    app::{App, FixedUpdate, Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};

use super::{
    berry_bush::{pick_berry_bush, BerryBushPickedEvent},
    collisions::process_collisions,
    player::{process_player_interaction, update_player_movement},
    setup::setup_overworld,
    stick::{pick_up_stick, StickPickedUpEvent},
};

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StickPickedUpEvent>()
            .add_event::<BerryBushPickedEvent>()
            .add_systems(Startup, setup_overworld)
            .add_systems(
                Update,
                (process_player_interaction, pick_up_stick, pick_berry_bush),
            )
            .add_systems(
                FixedUpdate,
                (update_player_movement, process_collisions).chain(),
            );
    }
}
