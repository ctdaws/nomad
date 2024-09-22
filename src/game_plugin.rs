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
        update_party_food, update_party_water, update_party_wood, PartyResources,
        UpdatePartyFoodEvent, UpdatePartyWaterEvent, UpdatePartyWoodEvent,
    },
    settlement_resources::{
        update_settlement_food, update_settlement_water, update_settlement_wood,
        SettlementResources, UpdateSettlementFoodEvent, UpdateSettlementWaterEvent,
        UpdateSettlementWoodEvent,
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
                food: 10,
                water: 10,
                wood: 0,
            })
            .insert_resource(SettlementResources {
                food: 0,
                water: 0,
                wood: 0,
            })
            .insert_resource(CurrentLocation(LocationId(0)))
            .add_event::<UpdatePartyFoodEvent>()
            .add_event::<UpdatePartyWaterEvent>()
            .add_event::<UpdatePartyWoodEvent>()
            .add_event::<UpdateSettlementFoodEvent>()
            .add_event::<UpdateSettlementWaterEvent>()
            .add_event::<UpdateSettlementWoodEvent>()
            .add_event::<ChangeLocationEvent>()
            .add_event::<SpawnLocationEvent>()
            .add_event::<DespawnLocationEvent>()
            .add_systems(Startup, setup_game_camera)
            .add_systems(
                Update,
                (
                    update_party_food,
                    update_party_water,
                    update_party_wood,
                    update_settlement_food,
                    update_settlement_water,
                    update_settlement_wood,
                ),
            )
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
