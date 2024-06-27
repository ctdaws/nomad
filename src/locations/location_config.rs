use bevy::math::Vec3;

use super::location::LocationId;

pub struct EncounterConfig {
    pub text: String,
    // The amount of a resource to add/remove from the player
    pub food: Option<i32>,
    pub water: Option<i32>,
    pub wood: Option<i32>,
}

pub struct LocationConfig {
    pub location_id: LocationId,
    pub position: Vec3,
    pub encounter: EncounterConfig,
    pub connected_locations: Vec<LocationId>,
}
