use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::AssetServer,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        query::{With, Without},
        system::{Commands, Query, Res},
    },
    input::{keyboard::KeyCode, ButtonInput},
    math::{Vec2, Vec3},
    reflect::Reflect,
    render::{camera::ScalingMode, color::Color},
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};

use crate::{plugin::GameCamera, WINDOW_START_HEIGHT, WINDOW_START_WIDTH};

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, update);
    }
}

#[derive(Component)]
pub struct Player;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::Fixed {
        width: WINDOW_START_WIDTH,
        height: WINDOW_START_HEIGHT,
    };

    commands.spawn((camera_bundle, GameCamera));

    commands.spawn(SpriteBundle {
        texture: asset_server.load("textures/venture_background.png"),
        ..Default::default()
    });

    commands.spawn((
        Player,
        SpriteBundle {
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            sprite: Sprite {
                color: Color::BLUE,
                custom_size: Some(Vec2::new(70., 70.)),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

fn update(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<GameCamera>)>,
    mut camera_query: Query<&mut Transform, (With<GameCamera>, Without<Player>)>,
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

    let speed = 10.;

    let mut transform = player_query.single_mut();

    let mut next_pos =
        transform.translation + (Vec3::new(move_vector.x, move_vector.y, 0.) * speed);

    let top_wall_bound = 540.;
    let bottom_wall_bound = -540.;
    let left_wall_bound = -960.;
    let right_wall_bound = 960.;

    let player_width = 35.;
    let player_height = 35.;

    if next_pos.y > (top_wall_bound - player_height) {
        next_pos.y = top_wall_bound - player_height;
    }

    if next_pos.y < (bottom_wall_bound + player_height) {
        next_pos.y = bottom_wall_bound + player_height;
    }

    if next_pos.x > (right_wall_bound - player_width) {
        next_pos.x = right_wall_bound - player_width;
    }

    if next_pos.x < (left_wall_bound + player_width) {
        next_pos.x = left_wall_bound + player_width;
    }

    transform.translation = next_pos;

    camera_query.single_mut().translation = transform.translation;
}
