use bevy::{
    asset::{AssetServer, Handle},
    ecs::system::{Commands, Res},
    math::Vec2,
    render::texture::Image,
    sprite::SpriteBundle,
    transform::components::Transform,
    utils::default,
};

use super::{
    berry_bush::BerryBushBundle, player::PlayerBundle, stick::StickBundle, tree::TreeBundle,
};

pub const OVERWORLD_BACKGROUND_LAYER: f32 = 0.;
pub const OVERWORLD_INTERACTABLE_ENTITIES_LAYER: f32 = 1.;
pub const OVERWORLD_PLAYER_LAYER: f32 = 2.;

pub fn setup_overworld(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("textures/overworld_background.png"),
        transform: Transform::from_xyz(0., 0., OVERWORLD_BACKGROUND_LAYER),
        ..default()
    });

    commands.spawn(PlayerBundle::new(asset_server.load("textures/player.png")));

    let tree_texture: Handle<Image> = asset_server.load("textures/tree.png");
    let berry_bush_texture: Handle<Image> = asset_server.load("textures/berry_bush.png");
    let stick_texture: Handle<Image> = asset_server.load("textures/stick.png");

    commands.spawn(TreeBundle::new(Vec2::new(50., 50.), tree_texture));

    commands.spawn(BerryBushBundle::new(
        Vec2::new(-100., 100.),
        berry_bush_texture,
    ));

    commands.spawn(StickBundle::new(Vec2::new(200., -300.), stick_texture));
}
