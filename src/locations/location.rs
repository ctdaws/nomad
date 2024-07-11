use bevy::{
    ecs::{bundle::Bundle, component::Component, entity::Entity, event::Event, system::Resource},
    sprite::SpriteBundle,
    utils::HashMap,
};

#[derive(Resource, Default)]
pub struct Locations(pub HashMap<u32, Entity>);

#[derive(Resource)]
pub struct CurrentLocation(pub u32);

#[derive(Event)]
pub struct LocationSelected(pub u32);

#[derive(Component)]
pub struct Location;

#[derive(Component, Clone, Copy)]
pub struct LocationId(pub u32);

#[derive(Component, Clone, Debug)]
pub struct Interaction {
    pub text: String,
    // The amount of a resource to add/remove from the player
    pub food: Option<i32>,
    pub water: Option<i32>,
    pub wood: Option<i32>,
}

#[derive(Component)]
pub struct Encounter {
    pub text: String,
    pub interactions: Vec<Interaction>,
}

#[derive(Component)]
pub struct SquareCollider {
    pub half_width: f32,
    pub half_height: f32,
}

#[derive(Component)]
pub enum LocationState {
    Current,
    Selectable,
    NotSelectable,
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
