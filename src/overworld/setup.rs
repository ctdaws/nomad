use bevy::{
    asset::{AssetServer, Handle},
    ecs::system::{Commands, Res},
    math::Vec2,
    render::texture::Image,
    sprite::SpriteBundle,
    transform::components::Transform,
    utils::default,
};

use super::entities::{
    berry_bush::BerryBushBundle, change_location_zone::ChangeLocationZoneBundle,
    player::PlayerBundle, stick::StickBundle, tree::TreeBundle, water_pool::WaterPoolBundle,
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
    let water_pool_texture: Handle<Image> = asset_server.load("textures/water_pool.png");
    let change_location_zone_texture: Handle<Image> =
        asset_server.load("textures/change_location_zone.png");

    commands.spawn(WaterPoolBundle::new(
        Vec2::new(700., 400.),
        water_pool_texture,
    ));

    commands.spawn(TreeBundle::new(Vec2::new(0., -200.), tree_texture.clone()));
    commands.spawn(TreeBundle::new(
        Vec2::new(-100., -300.),
        tree_texture.clone(),
    ));
    commands.spawn(TreeBundle::new(
        Vec2::new(100., -260.),
        tree_texture.clone(),
    ));

    commands.spawn(BerryBushBundle::new(
        Vec2::new(-250., 400.),
        berry_bush_texture.clone(),
    ));
    commands.spawn(BerryBushBundle::new(
        Vec2::new(-275., 350.),
        berry_bush_texture.clone(),
    ));
    commands.spawn(BerryBushBundle::new(
        Vec2::new(-350., 300.),
        berry_bush_texture.clone(),
    ));
    commands.spawn(BerryBushBundle::new(
        Vec2::new(-400., 375.),
        berry_bush_texture.clone(),
    ));

    commands.spawn(StickBundle::new(
        Vec2::new(600., -450.),
        stick_texture.clone(),
    ));
    commands.spawn(StickBundle::new(
        Vec2::new(650., -475.),
        stick_texture.clone(),
    ));
    commands.spawn(StickBundle::new(
        Vec2::new(700., -400.),
        stick_texture.clone(),
    ));

    commands.spawn(StickBundle::new(
        Vec2::new(-450., -200.),
        stick_texture.clone(),
    ));
    commands.spawn(StickBundle::new(
        Vec2::new(-475., -250.),
        stick_texture.clone(),
    ));
    commands.spawn(StickBundle::new(
        Vec2::new(-500., -225.),
        stick_texture.clone(),
    ));

    commands.spawn(ChangeLocationZoneBundle::new(
        Vec2::new(0., 500.),
        change_location_zone_texture.clone(),
    ));
}
