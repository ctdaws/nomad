use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::overworld::plugin::OverworldSet;

use super::party_resources::{
    setup_party_resources, update_food, update_water, update_wood, UpdateFoodUIEvent,
    UpdateWaterUIEvent, UpdateWoodUIEvent,
};

pub struct OverworldUIPlugin;

impl Plugin for OverworldUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateFoodUIEvent>()
            .add_event::<UpdateWaterUIEvent>()
            .add_event::<UpdateWoodUIEvent>()
            .add_systems(Startup, (setup_party_resources).in_set(OverworldSet))
            .add_systems(
                Update,
                (update_food, update_water, update_wood).in_set(OverworldSet),
            );
    }
}
