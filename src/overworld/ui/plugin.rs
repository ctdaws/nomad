use bevy::app::{App, Plugin, Startup, Update};

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
            .add_systems(Startup, setup_party_resources)
            .add_systems(Update, (update_food, update_water, update_wood));
    }
}
