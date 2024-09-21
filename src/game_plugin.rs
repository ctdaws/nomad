use bevy::{
    app::{App, Plugin, Startup, Update},
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{component::Component, schedule::IntoSystemConfigs, system::Commands},
    render::camera::ScalingMode,
    state::{
        app::AppExtStates,
        state::{OnEnter, States},
    },
};

use crate::{
    game_over::{check_for_game_over, game_over_setup},
    locations::location::{
        despawn_location, ChangeLocationEvent, CurrentLocation, DespawnLocationEvent,
        LocationEntities, LocationId, Locations, SpawnLocationEvent,
    },
    overworld::{
        entities::player::despawn_player,
        plugin::{OverworldPlugin, OverworldSet},
    },
    party_resources::{
        update_food, update_water, update_wood, PartyResources, UpdateFoodEvent, UpdateWaterEvent,
        UpdateWoodEvent,
    },
    WINDOW_START_HEIGHT, WINDOW_START_WIDTH,
};

#[derive(States, Debug, Hash, Eq, PartialEq, Clone, Default)]
pub enum GameState {
    #[default]
    Overworld,
    GameOver,
}

#[derive(Component)]
pub struct GameCamera;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OverworldPlugin)
            .init_state::<GameState>()
            .init_resource::<Locations>()
            .init_resource::<LocationEntities>()
            .insert_resource(PartyResources {
                food: 1,
                water: 1,
                wood: 0,
            })
            .insert_resource(CurrentLocation(LocationId(0)))
            .add_event::<UpdateFoodEvent>()
            .add_event::<UpdateWaterEvent>()
            .add_event::<UpdateWoodEvent>()
            .add_event::<ChangeLocationEvent>()
            .add_event::<SpawnLocationEvent>()
            .add_event::<DespawnLocationEvent>()
            .add_systems(Startup, setup_game_camera)
            .add_systems(Update, (update_food, update_water, update_wood))
            .add_systems(Update, check_for_game_over.in_set(OverworldSet))
            .add_systems(
                OnEnter(GameState::GameOver),
                (game_over_setup, (despawn_location, despawn_player)).chain(),
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
