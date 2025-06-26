use std::collections::VecDeque;
use std::default::*;

use crate::{collision::Collider, Mesh, SPEED};
use bevy::{
    asset::Assets,
    color::Color,
    ecs::{
        bundle::Bundle,
        system::{Commands, Query, ResMut},
    },
    math::{primitives::Rectangle, Vec3},
    prelude::{default, Component},
    render::mesh::Mesh2d,
    sprite::{ColorMaterial, MeshMaterial2d},
    time::Timer,
    transform::components::Transform,
};

#[derive(Component, Copy, Clone, Debug)]
pub struct Direction(Vec3);

impl Direction {
    pub const NORTH: Direction = Direction(Vec3 {
        x: 0f32,
        y: 1f32,
        z: 0f32,
    });
    pub const EAST: Direction = Direction(Vec3 {
        x: 1f32,
        y: 0f32,
        z: 0f32,
    });
    pub const SOUTH: Direction = Direction(Vec3 {
        x: 0f32,
        y: -1f32,
        z: 0f32,
    });
    pub const WEST: Direction = Direction(Vec3 {
        x: -1f32,
        y: 0f32,
        z: 0f32,
    });
}

#[derive(Debug)]
pub struct Turn {
    pub direction: Direction,
    pub timer: Timer,
}

#[derive(Component)]
pub struct Turns(pub VecDeque<Turn>);

#[derive(Bundle)]
pub struct Primitive {
    pub shape: Mesh2d,
    pub color: MeshMaterial2d<ColorMaterial>,
}

#[derive(Component)]
pub struct SegmentNumber(pub i32);

#[derive(Component)]
pub struct Head;

#[derive(Component)]
pub struct PlayerKiller;

#[derive(Bundle)]
pub struct SnakeSegment {
    direction: Direction,
    turns: Turns,
    trans: Transform,
    segment: Primitive,
    segment_number: SegmentNumber,
}

#[derive(Component)]
pub struct Food;

#[derive(Bundle)]
pub struct Item {
    trans: Transform,
    prim: Primitive,
}

pub fn move_snake_segment(mut query: Query<(&mut Transform, &Direction)>) {
    query
        .iter_mut()
        .for_each(|(mut trans, direction)| trans.translation += direction.0 * SPEED);
}

pub fn turn_snake_segment(mut query: Query<(&mut Turns, &mut Direction)>) {
    query.iter_mut().for_each(|(mut turns, mut direction)| {
        // dbg!(&turns.0);
        if let Some(turn) = turns.0.pop_front() {
            // dbg!(&turn.timer);
            if turn.timer.finished() {
                *direction = turn.direction;
            } else {
                turns.0.push_front(turn);
            }
        }
    });
}

pub fn spawn_food(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = meshes.add(Rectangle::new(10., 10.));
    let color = materials.add(Color::srgba(1f32, 0f32, 0f32, 1f32));

    let x = (fastrand::f32() - fastrand::f32()) * 1000f32;
    let y = (fastrand::f32() - fastrand::f32()) * 1000f32;
    let z = (fastrand::f32() - fastrand::f32()) * 1000f32;

    commands.spawn((
        Item {
            trans: Transform::from_xyz(x, y, z),
            prim: Primitive {
                shape: Mesh2d(shape),
                color: MeshMaterial2d::<ColorMaterial>(color),
            },
        },
        Food,
        Collider,
    ));
}

pub fn grow_snake(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let shape = meshes.add(Rectangle::new(10., 10.));
    let color = materials.add(Color::srgba(0f32, 1f32, 0f32, 1f32));

    commands.spawn((
        SnakeSegment {
            direction: Direction::SOUTH,
            turns: Turns(VecDeque::new()),
            trans: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                ..default()
            },
            segment: Primitive {
                shape: Mesh2d(shape),
                color: MeshMaterial2d::<ColorMaterial>(color),
            },
            segment_number: SegmentNumber(0),
        },
        Collider,
        Head,
    ));
}
