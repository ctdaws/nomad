use bevy::{
    asset::{AssetServer, Assets},
    color::{Color, Srgba},
    ecs::system::{Commands, Res, ResMut},
    math::{primitives::Circle, Vec2},
    render::mesh::Mesh,
    sprite::{
        ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle, Sprite, SpriteBundle, Wireframe2dConfig,
    },
    transform::components::Transform,
    utils::default,
};

use super::player::PlayerBundle;

pub const OVERWORLD_BACKGROUND_LAYER: f32 = 0.;
pub const OVERWORLD_INTERACTABLE_ENTITIES_LAYER: f32 = 1.;
pub const OVERWORLD_PLAYER_LAYER: f32 = 2.;

pub fn setup_overworld(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("textures/overworld_background.png"),
        transform: Transform::from_xyz(0., 0., OVERWORLD_BACKGROUND_LAYER),
        ..default()
    });

    commands.spawn(PlayerBundle::new(
        10.,
        Vec2::new(70., 70.),
        asset_server.load("textures/player.png"),
    ));

    commands.spawn(SpriteBundle {
        texture: asset_server.load("textures/tree.png"),
        transform: Transform::from_xyz(10., 10., OVERWORLD_INTERACTABLE_ENTITIES_LAYER),
        sprite: Sprite {
            custom_size: Some(Vec2::new(90., 150.)),
            ..default()
        },
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("textures/berry_bush.png"),
        transform: Transform::from_xyz(-100., 100., OVERWORLD_INTERACTABLE_ENTITIES_LAYER),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100., 100.)),
            ..default()
        },
        ..default()
    });

    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
        material: materials.add(Color::Srgba(Srgba::GREEN)),
        transform: Transform::from_xyz(0.0, 0.0, 5.0),
        ..default()
    });
}
