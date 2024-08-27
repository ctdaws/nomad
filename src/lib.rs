pub mod locations {
    pub mod config;
    pub mod events;
    pub mod location;
    pub mod plugin;
    pub mod setup;
}
pub mod input;
pub mod plugin;
pub mod ui {
    pub mod encounter;
    pub mod game_over;
    pub mod home;
    pub mod plugin;
    pub mod resources;
}
pub mod events;
pub mod venture;

pub const WINDOW_START_WIDTH: f32 = 1920.;
pub const WINDOW_START_HEIGHT: f32 = 1080.;
