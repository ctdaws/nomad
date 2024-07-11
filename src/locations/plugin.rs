use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
    render::color::Color,
};

use super::{
    events::location_selected,
    location::{CurrentLocation, LocationSelected, Locations},
    setup::{set_start_location, setup_location_connections, setup_locations},
};

pub const LOCATION_MARKER_Z: f32 = 2.;
pub const LOCATION_CONNECTION_Z: f32 = 1.;

pub const CURRENT_LOCATION_COLOUR: Color = Color::MAROON;
pub const SELECTABLE_LOCATION_COLOUR: Color = Color::ORANGE_RED;
pub const NOT_SELECTABLE_LOCATION_COLOUR: Color = Color::BLACK;
pub const LOCATION_CONNECTION_COLOUR: Color = Color::BLACK;

pub struct LocationsPlugin;

impl Plugin for LocationsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Locations>()
            .insert_resource::<CurrentLocation>(CurrentLocation(0))
            .add_event::<LocationSelected>()
            .add_systems(
                Startup,
                (
                    setup_locations,
                    setup_location_connections,
                    set_start_location,
                )
                    .chain(),
            )
            .add_systems(Update, location_selected);
    }
}
