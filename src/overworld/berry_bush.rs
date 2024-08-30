use bevy::{
    asset::{AssetServer, Handle},
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        event::{Event, EventReader},
        query::With,
        system::{Query, Res},
    },
    math::{Vec2, Vec3},
    render::texture::Image,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::default,
};

use super::{collisions::CircleCollider, setup::OVERWORLD_INTERACTABLE_ENTITIES_LAYER};

const BERRY_BUSH_INTERACTION_RADIUS: f32 = 50.;

#[derive(Component)]
pub struct BerryBush;

#[derive(Event)]
pub struct BerryBushPickedEvent(pub Entity);

#[derive(Bundle)]
pub struct BerryBushBundle {
    marker: BerryBush,
    interaction_collider: CircleCollider,
    sprite: SpriteBundle,
}

impl BerryBushBundle {
    pub fn new(position: Vec2, texture: Handle<Image>) -> Self {
        BerryBushBundle {
            marker: BerryBush,
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
    mut query: Query<&mut Handle<Image>, With<BerryBush>>,
) {
    for ev in berry_bush_picked_events.read() {
        let mut texture = query.get_mut(ev.0).unwrap();
        *texture = asset_server.load("textures/berry_bush_picked.png");
    }
}
