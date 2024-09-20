use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
};

use super::entities::player::PlayerBundle;

pub const OVERWORLD_BACKGROUND_LAYER: f32 = 0.;
pub const OVERWORLD_INTERACTABLE_ENTITIES_LAYER: f32 = 1.;
pub const OVERWORLD_PLAYER_LAYER: f32 = 2.;

pub fn setup_overworld(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle::new(asset_server.load("textures/player.png")));
}
