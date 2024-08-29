use bevy::{
    color::{Color, Srgba},
    ecs::{
        bundle::Bundle,
        component::Component,
        query::With,
        system::{Query, Res},
    },
    input::{keyboard::KeyCode, ButtonInput},
    math::{Vec2, Vec3},
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Size {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: Player,
    pub speed: Speed,
    pub size: Size,
    pub sprite: SpriteBundle,
}

impl PlayerBundle {
    pub fn new(speed: f32, size: Vec2) -> Self {
        PlayerBundle {
            marker: Player,
            speed: Speed(speed),
            size: Size {
                x: size.x,
                y: size.y,
            },
            sprite: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
                sprite: Sprite {
                    color: Color::Srgba(Srgba::BLUE),
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
