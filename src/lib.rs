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
use locations::{setup_locations, Encounter, Location, SquareCollider};
use ui::{
    setup_ui, update_encounter_text, update_food_value, update_water_value, update_wood_value,
    UpdateEncounterText, UpdateFoodValue, UpdateWaterValue, UpdateWoodValue,
};

pub const WINDOW_START_WIDTH: f32 = 1920.;
pub const WINDOW_START_HEIGHT: f32 = 1080.;

pub mod locations;
pub mod ui;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorWorldCoords>()
            .insert_resource(PlayerResources {
                food: 50,
                water: 50,
                wood: 50,
            })
            .add_event::<UpdateEncounterText>()
            .add_event::<UpdateFoodValue>()
            .add_event::<UpdateWaterValue>()
            .add_event::<UpdateWoodValue>()
            .add_systems(Startup, (setup, setup_ui, setup_locations))
            .add_systems(
                Update,
                (
                    update_cursor_position,
                    process_mouse_click.after(update_cursor_position),
                    update_encounter_text,
                    update_food_value,
                    update_water_value,
                    update_wood_value,
                ),
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
    mut player_resources: ResMut<PlayerResources>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    query: Query<(&Transform, &Encounter, &SquareCollider), With<Location>>,
    mut encounter_text_events: EventWriter<UpdateEncounterText>,
    mut food_value_events: EventWriter<UpdateFoodValue>,
    mut water_value_events: EventWriter<UpdateWaterValue>,
    mut wood_value_events: EventWriter<UpdateWoodValue>,
) {
    if mouse_buttons.just_released(MouseButton::Left) {
        for (transform, encounter, collider) in &query {
            let pos = transform.translation;
            let cursor_pos = cursor_world_coords.0;

            if (cursor_pos.x <= pos.x + collider.half_width)
                && (cursor_pos.x >= pos.x - collider.half_width)
                && (cursor_pos.y <= pos.y + collider.half_height)
                && (cursor_pos.y >= pos.y - collider.half_height)
            {
                encounter_text_events.send(UpdateEncounterText(encounter.text.clone()));

                if let Some(food) = encounter.food {
                    let new_food = player_resources.food + food;

                    if new_food > 0 {
                        food_value_events.send(UpdateFoodValue(new_food));
                        player_resources.food = new_food;
                    } else {
                        encounter_text_events
                            .send(UpdateEncounterText("You fucked it".to_string()));

                        player_resources.food = 0;
                        food_value_events.send(UpdateFoodValue(0));
                    }
                }

                if let Some(water) = encounter.water {
                    let new_water = player_resources.water + water;

                    if new_water > 0 {
                        water_value_events.send(UpdateWaterValue(new_water));
                        player_resources.water = new_water;
                    } else {
                        encounter_text_events
                            .send(UpdateEncounterText("You fucked it".to_string()));

                        player_resources.water = 0;
                        water_value_events.send(UpdateWaterValue(0));
                    }
                }

                if let Some(wood) = encounter.wood {
                    let new_wood = player_resources.wood + wood;

                    if new_wood > 0 {
                        wood_value_events.send(UpdateWoodValue(new_wood));
                        player_resources.wood = new_wood;
                    } else {
                        encounter_text_events
                            .send(UpdateEncounterText("You fucked it".to_string()));

                        player_resources.wood = 0;
                        wood_value_events.send(UpdateWoodValue(0));
                    }
                }
            }
        }
    }
}
