use std::f32::consts::PI;

use bevy::{
    asset::Handle,
    ecs::{bundle::Bundle, component::Component},
    math::{Quat, Vec2, Vec3},
    render::texture::Image,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::default,
};

use crate::{
    locations::location::LocationId,
    overworld::{collisions::RectangleCollider, setup::OVERWORLD_INTERACTABLE_ENTITIES_LAYER},
};

#[derive(Component, Clone)]
pub struct ChangeLocationZone;

#[derive(Component, Clone)]
pub struct ConnectedLocationId(pub LocationId);

#[derive(Bundle, Clone)]
pub struct ChangeLocationZoneBundle {
    marker: ChangeLocationZone,
    collider: RectangleCollider,
    sprite: SpriteBundle,
    connected_location: ConnectedLocationId,
}

impl ChangeLocationZoneBundle {
    pub fn new(
        position: Vec2,
        rotation_degrees: f32,
        texture: Handle<Image>,
        connected_location_id: LocationId,
    ) -> Self {
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
                ))
                .with_rotation(Quat::from_rotation_z(rotation_degrees * (PI / 180.))),
                texture,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(80., 80.)),
                    ..default()
                },
                ..default()
            },
            connected_location: ConnectedLocationId(connected_location_id),
        }
    }
}
