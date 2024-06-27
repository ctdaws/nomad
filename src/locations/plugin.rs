use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::schedule::IntoSystemConfigs,
};

use super::location::{
    location_selected, set_start_location, setup_locations, CurrentLocation, LocationId,
    LocationSelected, Locations,
};

pub struct LocationsPlugin;

impl Plugin for LocationsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Locations>()
            .insert_resource::<CurrentLocation>(CurrentLocation(LocationId(0)))
            .add_event::<LocationSelected>()
            .add_systems(Startup, (setup_locations, set_start_location).chain())
            .add_systems(Update, location_selected);
    }
}
