use bevy::{
    ecs::{
        query::With,
        system::{ParamSet, Query},
    },
    transform::components::Transform,
};

use crate::game_plugin::GameCamera;

use super::player::Player;

pub fn update_camera_position(
    mut transforms: ParamSet<(
        Query<&mut Transform, With<GameCamera>>,
        Query<&Transform, With<Player>>,
    )>,
) {
    transforms.p0().single_mut().translation = transforms.p1().single_mut().translation;
}
