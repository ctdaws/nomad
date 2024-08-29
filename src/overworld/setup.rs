use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
    math::Vec2,
    sprite::SpriteBundle,
};

use super::player::PlayerBundle;

pub fn setup_overworld(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("textures/venture_background.png"),
        ..Default::default()
    });

    commands.spawn(PlayerBundle::new(10., Vec2::new(70., 70.)));
}
