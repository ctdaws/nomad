use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
    math::Vec2,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};

use super::player::PlayerBundle;

pub fn setup_overworld(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("textures/overworld_background.png"),
        ..Default::default()
    });

    commands.spawn(PlayerBundle::new(
        10.,
        Vec2::new(70., 70.),
        asset_server.load("textures/player.png"),
    ));

    commands.spawn(SpriteBundle {
        texture: asset_server.load("textures/tree.png"),
        transform: Transform::from_xyz(10., 10., 1.),
        sprite: Sprite {
            custom_size: Some(Vec2::new(90., 150.)),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("textures/berry_bush.png"),
        transform: Transform::from_xyz(-100., 100., 1.),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..Default::default()
        },
        ..Default::default()
    });
}
