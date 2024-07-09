use bevy::math::Vec3;

use super::location::LocationId;

pub struct EncounterOptionConfig {
    pub text: String,
    // The amount of a resource to add/remove from the player
    pub food: Option<i32>,
    pub water: Option<i32>,
    pub wood: Option<i32>,
}

pub struct EncounterConfig {
    pub text: String,
    pub options: Vec<EncounterOptionConfig>,
}

pub struct LocationConfig {
    pub location_id: LocationId,
    pub position: Vec3,
    pub encounter: EncounterConfig,
    pub connected_locations: Vec<LocationId>,
}
