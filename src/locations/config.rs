use bevy::utils::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct StickConfig {
    pub x: f32,
    pub y: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TreeConfig {
    pub x: f32,
    pub y: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BerryBushConfig {
    pub x: f32,
    pub y: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WaterPoolConfig {
    pub x: f32,
    pub y: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChangeLocationZoneConfig {
    pub x: f32,
    pub y: f32,
    pub connected_location: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LocationConfig {
    pub sticks: Option<Vec<StickConfig>>,
    pub trees: Option<Vec<TreeConfig>>,
    pub berry_bushes: Option<Vec<BerryBushConfig>>,
    pub water_pools: Option<Vec<WaterPoolConfig>>,
    pub change_location_zones: Option<Vec<ChangeLocationZoneConfig>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LocationConfigs {
    pub locations: HashMap<String, LocationConfig>,
}
