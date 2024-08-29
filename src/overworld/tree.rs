use bevy::{
    asset::Handle,
    ecs::{bundle::Bundle, component::Component},
    math::{Vec2, Vec3},
    render::texture::Image,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::default,
};

use super::{collisions::CircleCollider, setup::OVERWORLD_INTERACTABLE_ENTITIES_LAYER};

const TREE_INTERACTION_RADIUS: f32 = 10.;

#[derive(Component)]
pub struct Tree;

#[derive(Bundle)]
pub struct TreeBundle {
    marker: Tree,
    interaction_collider: CircleCollider,
    sprite: SpriteBundle,
}

impl TreeBundle {
    pub fn new(position: Vec2, texture: Handle<Image>) -> Self {
        TreeBundle {
            marker: Tree,
            interaction_collider: CircleCollider {
                radius: TREE_INTERACTION_RADIUS,
            },
            sprite: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    position.x,
                    position.y,
                    OVERWORLD_INTERACTABLE_ENTITIES_LAYER,
                )),
                texture,
                sprite: Sprite {
                    custom_size: Some(Vec2::new(90., 150.)),
                    ..default()
                },
                ..default()
            },
        }
    }
}
