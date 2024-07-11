use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::AssetServer,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        system::{Commands, Res, Resource},
    },
    prelude::IntoSystemConfigs,
    render::camera::ScalingMode,
    sprite::SpriteBundle,
};

use crate::{
    input::{process_mouse_click, update_cursor_position, CursorWorldCoords},
    locations::plugin::LocationsPlugin,
    ui::plugin::UIPlugin,
    WINDOW_START_HEIGHT, WINDOW_START_WIDTH,
};

#[derive(Component)]
pub struct GameCamera;

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

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UIPlugin, LocationsPlugin))
            .init_resource::<CursorWorldCoords>()
            .insert_resource(PlayerResources {
                food: 10,
                water: 10,
                wood: 10,
            })
            .insert_resource(SettlementResources {
                food: 0,
                water: 0,
                wood: 0,
            })
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (update_cursor_position, process_mouse_click).chain(),
            );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::Fixed {
        width: WINDOW_START_WIDTH,
        height: WINDOW_START_HEIGHT,
    };

    commands.spawn((camera_bundle, GameCamera));

    commands.spawn(SpriteBundle {
        texture: asset_server.load("textures/background.png"),
        ..Default::default()
    });
}
