use bevy::{
    app::{App, Plugin, Startup, Update},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{component::Component, schedule::IntoSystemConfigs, system::Commands},
    render::camera::ScalingMode,
};

use crate::{
    locations::{
        location::{
            change_location, spawn_location, ChangeLocationEvent, CurrentLocation,
            LocationEntities, LocationId, Locations, SpawnLocationEvent,
        },
        setup::{setup_locations, spawn_initial_location},
    },
    overworld::plugin::OverworldPlugin,
    party_resources::{
        update_food, update_water, update_wood, PartyResources, UpdateFoodEvent, UpdateWaterEvent,
        UpdateWoodEvent,
    },
    WINDOW_START_HEIGHT, WINDOW_START_WIDTH,
};

#[derive(Component)]
pub struct GameCamera;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OverworldPlugin)
            .init_resource::<Locations>()
            .init_resource::<LocationEntities>()
            .init_resource::<PartyResources>()
            .insert_resource(CurrentLocation(LocationId(0)))
            .add_event::<UpdateFoodEvent>()
            .add_event::<UpdateWaterEvent>()
            .add_event::<UpdateWoodEvent>()
            .add_event::<ChangeLocationEvent>()
            .add_event::<SpawnLocationEvent>()
            .add_systems(
                Startup,
                (
                    setup_game_camera,
                    (setup_locations, spawn_initial_location).chain(),
                ),
            )
            .add_systems(
                Update,
                (
                    (change_location, spawn_location).chain(),
                    update_food,
                    update_water,
                    update_wood,
                ),
            );
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
