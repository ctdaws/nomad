use bevy::{
    app::{App, FixedUpdate, Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};

use super::{
    collisions::collisions,
    entities::{
        berry_bush::{pick_berry_bush, BerryBushPickedEvent},
        player::{player_interaction, player_movement},
        stick::{pick_up_stick, StickPickedUpEvent},
        water_pool::{collect_water, WaterCollectedEvent},
    },
    party_resources::{
        update_food, update_water, update_wood, PartyResources, UpdateFoodEvent, UpdateWaterEvent,
        UpdateWoodEvent,
    },
    setup::setup_overworld,
    ui::plugin::OverworldUIPlugin,
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
                    player_interaction,
                    pick_up_stick,
                    pick_berry_bush,
                    collect_water,
                    update_food,
                    update_water,
                    update_wood,
                ),
            )
            .add_systems(FixedUpdate, (player_movement, collisions).chain());
    }
}
