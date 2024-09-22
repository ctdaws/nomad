use bevy::ecs::{
    event::{Event, EventReader, EventWriter},
    system::{ResMut, Resource},
};

use crate::overworld::ui::party_resources::{
    UpdatePartyFoodUIEvent, UpdatePartyWaterUIEvent, UpdatePartyWoodUIEvent,
};

pub const PARTY_MAX_WATER: i32 = 30;

#[derive(Resource, Default)]
pub struct PartyResources {
    pub food: i32,
    pub water: i32,
    pub wood: i32,
}

#[derive(Event)]
pub struct UpdatePartyFoodEvent(pub i32);
#[derive(Event)]
pub struct UpdatePartyWaterEvent(pub i32);
#[derive(Event)]
pub struct UpdatePartyWoodEvent(pub i32);

pub fn update_party_food(
    mut party_resources: ResMut<PartyResources>,
    mut update_party_food_events: EventReader<UpdatePartyFoodEvent>,
    mut update_party_food_ui_events: EventWriter<UpdatePartyFoodUIEvent>,
) {
    for ev in update_party_food_events.read() {
        party_resources.food += ev.0;
        update_party_food_ui_events.send(UpdatePartyFoodUIEvent(party_resources.food));
    }
}

pub fn update_party_water(
    mut party_resources: ResMut<PartyResources>,
    mut update_party_water_events: EventReader<UpdatePartyWaterEvent>,
    mut update_party_water_ui_events: EventWriter<UpdatePartyWaterUIEvent>,
) {
    for ev in update_party_water_events.read() {
        party_resources.water += ev.0;
        update_party_water_ui_events.send(UpdatePartyWaterUIEvent(party_resources.water));
    }
}

pub fn update_party_wood(
    mut party_resources: ResMut<PartyResources>,
    mut update_party_wood_events: EventReader<UpdatePartyWoodEvent>,
    mut update_party_wood_ui_events: EventWriter<UpdatePartyWoodUIEvent>,
) {
    for ev in update_party_wood_events.read() {
        party_resources.wood += ev.0;
        update_party_wood_ui_events.send(UpdatePartyWoodUIEvent(party_resources.wood));
    }
}
