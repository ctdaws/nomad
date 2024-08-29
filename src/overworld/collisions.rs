use bevy::{
    ecs::{component::Component, query::With, system::Query},
    transform::components::Transform,
};

use super::player::Player;

#[derive(Component)]
pub struct CircleCollider {
    pub radius: f32,
}

#[derive(Component)]
pub struct SquareCollider {
    pub half_width: f32,
    pub half_height: f32,
}

pub fn process_collisions(mut query: Query<(&mut Transform, &SquareCollider), With<Player>>) {
    let top_wall_bound = 540.;
    let bottom_wall_bound = -540.;
    let left_wall_bound = -960.;
    let right_wall_bound = 960.;

    let (mut transform, collider) = query.single_mut();

    // We use half width/height here as translation is
    // measured from the centre of the sprite
    if transform.translation.y > (top_wall_bound - collider.half_height) {
        transform.translation.y = top_wall_bound - collider.half_height;
    }

    if transform.translation.y < (bottom_wall_bound + collider.half_height) {
        transform.translation.y = bottom_wall_bound + collider.half_height;
    }

    if transform.translation.x > (right_wall_bound - collider.half_width) {
        transform.translation.x = right_wall_bound - collider.half_width;
    }

    if transform.translation.x < (left_wall_bound + collider.half_width) {
        transform.translation.x = left_wall_bound + collider.half_width;
    }
}
