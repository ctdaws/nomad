use bevy::{
    asset::AssetServer,
    ecs::{
        bundle::Bundle,
        component::Component,
        system::{Commands, Res},
    },
    math::Vec3,
    sprite::SpriteBundle,
    transform::components::Transform,
};

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

#[derive(Bundle)]
pub struct LocationBundle {
    pub marker: Location,
    pub encounter: Encounter,
    pub collider: SquareCollider,
    pub sprite: SpriteBundle,
}

impl LocationBundle {
    fn new(position: Vec3, encounter: Encounter, asset_server: &Res<AssetServer>) -> Self {
        LocationBundle {
            marker: Location,
            encounter,
            collider: SquareCollider {
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

pub fn setup_locations(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LocationBundle::new(
        Vec3::new(-550., -150., 1.),
        Encounter {
            text: "Home".to_string(),
            food: None,
            water: None,
            wood: None,
        },
        &asset_server,
    ));

    commands.spawn(LocationBundle::new(
        Vec3::new(-300., 50., 1.),
        Encounter {
            text: "You find a berry bush by a stream".to_string(),
            food: Some(10),
            water: Some(20),
            wood: None,
        },
        &asset_server,
    ));
    commands.spawn(LocationBundle::new(
        Vec3::new(0., 200., 1.),
        Encounter {
            text: "You start a fire and have a meal".to_string(),
            food: Some(-10),
            water: Some(-10),
            wood: Some(-10),
        },
        &asset_server,
    ));
}
