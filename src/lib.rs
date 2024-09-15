pub mod game_plugin;
pub mod overworld {
    pub mod locations {
        pub mod config;
    }
    pub mod change_location;
    pub mod collisions;
    pub mod entities {
        pub mod berry_bush;
        pub mod change_location_zone;
        pub mod player;
        pub mod stick;
        pub mod tree;
        pub mod water_pool;
    }
    pub mod party_resources;
    pub mod plugin;
    pub mod setup;
    pub mod ui {
        pub mod party_resources;
        pub mod plugin;
    }
}

pub const WINDOW_START_WIDTH: f32 = 1920.;
pub const WINDOW_START_HEIGHT: f32 = 1080.;
