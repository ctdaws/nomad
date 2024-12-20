use bevy::{
    asset::Handle,
    ecs::{
        bundle::Bundle,
        component::Component,
        event::{Event, EventReader, EventWriter},
        system::Res,
    },
    math::{Vec2, Vec3},
    render::texture::Image,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::default,
};

use crate::{
    overworld::{collisions::CircleCollider, setup::OVERWORLD_INTERACTABLE_ENTITIES_LAYER},
    party_resources::{PartyResources, UpdatePartyWaterEvent, PARTY_MAX_WATER},
};

const WATER_POOL_INTERACTION_RADIUS: f32 = 40.;

#[derive(Event)]
pub struct WaterCollectedEvent();

#[derive(Component, Clone)]
pub struct WaterPool;

#[derive(Bundle, Clone)]
pub struct WaterPoolBundle {
    marker: WaterPool,
    interaction_collider: CircleCollider,
    sprite: SpriteBundle,
}

impl WaterPoolBundle {
    pub fn new(position: Vec2, texture: Handle<Image>) -> Self {
        WaterPoolBundle {
            marker: WaterPool,
            interaction_collider: CircleCollider {
                radius: WATER_POOL_INTERACTION_RADIUS,
            },
            sprite: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    position.x,
                    position.y,
                    OVERWORLD_INTERACTABLE_ENTITIES_LAYER,
                )),
                texture,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(80., 80.)),
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn collect_water(
    party_resources: Res<PartyResources>,
    mut water_collected_events: EventReader<WaterCollectedEvent>,
    mut update_water_events: EventWriter<UpdatePartyWaterEvent>,
) {
    for _ in water_collected_events.read() {
        update_water_events.send(UpdatePartyWaterEvent(PARTY_MAX_WATER - party_resources.water));
    }
}
