use bevy::{
    ecs::{query::With, system::Query},
    transform::components::Transform,
};

use super::player::{Player, Size};

pub fn process_collisions(mut query: Query<(&mut Transform, &Size), With<Player>>) {
    let top_wall_bound = 540.;
    let bottom_wall_bound = -540.;
    let left_wall_bound = -960.;
    let right_wall_bound = 960.;

    let (mut transform, size) = query.single_mut();

    // Need half width/height here as translation is
    // measured from the centre of the sprite
    let player_half_width = size.x / 2.;
    let player_half_height = size.y / 2.;

    if transform.translation.y > (top_wall_bound - player_half_height) {
        transform.translation.y = top_wall_bound - player_half_height;
    }

    if transform.translation.y < (bottom_wall_bound + player_half_height) {
        transform.translation.y = bottom_wall_bound + player_half_height;
    }

    if transform.translation.x > (right_wall_bound - player_half_width) {
        transform.translation.x = right_wall_bound - player_half_width;
    }

    if transform.translation.x < (left_wall_bound + player_half_width) {
        transform.translation.x = left_wall_bound + player_half_width;
    }
}
