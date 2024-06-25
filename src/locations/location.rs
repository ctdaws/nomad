use bevy::{
    asset::AssetServer,
    ecs::{bundle::Bundle, component::Component, system::Res},
    math::{Vec2, Vec3},
    render::color::Color,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};

use super::location_config::LocationId;

#[derive(Component)]
pub struct Location;

#[derive(Component)]
pub struct Encounter {
    pub text: String,
    // The amount of a resource to add/remove from the player
    pub food: Option<i32>,
    pub water: Option<i32>,
    pub wood: Option<i32>,
}

#[derive(Component)]
pub struct SquareCollider {
    pub half_width: f32,
    pub half_height: f32,
}

#[derive(Component)]
pub enum LocationState {
    Current,
    Clickable,
    NotClickable,
}

#[derive(Component)]
pub struct ConnectedLocations(pub Vec<LocationId>);

#[derive(Bundle)]
pub struct LocationBundle {
    pub marker: Location,
    pub id: LocationId,

    pub encounter: Encounter,
    pub state: LocationState,
    pub connected_locations: ConnectedLocations,

    pub collider: SquareCollider,
    pub sprite: SpriteBundle,
}

impl LocationBundle {
    pub fn new(
        id: LocationId,
        position: Vec3,
        encounter: Encounter,
        connected_locations: ConnectedLocations,
    ) -> Self {
        LocationBundle {
            marker: Location,
            id,
            encounter,
            collider: SquareCollider {
                half_width: 25.,
                half_height: 25.,
            },
            state: LocationState::NotClickable,
            connected_locations,
            sprite: SpriteBundle {
                transform: Transform::from_translation(position),
                // texture: asset_server.load("textures/location.png"),
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(50., 50.)),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}
