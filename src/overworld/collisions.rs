use bevy::{
    ecs::{
        component::Component,
        event::EventWriter,
        query::{With, Without},
        system::Query,
    },
    math::Vec3Swizzles,
    transform::components::Transform,
};

use crate::locations::location::ChangeLocationEvent;

use super::entities::{
    change_location_zone::{ChangeLocationZone, ConnectedLocationId},
    player::Player,
};

const SCREEN_TOP_BOUND: f32 = 540.;
const SCREEN_BOTTOM_BOUND: f32 = -540.;
const SCREEN_LEFT_BOUND: f32 = -960.;
const SCREEN_RIGHT_BOUND: f32 = 960.;

#[derive(Component)]
pub struct CircleCollider {
    pub radius: f32,
}

impl CircleCollider {
    pub fn did_collide_with_circle(
        &self,
        transform: &Transform,
        other_transform: &Transform,
        other_collider: &CircleCollider,
    ) -> bool {
        let distance = transform
            .translation
            .xy()
            .distance(other_transform.translation.xy());

        if distance < (self.radius + other_collider.radius) {
            true
        } else {
            false
        }
    }
}

#[derive(Component)]
pub struct RectangleCollider {
    pub half_width: f32,
    pub half_height: f32,
}

impl RectangleCollider {
    pub fn did_collide_with_rectangle(
        &self,
        transform: &Transform,
        other_transform: &Transform,
        other_collider: &RectangleCollider,
    ) -> bool {
        let translation = transform.translation;
        let other_translation = other_transform.translation;

        if (translation.x + self.half_width) > (other_translation.x - other_collider.half_width)
            && (translation.x - self.half_width) < (other_translation.x + other_collider.half_width)
            && (translation.y + self.half_height)
                > (other_translation.y - other_collider.half_height)
            && (translation.y - self.half_height)
                < (other_translation.y + other_collider.half_height)
        {
            true
        } else {
            false
        }
    }
}

pub fn collisions(
    mut player_query: Query<
        (&mut Transform, &RectangleCollider),
        (With<Player>, Without<ChangeLocationZone>),
    >,
    change_location_zone_query: Query<
        (&Transform, &RectangleCollider, &ConnectedLocationId),
        (With<ChangeLocationZone>, Without<Player>),
    >,
    mut change_location_events: EventWriter<ChangeLocationEvent>,
) {
    let (mut player_transform, player_collider) = player_query.single_mut();

    // We use half width/height here as translation is
    // measured from the centre of the sprite
    if player_transform.translation.y > (SCREEN_TOP_BOUND - player_collider.half_height) {
        player_transform.translation.y = SCREEN_TOP_BOUND - player_collider.half_height;
    }

    if player_transform.translation.y < (SCREEN_BOTTOM_BOUND + player_collider.half_height) {
        player_transform.translation.y = SCREEN_BOTTOM_BOUND + player_collider.half_height;
    }

    if player_transform.translation.x > (SCREEN_RIGHT_BOUND - player_collider.half_width) {
        player_transform.translation.x = SCREEN_RIGHT_BOUND - player_collider.half_width;
    }

    if player_transform.translation.x < (SCREEN_LEFT_BOUND + player_collider.half_width) {
        player_transform.translation.x = SCREEN_LEFT_BOUND + player_collider.half_width;
    }

    for (transform, collider, connected_location_id) in change_location_zone_query.iter() {
        if player_collider.did_collide_with_rectangle(&*player_transform, transform, collider) {
            change_location_events.send(ChangeLocationEvent(connected_location_id.0.clone()));
        }
    }
}
