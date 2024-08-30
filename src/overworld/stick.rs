use bevy::{
    asset::Handle,
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        event::{Event, EventReader},
        system::Commands,
    },
    math::{Vec2, Vec3},
    render::texture::Image,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::default,
};

use super::{collisions::CircleCollider, setup::OVERWORLD_INTERACTABLE_ENTITIES_LAYER};

const STICK_INTERACTION_RADIUS: f32 = 40.;

#[derive(Component)]
pub struct Stick;

#[derive(Event)]
pub struct StickPickedUpEvent(pub Entity);

#[derive(Bundle)]
pub struct StickBundle {
    marker: Stick,
    interaction_collider: CircleCollider,
    sprite: SpriteBundle,
}

impl StickBundle {
    pub fn new(position: Vec2, texture: Handle<Image>) -> Self {
        StickBundle {
            marker: Stick,
            interaction_collider: CircleCollider {
                radius: STICK_INTERACTION_RADIUS,
            },
            sprite: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    position.x,
                    position.y,
                    OVERWORLD_INTERACTABLE_ENTITIES_LAYER,
                )),
                texture,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(80., 80.)),
                    ..default()
                },
                ..default()
            },
        }
    }
}

pub fn pick_up_stick(
    mut stick_picked_up_events: EventReader<StickPickedUpEvent>,
    mut commands: Commands,
) {
    for ev in stick_picked_up_events.read() {
        commands.entity(ev.0).despawn();
    }
}
