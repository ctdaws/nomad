// use bevy::{
//     ecs::{
//         event::EventWriter,
//         query::With,
//         system::{Query, Res, ResMut, Resource},
//     },
//     input::{mouse::MouseButton, ButtonInput},
//     math::Vec2,
//     render::camera::Camera,
//     transform::components::{GlobalTransform, Transform},
//     window::{PrimaryWindow, Window},
// };

// use crate::{
//     locations::{
//         events::LocationClicked,
//         location::{Location, LocationId, SquareCollider},
//     },
//     plugin::GameCamera,
// };

// #[derive(Resource, Default)]
// pub struct CursorWorldCoords(Vec2);

// pub fn update_cursor_position(
//     mut cursor_world_coords: ResMut<CursorWorldCoords>,
//     query_window: Query<&Window, With<PrimaryWindow>>,
//     query_camera: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
// ) {
//     let (camera, camera_transform) = query_camera.single();
//     let window = query_window.single();

//     if let Some(world_position) = window
//         .cursor_position()
//         .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
//         .map(|ray| ray.origin.truncate())
//     {
//         cursor_world_coords.0 = world_position;
//     }
// }

// pub fn process_mouse_click(
//     cursor_world_coords: Res<CursorWorldCoords>,
//     mouse_buttons: Res<ButtonInput<MouseButton>>,
//     query: Query<(&Transform, &SquareCollider, &LocationId), With<Location>>,
//     mut location_clicked_events: EventWriter<LocationClicked>,
// ) {
//     if mouse_buttons.just_released(MouseButton::Left) {
//         for (transform, collider, location_id) in &query {
//             let pos = transform.translation;
//             let cursor_pos = cursor_world_coords.0;

//             if (cursor_pos.x <= pos.x + collider.half_width)
//                 && (cursor_pos.x >= pos.x - collider.half_width)
//                 && (cursor_pos.y <= pos.y + collider.half_height)
//                 && (cursor_pos.y >= pos.y - collider.half_height)
//             {
//                 location_clicked_events.send(LocationClicked(location_id.0));
//             }
//         }
//     }
// }
