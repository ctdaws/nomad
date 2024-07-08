use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::AssetServer,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        component::Component,
        event::EventWriter,
        query::With,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    input::{mouse::MouseButton, ButtonInput},
    math::Vec2,
    prelude::IntoSystemConfigs,
    render::camera::{Camera, ScalingMode},
    sprite::SpriteBundle,
    transform::components::{GlobalTransform, Transform},
    window::{PrimaryWindow, Window},
};
use locations::{
    location::{Location, LocationId, LocationSelected, SquareCollider},
    plugin::LocationsPlugin,
};
use ui::plugin::UIPlugin;

pub const WINDOW_START_WIDTH: f32 = 1920.;
pub const WINDOW_START_HEIGHT: f32 = 1080.;

pub mod locations {
    pub mod location;
    pub mod location_config;
    pub mod locations;
    pub mod plugin;
}
pub mod ui {
    pub mod encounter;
    pub mod home;
    pub mod plugin;
    pub mod resources;
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UIPlugin, LocationsPlugin))
            .init_resource::<CursorWorldCoords>()
            .insert_resource(PlayerResources {
                food: 5,
                water: 5,
                wood: 5,
            })
            .insert_resource(SettlementResources {
                food: 0,
                water: 0,
                wood: 0,
            })
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                ((update_cursor_position, process_mouse_click).chain(),),
            );
    }
}

#[derive(Component)]
struct GameCamera;

#[derive(Resource, Default)]
struct CursorWorldCoords(Vec2);

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

fn update_cursor_position(
    mut cursor_world_coords: ResMut<CursorWorldCoords>,
    query_window: Query<&Window, With<PrimaryWindow>>,
    query_camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
) {
    let (camera, camera_transform) = query_camera.single();
    let window = query_window.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        cursor_world_coords.0 = world_position;
    }
}

fn process_mouse_click(
    cursor_world_coords: Res<CursorWorldCoords>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    query: Query<(&Transform, &SquareCollider, &LocationId), With<Location>>,
    mut location_selected_events: EventWriter<LocationSelected>,
) {
    if mouse_buttons.just_released(MouseButton::Left) {
        for (transform, collider, location_id) in &query {
            let pos = transform.translation;
            let cursor_pos = cursor_world_coords.0;

            if (cursor_pos.x <= pos.x + collider.half_width)
                && (cursor_pos.x >= pos.x - collider.half_width)
                && (cursor_pos.y <= pos.y + collider.half_height)
                && (cursor_pos.y >= pos.y - collider.half_height)
            {
                location_selected_events.send(LocationSelected(location_id.clone()));
            }
        }
    }
}
