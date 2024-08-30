use bevy::{
    app::{App, FixedUpdate, Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};

use super::{
    berry_bush::{pick_berry_bush, BerryBushPickedEvent},
    collisions::process_collisions,
    party_resources::{
        update_food, update_water, update_wood, PartyResources, UpdateFoodEvent, UpdateWaterEvent,
        UpdateWoodEvent,
    },
    player::{process_player_interaction, update_player_movement},
    setup::setup_overworld,
    stick::{pick_up_stick, StickPickedUpEvent},
    ui::plugin::OverworldUIPlugin,
    water_pool::{collect_water, WaterCollectedEvent},
};

pub struct OverworldPlugin;

impl Plugin for OverworldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OverworldUIPlugin)
            .add_event::<UpdateFoodEvent>()
            .add_event::<UpdateWaterEvent>()
            .add_event::<UpdateWoodEvent>()
            .add_event::<StickPickedUpEvent>()
            .add_event::<BerryBushPickedEvent>()
            .add_event::<WaterCollectedEvent>()
            .init_resource::<PartyResources>()
            .add_systems(Startup, setup_overworld)
            .add_systems(
                Update,
                (
                    process_player_interaction,
                    pick_up_stick,
                    pick_berry_bush,
                    collect_water,
                    update_food,
                    update_water,
                    update_wood,
                ),
            )
            .add_systems(
                FixedUpdate,
                (update_player_movement, process_collisions).chain(),
            );
    }
}
