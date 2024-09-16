use bevy::{
    app::{App, FixedUpdate, Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::{
    change_location::{change_location, ChangeLocationEvent},
    locations::setup::setup_locations,
    party_resources::{PartyResources, UpdateFoodEvent, UpdateWaterEvent, UpdateWoodEvent},
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
    ui::{
        party_resources::{update_food, update_water, update_wood},
        plugin::OverworldUIPlugin,
    },
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
            .add_event::<ChangeLocationEvent>()
            .init_resource::<PartyResources>()
            .add_systems(Startup, setup_overworld.after(setup_locations))
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
                    change_location,
                ),
            )
            .add_systems(FixedUpdate, (player_movement, collisions).chain());
    }
}
