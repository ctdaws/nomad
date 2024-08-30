use bevy::{
    asset::Handle,
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        event::EventWriter,
        query::With,
        system::{ParamSet, Query, Res},
    },
    input::{keyboard::KeyCode, ButtonInput},
    math::{Vec2, Vec3, Vec3Swizzles},
    render::texture::Image,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::default,
};

use super::{
    berry_bush::{BerryBush, BerryBushPickedEvent, BerryBushState},
    collisions::{CircleCollider, SquareCollider},
    setup::OVERWORLD_PLAYER_LAYER,
    stick::{Stick, StickPickedUpEvent},
    water_pool::{WaterCollectedEvent, WaterPool},
};

const PLAYER_SPEED: f32 = 7.;
const PLAYER_INTERACTION_RADIUS: f32 = 40.;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: Player,
    speed: Speed,
    collider: SquareCollider,
    interaction_collider: CircleCollider,
    sprite: SpriteBundle,
}

impl PlayerBundle {
    pub fn new(texture: Handle<Image>) -> Self {
        let size = Vec2::new(70., 70.);

        PlayerBundle {
            marker: Player,
            speed: Speed(PLAYER_SPEED),
            collider: SquareCollider {
                half_width: size.x / 2.,
                half_height: size.y / 2.,
            },
            interaction_collider: CircleCollider {
                radius: PLAYER_INTERACTION_RADIUS,
            },
            sprite: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., OVERWORLD_PLAYER_LAYER)),
                texture,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(size.x, size.y)),
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn update_player_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Speed), With<Player>>,
) {
    let mut move_vector = Vec2::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        move_vector += Vec2::new(0., 1.);
    }

    if keys.pressed(KeyCode::KeyS) {
        move_vector += Vec2::new(0., -1.);
    }

    if keys.pressed(KeyCode::KeyA) {
        move_vector += Vec2::new(-1., 0.);
    }

    if keys.pressed(KeyCode::KeyD) {
        move_vector += Vec2::new(1., 0.);
    }

    let (mut transform, speed) = query.single_mut();

    transform.translation =
        transform.translation + (Vec3::new(move_vector.x, move_vector.y, 0.) * speed.0);
}

fn did_collide(
    player_transform: &Transform,
    player_collider: &CircleCollider,
    interactable_transform: &Transform,
    interactable_collider: &CircleCollider,
) -> bool {
    let distance = player_transform
        .translation
        .xy()
        .distance(interactable_transform.translation.xy());

    if distance < (player_collider.radius + interactable_collider.radius) {
        true
    } else {
        false
    }
}

pub fn process_player_interaction(
    keys: Res<ButtonInput<KeyCode>>,
    player_query: Query<(&Transform, &CircleCollider), With<Player>>,
    mut interactables: ParamSet<(
        Query<(Entity, &Transform, &CircleCollider), With<Stick>>,
        Query<(Entity, &BerryBushState, &Transform, &CircleCollider), With<BerryBush>>,
        Query<(&Transform, &CircleCollider), With<WaterPool>>,
    )>,
    mut stick_picked_up_events: EventWriter<StickPickedUpEvent>,
    mut berry_bush_picked_events: EventWriter<BerryBushPickedEvent>,
    mut water_collected_events: EventWriter<WaterCollectedEvent>,
) {
    if keys.just_pressed(KeyCode::Space) {
        let (player_tranform, player_interaction_collider) = player_query.single();

        for (id, stick_transform, stick_interaction_collider) in interactables.p0().iter() {
            if did_collide(
                player_tranform,
                player_interaction_collider,
                stick_transform,
                stick_interaction_collider,
            ) {
                stick_picked_up_events.send(StickPickedUpEvent(id));
            }
        }

        for (id, state, berry_bush_transform, berry_bush_interaction_collider) in
            interactables.p1().iter()
        {
            if did_collide(
                player_tranform,
                player_interaction_collider,
                berry_bush_transform,
                berry_bush_interaction_collider,
            ) {
                if !matches!(state, BerryBushState::Picked) {
                    berry_bush_picked_events.send(BerryBushPickedEvent(id));
                }
            }
        }

        for (water_pool_transform, water_pool_interaction_collider) in interactables.p2().iter() {
            if did_collide(
                player_tranform,
                player_interaction_collider,
                water_pool_transform,
                water_pool_interaction_collider,
            ) {
                water_collected_events.send(WaterCollectedEvent());
            }
        }
    }
}
