use bevy::{
    app::{App, Plugin, Startup},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{component::Component, system::Commands},
    render::camera::ScalingMode,
};

use crate::{
    locations::setup::{setup_locations, CurrentLocation, LocationId, Locations},
    overworld::plugin::OverworldPlugin,
    WINDOW_START_HEIGHT, WINDOW_START_WIDTH,
};

#[derive(Component)]
pub struct GameCamera;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OverworldPlugin)
            .init_resource::<Locations>()
            .insert_resource(CurrentLocation(LocationId(0)))
            .add_systems(Startup, (setup_game_camera, setup_locations));
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
