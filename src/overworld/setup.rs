use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        system::{Commands, Res},
    },
    hierarchy::BuildChildren,
    render::view::VisibilityBundle,
    transform::bundles::TransformBundle,
};

use crate::locations::{
    location::load_location,
    setup::{CurrentLocation, Locations},
};

use super::entities::player::PlayerBundle;

pub const OVERWORLD_BACKGROUND_LAYER: f32 = 0.;
pub const OVERWORLD_INTERACTABLE_ENTITIES_LAYER: f32 = 1.;
pub const OVERWORLD_PLAYER_LAYER: f32 = 2.;

#[derive(Component)]
pub struct OverworldScene;

pub fn setup_overworld(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_location: Res<CurrentLocation>,
    locations: Res<Locations>,
) {
    let mut scene_id = commands
        .spawn((
            OverworldScene,
            TransformBundle::default(),
            VisibilityBundle::default(),
        ))
        .id();

    load_location(
        &mut commands,
        &asset_server,
        &current_location.0,
        &locations,
        &mut scene_id,
    );

    let player = commands
        .spawn(PlayerBundle::new(asset_server.load("textures/player.png")))
        .id();

    commands.entity(scene_id).push_children(&[player]);
}
