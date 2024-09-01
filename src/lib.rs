pub mod game_plugin;
pub mod overworld {
    pub mod berry_bush;
    pub mod collisions;
    pub mod party_resources;
    pub mod player;
    pub mod plugin;
    pub mod setup;
    pub mod stick;
    pub mod tree;
    pub mod water_pool;
    pub mod ui {
        pub mod party_resources;
        pub mod plugin;
    }
}

pub const WINDOW_START_WIDTH: f32 = 1920.;
pub const WINDOW_START_HEIGHT: f32 = 1080.;
