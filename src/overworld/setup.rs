use bevy::{
    asset::{AssetServer, Handle},
    ecs::system::{Commands, Res},
    math::Vec2,
    render::texture::Image,
    sprite::SpriteBundle,
    transform::components::Transform,
    utils::default,
};

use crate::locations::setup::{CurrentLocation, Locations};

use super::entities::{
    berry_bush::BerryBushBundle, change_location_zone::ChangeLocationZoneBundle,
    player::PlayerBundle, stick::StickBundle, tree::TreeBundle, water_pool::WaterPoolBundle,
};

pub const OVERWORLD_BACKGROUND_LAYER: f32 = 0.;
pub const OVERWORLD_INTERACTABLE_ENTITIES_LAYER: f32 = 1.;
pub const OVERWORLD_PLAYER_LAYER: f32 = 2.;

pub fn setup_overworld(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_location: Res<CurrentLocation>,
    locations: Res<Locations>,
) {
    let tree_texture: Handle<Image> = asset_server.load("textures/tree.png");
    let berry_bush_texture: Handle<Image> = asset_server.load("textures/berry_bush.png");
    let stick_texture: Handle<Image> = asset_server.load("textures/stick.png");
    let water_pool_texture: Handle<Image> = asset_server.load("textures/water_pool.png");
    let change_location_zone_texture: Handle<Image> =
        asset_server.load("textures/change_location_zone.png");

    let config = locations.0[&current_location.0].clone();

    commands.spawn(SpriteBundle {
        texture: asset_server.load("textures/overworld_background.png"),
        transform: Transform::from_xyz(0., 0., OVERWORLD_BACKGROUND_LAYER),
        ..default()
    });

    if let Some(sticks) = config.sticks {
        for stick in sticks {
            commands.spawn(StickBundle::new(
                Vec2::new(stick.x, stick.y),
                stick_texture.clone(),
            ));
        }
    }

    if let Some(trees) = config.trees {
        for tree in trees {
            commands.spawn(TreeBundle::new(
                Vec2::new(tree.x, tree.y),
                tree_texture.clone(),
            ));
        }
    }

    if let Some(berry_bushes) = config.berry_bushes {
        for berry_bush in berry_bushes {
            commands.spawn(BerryBushBundle::new(
                Vec2::new(berry_bush.x, berry_bush.y),
                berry_bush_texture.clone(),
            ));
        }
    }

    if let Some(water_pools) = config.water_pools {
        for water_pool in water_pools {
            commands.spawn(WaterPoolBundle::new(
                Vec2::new(water_pool.x, water_pool.y),
                water_pool_texture.clone(),
            ));
        }
    }

    if let Some(change_location_zones) = config.change_location_zones {
        for change_location_zone in change_location_zones {
            commands.spawn(ChangeLocationZoneBundle::new(
                Vec2::new(change_location_zone.x, change_location_zone.y),
                change_location_zone_texture.clone(),
            ));
        }
    }

    commands.spawn(PlayerBundle::new(asset_server.load("textures/player.png")));
}
