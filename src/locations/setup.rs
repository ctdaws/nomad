use std::fs;

use bevy::{
    asset::{AssetServer, Handle},
    ecs::{
        event::EventWriter,
        system::{Commands, Res, ResMut},
    },
    math::Vec2,
    render::{texture::Image, view::VisibilityBundle},
    sprite::SpriteBundle,
    transform::{bundles::TransformBundle, components::Transform},
    utils::{default, HashMap},
};

use crate::overworld::{
    entities::{
        berry_bush::BerryBushBundle, change_location_zone::ChangeLocationZoneBundle,
        stick::StickBundle, stockpile::StockpileBundle, tree::TreeBundle,
        water_pool::WaterPoolBundle,
    },
    setup::OVERWORLD_BACKGROUND_LAYER,
};

use super::{
    config::{LocationConfig, LocationConfigs},
    location::{
        Location, LocationEntityId, LocationId, LocationScene, Locations, SpawnLocationEvent,
    },
};

pub fn load_location_configs() -> HashMap<String, LocationConfig> {
    let content = fs::read_to_string("config/locations.toml").unwrap();

    let location_configs: LocationConfigs = toml::from_str(&content).unwrap();
    location_configs.locations
}

pub fn setup_locations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut locations: ResMut<Locations>,
) {
    let tree_texture: Handle<Image> = asset_server.load("textures/tree.png");
    let berry_bush_texture: Handle<Image> = asset_server.load("textures/berry_bush.png");
    let stick_texture: Handle<Image> = asset_server.load("textures/stick.png");
    let water_pool_texture: Handle<Image> = asset_server.load("textures/water_pool.png");
    let change_location_zone_texture: Handle<Image> =
        asset_server.load("textures/change_location_zone.png");
    let stockpile_texture: Handle<Image> = asset_server.load("textures/settlement_stockpile.png");

    let location_configs = load_location_configs();

    for (id, config) in location_configs {
        let background = SpriteBundle {
            texture: asset_server.load(format!("textures/{}.png", config.background)),
            transform: Transform::from_xyz(0., 0., OVERWORLD_BACKGROUND_LAYER),
            ..default()
        };

        let mut location_entity_id = 0;
        let sticks = if let Some(sticks) = config.sticks {
            let mut map = HashMap::new();

            for stick in sticks {
                map.insert(
                    LocationEntityId(location_entity_id),
                    StickBundle::new(Vec2::new(stick.x, stick.y), stick_texture.clone()),
                );
                location_entity_id += 1;
            }

            Some(map)
        } else {
            None
        };

        let trees = if let Some(trees) = config.trees {
            let mut map = HashMap::new();

            for tree in trees {
                map.insert(
                    LocationEntityId(location_entity_id),
                    TreeBundle::new(Vec2::new(tree.x, tree.y), tree_texture.clone()),
                );
                location_entity_id += 1;
            }

            Some(map)
        } else {
            None
        };

        let berry_bushes = if let Some(berry_bushes) = config.berry_bushes {
            let mut map = HashMap::new();

            for berry_bush in berry_bushes {
                map.insert(
                    LocationEntityId(location_entity_id),
                    BerryBushBundle::new(
                        Vec2::new(berry_bush.x, berry_bush.y),
                        berry_bush_texture.clone(),
                    ),
                );
                location_entity_id += 1;
            }

            Some(map)
        } else {
            None
        };

        let water_pools = if let Some(water_pools) = config.water_pools {
            let mut map = HashMap::new();

            for water_pool in water_pools {
                map.insert(
                    LocationEntityId(location_entity_id),
                    WaterPoolBundle::new(
                        Vec2::new(water_pool.x, water_pool.y),
                        water_pool_texture.clone(),
                    ),
                );
                location_entity_id += 1;
            }

            Some(map)
        } else {
            None
        };

        let change_location_zones =
            if let Some(change_location_zones) = config.change_location_zones {
                let mut map = HashMap::new();

                for change_location_zone in change_location_zones {
                    map.insert(
                        LocationEntityId(location_entity_id),
                        ChangeLocationZoneBundle::new(
                            Vec2::new(change_location_zone.x, change_location_zone.y),
                            change_location_zone.rotation_degrees,
                            change_location_zone_texture.clone(),
                            LocationId(change_location_zone.connected_location),
                        ),
                    );
                    location_entity_id += 1;
                }

                Some(map)
            } else {
                None
            };

        let stockpile = if let Some(stockpile) = config.stockpile {
            Some(StockpileBundle::new(
                Vec2::new(stockpile.x, stockpile.y),
                stockpile_texture.clone(),
            ))
        } else {
            None
        };

        let location = Location {
            background,
            sticks,
            trees,
            berry_bushes,
            water_pools,
            change_location_zones,
            stockpile,
        };

        locations
            .0
            .insert(LocationId(id.parse().unwrap()), location);
    }

    commands.spawn((
        LocationScene,
        TransformBundle::default(),
        VisibilityBundle::default(),
    ));
}

pub fn spawn_initial_location(mut spawn_location_events: EventWriter<SpawnLocationEvent>) {
    spawn_location_events.send(SpawnLocationEvent(LocationId(0)));
}
