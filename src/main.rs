use bevy::{prelude::*, window::WindowResolution};
use nomad::{game_plugin::GamePlugin, WINDOW_START_HEIGHT, WINDOW_START_WIDTH};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Nomad".to_string(),
                    resolution: WindowResolution::new(WINDOW_START_WIDTH, WINDOW_START_HEIGHT),
                    ..default()
                }),
                ..default()
            }),
            GamePlugin,
        ))
        .run();
}
