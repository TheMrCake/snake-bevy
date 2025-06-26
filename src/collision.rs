use bevy::ecs::query::{With, Without};
use bevy::ecs::system::{Query, Single};
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::{Component, Event, EventWriter, GlobalTransform};

use crate::ingame::{Food, Head, PlayerKiller};

#[derive(Component)]
pub struct Collider;

#[derive(Event, Copy, Clone)]
pub struct FoodEaten;

#[derive(Event, Copy, Clone)]
pub struct PlayerKilled;

fn detect_collision<T: Event + Clone>(
    head_transform: &GlobalTransform,
    colliders_transforms: Vec<&GlobalTransform>,
    mut event_writer: EventWriter<T>,
    event: &T,
) {
    // Get iterators for all colliders
    let mut iter = colliders_transforms
        .iter()
        .map(|trans| Aabb2d::new(trans.translation().truncate(), trans.scale().truncate()));

    let head_aabb = Aabb2d::new(
        head_transform.translation().truncate(),
        head_transform.scale().truncate(),
    );

    while let Some(aabb) = iter.next() {
        if aabb.intersects(&head_aabb) {
            event_writer.write(event.clone());
        }
    }
}

pub fn detect_collision_player_kill(
    event_writer: EventWriter<PlayerKilled>,
    collider_query: Query<&GlobalTransform, (With<Collider>, With<PlayerKiller>)>,
    head_query: Single<&GlobalTransform, Without<Head>>,
) {
    detect_collision(
        head_query.into_inner(),
        collider_query.iter().collect(),
        event_writer,
        &PlayerKilled,
    );
}

pub fn detect_collision_player_eat(
    event_writer: EventWriter<FoodEaten>,
    collider_query: Query<&GlobalTransform, (With<Collider>, With<Food>)>,
    head_query: Single<&GlobalTransform, With<Head>>,
) {
    detect_collision(
        head_query.into_inner(),
        collider_query.iter().collect(),
        event_writer,
        &FoodEaten,
    );
}
