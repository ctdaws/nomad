use bevy::{
    asset::{AssetServer, Handle},
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        query::With,
        system::{Query, Res},
    },
    math::{Vec2, Vec3},
    render::texture::Image,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::default,
};

use crate::{
    overworld::{collisions::CircleCollider, setup::OVERWORLD_INTERACTABLE_ENTITIES_LAYER},
    party_resources::UpdatePartyFoodEvent,
};

const BERRY_BUSH_INTERACTION_RADIUS: f32 = 50.;

#[derive(Event)]
pub struct BerryBushPickedEvent(pub Entity);

#[derive(Component, Clone)]
pub struct BerryBush;

#[derive(Component, Clone)]
pub enum BerryBushState {
    Unpicked,
    Picked,
}

#[derive(Bundle, Clone)]
pub struct BerryBushBundle {
    marker: BerryBush,
    pub state: BerryBushState,
    interaction_collider: CircleCollider,
    pub sprite: SpriteBundle,
}

impl BerryBushBundle {
    pub fn new(position: Vec2, texture: Handle<Image>) -> Self {
        BerryBushBundle {
            marker: BerryBush,
            state: BerryBushState::Unpicked,
            interaction_collider: CircleCollider {
                radius: BERRY_BUSH_INTERACTION_RADIUS,
            },
            sprite: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    position.x,
                    position.y,
                    OVERWORLD_INTERACTABLE_ENTITIES_LAYER,
                )),
                texture,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(100., 100.)),
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn pick_berry_bush(
    asset_server: Res<AssetServer>,
    mut berry_bush_picked_events: EventReader<BerryBushPickedEvent>,
    mut query: Query<(&mut BerryBushState, &mut Handle<Image>), With<BerryBush>>,
    mut update_food_events: EventWriter<UpdatePartyFoodEvent>,
) {
    for ev in berry_bush_picked_events.read() {
        let (mut state, mut texture) = query.get_mut(ev.0).unwrap();
        *state = BerryBushState::Picked;
        update_food_events.send(UpdatePartyFoodEvent(10));
        *texture = asset_server.load("textures/berry_bush_picked.png");
    }
}
