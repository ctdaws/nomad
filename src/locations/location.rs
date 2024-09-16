use bevy::{
    asset::{AssetServer, Handle},
    ecs::{
        entity::Entity,
        event::{Event, EventReader},
        query::With,
        system::{Commands, Query, Res, ResMut},
    },
    hierarchy::{BuildChildren, DespawnRecursiveExt},
    math::Vec2,
    render::texture::Image,
    sprite::SpriteBundle,
    transform::components::Transform,
    utils::default,
};

use crate::overworld::{
    entities::{
        berry_bush::BerryBushBundle, change_location_zone::ChangeLocationZoneBundle,
        player::PlayerBundle, stick::StickBundle, tree::TreeBundle, water_pool::WaterPoolBundle,
    },
    setup::{OverworldScene, OVERWORLD_BACKGROUND_LAYER},
};

use super::setup::{CurrentLocation, LocationId, Locations};

#[derive(Event)]
pub struct ChangeLocationEvent(pub LocationId);

pub fn load_location(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    location: &LocationId,
    locations: &Res<Locations>,
    overworld_scene_id: &mut Entity,
) {
    let tree_texture: Handle<Image> = asset_server.load("textures/tree.png");
    let berry_bush_texture: Handle<Image> = asset_server.load("textures/berry_bush.png");
    let stick_texture: Handle<Image> = asset_server.load("textures/stick.png");
    let water_pool_texture: Handle<Image> = asset_server.load("textures/water_pool.png");
    let change_location_zone_texture: Handle<Image> =
        asset_server.load("textures/change_location_zone.png");

    let config = locations.0[location].clone();

    let mut entities: Vec<Entity> = vec![];

    entities.push(
        commands
            .spawn(SpriteBundle {
                texture: asset_server.load("textures/overworld_background.png"),
                transform: Transform::from_xyz(0., 0., OVERWORLD_BACKGROUND_LAYER),
                ..default()
            })
            .id(),
    );

    if let Some(sticks) = config.sticks {
        for stick in sticks {
            entities.push(
                commands
                    .spawn(StickBundle::new(
                        Vec2::new(stick.x, stick.y),
                        stick_texture.clone(),
                    ))
                    .id(),
            );
        }
    }

    if let Some(trees) = config.trees {
        for tree in trees {
            entities.push(
                commands
                    .spawn(TreeBundle::new(
                        Vec2::new(tree.x, tree.y),
                        tree_texture.clone(),
                    ))
                    .id(),
            );
        }
    }

    if let Some(berry_bushes) = config.berry_bushes {
        for berry_bush in berry_bushes {
            entities.push(
                commands
                    .spawn(BerryBushBundle::new(
                        Vec2::new(berry_bush.x, berry_bush.y),
                        berry_bush_texture.clone(),
                    ))
                    .id(),
            );
        }
    }

    if let Some(water_pools) = config.water_pools {
        for water_pool in water_pools {
            entities.push(
                commands
                    .spawn(WaterPoolBundle::new(
                        Vec2::new(water_pool.x, water_pool.y),
                        water_pool_texture.clone(),
                    ))
                    .id(),
            );
        }
    }

    if let Some(change_location_zones) = config.change_location_zones {
        for change_location_zone in change_location_zones {
            entities.push(
                commands
                    .spawn(ChangeLocationZoneBundle::new(
                        Vec2::new(change_location_zone.x, change_location_zone.y),
                        change_location_zone_texture.clone(),
                        LocationId(change_location_zone.connected_location),
                    ))
                    .id(),
            );
        }
    }

    commands
        .entity(*overworld_scene_id)
        .push_children(entities.as_slice());
}

pub fn change_location(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut current_location: ResMut<CurrentLocation>,
    locations: Res<Locations>,
    mut change_location_events: EventReader<ChangeLocationEvent>,
    mut overworld_scene: Query<Entity, With<OverworldScene>>,
) {
    for ev in change_location_events.read() {
        commands
            .entity(overworld_scene.single_mut())
            .despawn_descendants();

        load_location(
            &mut commands,
            &asset_server,
            &ev.0,
            &locations,
            &mut overworld_scene.single_mut(),
        );

        let player = commands
            .spawn(PlayerBundle::new(asset_server.load("textures/player.png")))
            .id();

        commands
            .entity(overworld_scene.single_mut())
            .push_children(&[player]);

        current_location.0 = ev.0.clone();
    }
}
