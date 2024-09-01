use bevy::{
    asset::Handle,
    ecs::{bundle::Bundle, component::Component},
    math::{Vec2, Vec3},
    render::texture::Image,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::default,
};

use crate::overworld::{
    collisions::RectangleCollider, setup::OVERWORLD_INTERACTABLE_ENTITIES_LAYER,
};

#[derive(Component)]
pub struct ChangeLocationZone;

#[derive(Bundle)]
pub struct ChangeLocationZoneBundle {
    marker: ChangeLocationZone,
    collider: RectangleCollider,
    sprite: SpriteBundle,
}

impl ChangeLocationZoneBundle {
    pub fn new(position: Vec2, texture: Handle<Image>) -> Self {
        ChangeLocationZoneBundle {
            marker: ChangeLocationZone,
            collider: RectangleCollider {
                half_width: 40.,
                half_height: 40.,
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
