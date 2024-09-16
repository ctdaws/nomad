use bevy::{
    app::{App, FixedUpdate, Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::locations::setup::setup_locations;

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

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OverworldUIPlugin)
            .add_event::<StickPickedUpEvent>()
            .add_event::<BerryBushPickedEvent>()
            .add_event::<WaterCollectedEvent>()
            .add_systems(Startup, setup_overworld.after(setup_locations))
            .add_systems(
                Update,
                (
                    player_interaction,
                    pick_up_stick,
                    pick_berry_bush,
                    collect_water,
                ),
            )
            .add_systems(FixedUpdate, (player_movement, collisions).chain());
    }
}
