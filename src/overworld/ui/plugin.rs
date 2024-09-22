use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};

use crate::overworld::plugin::OverworldSet;

use super::{
    party_resources::{
        setup_party_resources_ui, update_party_food, update_party_water, update_party_wood,
        UpdatePartyFoodUIEvent, UpdatePartyWaterUIEvent, UpdatePartyWoodUIEvent,
    },
    settlement_resources::{
        setup_settlement_resources_ui, update_settlement_food,
        update_settlement_resources_ui_visibility, update_settlement_water, update_settlement_wood,
        UpdateSettlementFoodUIEvent, UpdateSettlementWaterUIEvent, UpdateSettlementWoodUIEvent,
    },
};

pub struct OverworldUIPlugin;

impl Plugin for OverworldUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdatePartyFoodUIEvent>()
            .add_event::<UpdatePartyWaterUIEvent>()
            .add_event::<UpdatePartyWoodUIEvent>()
            .add_event::<UpdateSettlementFoodUIEvent>()
            .add_event::<UpdateSettlementWaterUIEvent>()
            .add_event::<UpdateSettlementWoodUIEvent>()
            .add_systems(
                Startup,
                (setup_party_resources_ui, setup_settlement_resources_ui).in_set(OverworldSet),
            )
            .add_systems(
                Update,
                (
                    update_party_food,
                    update_party_water,
                    update_party_wood,
                    update_settlement_food,
                    update_settlement_water,
                    update_settlement_wood,
                    update_settlement_resources_ui_visibility,
                )
                    .in_set(OverworldSet),
            );
    }
}
