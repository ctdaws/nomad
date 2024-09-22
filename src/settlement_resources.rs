use bevy::ecs::{
    event::{Event, EventReader, EventWriter},
    system::{ResMut, Resource},
};

use crate::overworld::ui::settlement_resources::{
    UpdateSettlementFoodUIEvent, UpdateSettlementWaterUIEvent, UpdateSettlementWoodUIEvent,
};

pub const PARTY_MAX_WATER: i32 = 30;

#[derive(Resource, Default)]
pub struct SettlementResources {
    pub food: i32,
    pub water: i32,
    pub wood: i32,
}

#[derive(Event)]
pub struct UpdateSettlementFoodEvent(pub i32);
#[derive(Event)]
pub struct UpdateSettlementWaterEvent(pub i32);
#[derive(Event)]
pub struct UpdateSettlementWoodEvent(pub i32);

pub fn update_settlement_food(
    mut party_resources: ResMut<SettlementResources>,
    mut update_settlement_food_events: EventReader<UpdateSettlementFoodEvent>,
    mut update_settlement_food_ui_events: EventWriter<UpdateSettlementFoodUIEvent>,
) {
    for ev in update_settlement_food_events.read() {
        party_resources.food += ev.0;
        update_settlement_food_ui_events.send(UpdateSettlementFoodUIEvent(party_resources.food));
    }
}

pub fn update_settlement_water(
    mut party_resources: ResMut<SettlementResources>,
    mut update_settlement_water_events: EventReader<UpdateSettlementWaterEvent>,
    mut update_settlement_water_ui_events: EventWriter<UpdateSettlementWaterUIEvent>,
) {
    for ev in update_settlement_water_events.read() {
        party_resources.water += ev.0;
        update_settlement_water_ui_events.send(UpdateSettlementWaterUIEvent(party_resources.water));
    }
}

pub fn update_settlement_wood(
    mut party_resources: ResMut<SettlementResources>,
    mut update_settlement_wood_events: EventReader<UpdateSettlementWoodEvent>,
    mut update_settlement_wood_ui_events: EventWriter<UpdateSettlementWoodUIEvent>,
) {
    for ev in update_settlement_wood_events.read() {
        party_resources.wood += ev.0;
        update_settlement_wood_ui_events.send(UpdateSettlementWoodUIEvent(party_resources.wood));
    }
}
