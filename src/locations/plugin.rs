// use bevy::{
//     app::{App, Plugin, Startup, Update},
//     ecs::schedule::IntoSystemConfigs,
//     render::color::Color,
// };

// use crate::plugin::MapSet;

// use super::{
//     events::{
//         location_clicked, move_to_location, reduce_location_encounter_level,
//         show_connected_locations, spawn_location_connections, LocationClicked, MoveToLocation,
//         ReduceLocationEncounterLevel, ShowConnectedLocations, SpawnLocationConnections,
//     },
//     location::{CurrentLocation, Locations, SpawnedConnections},
//     setup::{set_start_location, setup_locations},
// };

// pub const LOCATION_MARKER_Z: f32 = 2.;
// pub const LOCATION_CONNECTION_Z: f32 = 1.;

// pub const CURRENT_LOCATION_COLOUR: Color = Color::MAROON;
// pub const SELECTABLE_LOCATION_COLOUR: Color = Color::ORANGE_RED;
// pub const NOT_SELECTABLE_LOCATION_COLOUR: Color = Color::BLACK;
// pub const LOCATION_CONNECTION_COLOUR: Color = Color::BLACK;

// pub struct LocationsPlugin;

// impl Plugin for LocationsPlugin {
//     fn build(&self, app: &mut App) {
//         app.init_resource::<Locations>()
//             .init_resource::<SpawnedConnections>()
//             .insert_resource::<CurrentLocation>(CurrentLocation(0))
//             .add_event::<LocationClicked>()
//             .add_event::<MoveToLocation>()
//             .add_event::<SpawnLocationConnections>()
//             .add_event::<ShowConnectedLocations>()
//             .add_event::<ReduceLocationEncounterLevel>()
//             .add_systems(
//                 Startup,
//                 (setup_locations, set_start_location).chain().in_set(MapSet),
//             )
//             .add_systems(
//                 Update,
//                 (
//                     location_clicked,
//                     move_to_location,
//                     spawn_location_connections,
//                     show_connected_locations,
//                     reduce_location_encounter_level,
//                 )
//                     .in_set(MapSet),
//             );
//     }
// }
