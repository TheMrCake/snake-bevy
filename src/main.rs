use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Component)]
struct Direction(Vec3);

impl Direction {
    const NORTH: Direction = Direction(Vec3 {
        x: 0f32,
        y: 1f32,
        z: 0f32,
    });
    const EAST: Direction = Direction(Vec3 {
        x: 0f32,
        y: 1f32,
        z: 0f32,
    });
    const SOUTH: Direction = Direction(Vec3 {
        x: 0f32,
        y: 1f32,
        z: 0f32,
    });
    const WEST: Direction = Direction(Vec3 {
        x: 0f32,
        y: 1f32,
        z: 0f32,
    });
}

struct Turn {
    direction: Direction,
    time: Time,
}

#[derive(Component)]
struct Turns(VecDeque<Turn>);

#[derive(Bundle)]
struct Primitive {
    shape: Mesh2d,
    color: MeshMaterial2d<ColorMaterial>,
}

#[derive(Bundle)]
struct SnakeHead {
    direction: Direction,
    turns: Turns,
    trans: Transform,
    segment: Primitive,
}

#[derive(Bundle)]
struct SnakeBody {
    direction: Direction,
    turns: Turns,
    trans: Transform,
    segment: Primitive,
}

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins,))
        .add_systems(Startup, setup);
    app.add_systems(Update, move_snake);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let shape = meshes.add(Rectangle::new(10., 10.));
    let color = materials.add(Color::srgba(0f32, 1f32, 0f32, 1f32));

    commands.spawn(SnakeHead {
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
    });
}

fn move_snake(mut query: Query<(&mut Transform, &Direction)>) {
    // const SPEED: i32 = 1;
    query
        .iter_mut()
        .for_each(|(mut trans, direction)| trans.translation += direction.0);
}
