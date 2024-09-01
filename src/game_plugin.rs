use bevy::{
    app::{App, Plugin, Startup},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        schedule::SystemSet,
        system::{Commands, Resource},
    },
    render::camera::ScalingMode,
};

use crate::{overworld::plugin::OverworldPlugin, WINDOW_START_HEIGHT, WINDOW_START_WIDTH};

pub const GAME_START_TEXT: &str = "Narrowly, you and your people escaped danger through the mountain pass. In the panic you didn't have time to bring anything with you. While you've found temporary sanctuary here, the tribe will die without support. You must explore and forage for supplies if the tribe is to survive";
pub const GAME_START_INTERACTION_TEXT: &str = "Leave camp";

#[derive(Component)]
pub struct GameCamera;

// #[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
// enum GameState {
//     MapScreen,
//     Game,
// }

#[derive(Resource)]
pub struct PlayerResources {
    pub food: i32,
    pub water: i32,
    pub wood: i32,
}

#[derive(Resource)]
pub struct SettlementResources {
    pub food: i32,
    pub water: i32,
    pub wood: i32,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapSet;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OverworldPlugin)
            .add_systems(Startup, setup_game_camera);
    }
}

fn setup_game_camera(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::Fixed {
        width: WINDOW_START_WIDTH,
        height: WINDOW_START_HEIGHT,
    };

    commands.spawn((camera_bundle, GameCamera));
}
