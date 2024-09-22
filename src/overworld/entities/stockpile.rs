use bevy::{
    asset::Handle,
    ecs::{
        bundle::Bundle,
        component::Component,
        event::{Event, EventReader, EventWriter},
    },
    math::{Vec2, Vec3},
    render::texture::Image,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::default,
};

use crate::{
    overworld::{collisions::CircleCollider, setup::OVERWORLD_INTERACTABLE_ENTITIES_LAYER},
    party_resources::{UpdatePartyFoodEvent, UpdatePartyWaterEvent, UpdatePartyWoodEvent},
    settlement_resources::{
        UpdateSettlementFoodEvent, UpdateSettlementWaterEvent, UpdateSettlementWoodEvent,
    },
};

const STOCKPILE_INTERACTION_RADIUS: f32 = 10.;

#[derive(Event)]
pub struct StockpileDepositEvent;

#[derive(Component, Clone)]
pub struct Stockpile;

#[derive(Bundle, Clone)]
pub struct StockpileBundle {
    marker: Stockpile,
    interaction_collider: CircleCollider,
    sprite: SpriteBundle,
}

impl StockpileBundle {
    pub fn new(position: Vec2, texture: Handle<Image>) -> Self {
        StockpileBundle {
            marker: Stockpile,
            interaction_collider: CircleCollider {
                radius: STOCKPILE_INTERACTION_RADIUS,
            },
            sprite: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    position.x,
                    position.y,
                    OVERWORLD_INTERACTABLE_ENTITIES_LAYER,
                )),
                texture,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(90., 150.)),
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn deposit_resources(
    mut stockpile_deposit_events: EventReader<StockpileDepositEvent>,
    mut update_party_food_events: EventWriter<UpdatePartyFoodEvent>,
    mut update_party_water_events: EventWriter<UpdatePartyWaterEvent>,
    mut update_party_wood_events: EventWriter<UpdatePartyWoodEvent>,
    mut update_settlement_food_events: EventWriter<UpdateSettlementFoodEvent>,
    mut update_settlement_water_events: EventWriter<UpdateSettlementWaterEvent>,
    mut update_settlement_wood_events: EventWriter<UpdateSettlementWoodEvent>,
) {
    for _ in stockpile_deposit_events.read() {
        update_party_food_events.send(UpdatePartyFoodEvent(-1));
        update_party_water_events.send(UpdatePartyWaterEvent(-1));
        update_party_wood_events.send(UpdatePartyWoodEvent(-1));

        update_settlement_food_events.send(UpdateSettlementFoodEvent(1));
        update_settlement_water_events.send(UpdateSettlementWaterEvent(1));
        update_settlement_wood_events.send(UpdateSettlementWoodEvent(1));
    }
}
