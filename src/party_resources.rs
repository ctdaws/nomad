use bevy::ecs::{
    event::{Event, EventReader, EventWriter},
    system::{ResMut, Resource},
};

use crate::overworld::ui::party_resources::{
    UpdateFoodUIEvent, UpdateWaterUIEvent, UpdateWoodUIEvent,
};

pub const PARTY_MAX_WATER: i32 = 30;

#[derive(Resource, Default)]
pub struct PartyResources {
    pub food: i32,
    pub water: i32,
    pub wood: i32,
}

#[derive(Event)]
pub struct UpdateFoodEvent(pub i32);
#[derive(Event)]
pub struct UpdateWaterEvent(pub i32);
#[derive(Event)]
pub struct UpdateWoodEvent(pub i32);

pub fn update_food(
    mut party_resources: ResMut<PartyResources>,
    mut update_food_events: EventReader<UpdateFoodEvent>,
    mut update_food_ui_events: EventWriter<UpdateFoodUIEvent>,
) {
    for ev in update_food_events.read() {
        party_resources.food += ev.0;
        update_food_ui_events.send(UpdateFoodUIEvent(party_resources.food));
    }
}

pub fn update_water(
    mut party_resources: ResMut<PartyResources>,
    mut update_water_events: EventReader<UpdateWaterEvent>,
    mut update_water_ui_events: EventWriter<UpdateWaterUIEvent>,
) {
    for ev in update_water_events.read() {
        party_resources.water += ev.0;
        update_water_ui_events.send(UpdateWaterUIEvent(party_resources.water));
    }
}

pub fn update_wood(
    mut party_resources: ResMut<PartyResources>,
    mut update_wood_events: EventReader<UpdateWoodEvent>,
    mut update_wood_ui_events: EventWriter<UpdateWoodUIEvent>,
) {
    for ev in update_wood_events.read() {
        party_resources.wood += ev.0;
        update_wood_ui_events.send(UpdateWoodUIEvent(party_resources.wood));
    }
}
