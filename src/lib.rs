use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::AssetServer,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        bundle::Bundle,
        component::Component,
        event::{Event, EventReader, EventWriter},
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    input::{mouse::MouseButton, ButtonInput},
    math::{Vec2, Vec3},
    render::{
        camera::{Camera, ScalingMode},
        color::Color,
    },
    sprite::SpriteBundle,
    text::{Text, TextStyle},
    transform::components::{GlobalTransform, Transform},
    ui::node_bundles::TextBundle,
    window::{PrimaryWindow, Window},
};

pub const WINDOW_START_WIDTH: f32 = 1920.;
pub const WINDOW_START_HEIGHT: f32 = 1080.;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorWorldCoords>()
            .add_event::<LocationClickedEvent>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    update_cursor_position,
                    detect_mouse_collisions.after(update_cursor_position),
                    update_text,
                ),
            );
    }
}

#[derive(Component)]
struct GameCamera;

#[derive(Component)]
struct Location;

#[derive(Resource, Default)]
struct CursorWorldCoords(Vec2);

#[derive(Component)]
struct SquareCollier {
    half_width: f32,
    half_height: f32,
}

#[derive(Component)]
struct LocationText(String);

#[derive(Bundle)]
struct LocationBundle {
    marker: Location,
    text: LocationText,
    collider: SquareCollier,
    sprite: SpriteBundle,
}

impl LocationBundle {
    fn new(position: Vec3, text: String, asset_server: &Res<AssetServer>) -> Self {
        LocationBundle {
            marker: Location,
            text: LocationText(text),
            collider: SquareCollier {
                half_width: 25.,
                half_height: 25.,
            },
            sprite: SpriteBundle {
                transform: Transform::from_translation(position)
                    .with_scale(Vec3::new(0.5, 0.5, 1.)),
                texture: asset_server.load("textures/location.png"),
                ..Default::default()
            },
        }
    }
}

#[derive(Component)]
struct GameText;

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

    commands.spawn(LocationBundle::new(
        Vec3::new(-550., -150., 1.),
        "location 1".to_string(),
        &asset_server,
    ));
    commands.spawn(LocationBundle::new(
        Vec3::new(-300., 50., 1.),
        "location 2".to_string(),
        &asset_server,
    ));
    commands.spawn(LocationBundle::new(
        Vec3::new(0., 200., 1.),
        "location 3".to_string(),
        &asset_server,
    ));

    commands.spawn((
        TextBundle::from_section(
            "TEST",
            TextStyle {
                font_size: 16.,
                color: Color::WHITE,
                ..Default::default()
            },
        ),
        GameText,
    ));
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

#[derive(Event)]
struct LocationClickedEvent(String);

fn detect_mouse_collisions(
    cursor_world_coords: Res<CursorWorldCoords>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    query: Query<(&Transform, &LocationText, &SquareCollier), With<Location>>,
    mut location_clicked_event_writer: EventWriter<LocationClickedEvent>,
) {
    for (transform, text, collider) in &query {
        let pos = transform.translation;
        let cursor_pos = cursor_world_coords.0;

        if (cursor_pos.x <= pos.x + collider.half_width)
            && (cursor_pos.x >= pos.x - collider.half_width)
            && (cursor_pos.y <= pos.y + collider.half_height)
            && (cursor_pos.y >= pos.y - collider.half_height)
        {
            if mouse_buttons.just_released(MouseButton::Left) {
                location_clicked_event_writer.send(LocationClickedEvent(text.0.clone()));
            }
        }
    }
}

fn update_text(
    mut ev_reader: EventReader<LocationClickedEvent>,
    mut query: Query<&mut Text, With<GameText>>,
) {
    for ev in ev_reader.read() {
        let mut text = query.single_mut();
        text.sections[0].value = ev.0.clone();
    }
}
