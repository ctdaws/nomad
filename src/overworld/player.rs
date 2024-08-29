use bevy::{
    asset::Handle,
    ecs::{
        bundle::Bundle,
        component::Component,
        query::With,
        system::{Query, Res},
    },
    input::{keyboard::KeyCode, ButtonInput},
    math::{Vec2, Vec3},
    render::texture::Image,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};

use super::{
    collisions::{CircleCollider, SquareCollider},
    setup::OVERWORLD_PLAYER_LAYER,
};

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    speed: Speed,
    collider: SquareCollider,
    interaction_collider: CircleCollider,
    sprite: SpriteBundle,
}

impl PlayerBundle {
    pub fn new(speed: f32, size: Vec2, interaction_radius: f32, texture: Handle<Image>) -> Self {
        PlayerBundle {
            marker: Player,
            speed: Speed(speed),
            collider: SquareCollider {
                half_width: size.x / 2.,
                half_height: size.y / 2.,
            },
            interaction_collider: CircleCollider {
                radius: interaction_radius,
            },
            sprite: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., OVERWORLD_PLAYER_LAYER)),
                texture,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(size.x, size.y)),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

pub fn update_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Speed), With<Player>>,
) {
    let mut move_vector = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        move_vector += Vec2::new(0., 1.);
    }

    if keys.pressed(KeyCode::KeyS) {
        move_vector += Vec2::new(0., -1.);
    }

    if keys.pressed(KeyCode::KeyA) {
        move_vector += Vec2::new(-1., 0.);
    }

    if keys.pressed(KeyCode::KeyD) {
        move_vector += Vec2::new(1., 0.);
    }

    let (mut transform, speed) = query.single_mut();

    transform.translation =
        transform.translation + (Vec3::new(move_vector.x, move_vector.y, 0.) * speed.0);
}
