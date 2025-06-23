use std::collections::VecDeque;

use bevy::prelude::*;

const SPEED: f32 = 1f32;

#[derive(Resource, Default)]
struct GameState {
    snake_length: i32,
}

#[derive(Component, Copy, Clone, Debug)]
struct Direction(Vec3);

impl Direction {
    const NORTH: Direction = Direction(Vec3 {
        x: 0f32,
        y: 1f32,
        z: 0f32,
    });
    const EAST: Direction = Direction(Vec3 {
        x: 1f32,
        y: 0f32,
        z: 0f32,
    });
    const SOUTH: Direction = Direction(Vec3 {
        x: 0f32,
        y: -1f32,
        z: 0f32,
    });
    const WEST: Direction = Direction(Vec3 {
        x: -1f32,
        y: 0f32,
        z: 0f32,
    });
}

#[derive(Debug)]
struct Turn {
    direction: Direction,
    timer: Timer,
}

#[derive(Component)]
struct Turns(VecDeque<Turn>);

#[derive(Bundle)]
struct Primitive {
    shape: Mesh2d,
    color: MeshMaterial2d<ColorMaterial>,
}

#[derive(Component)]
struct SegmentNumber(i32);

#[derive(Component)]
struct Head;

#[derive(Component)]
struct Body;

#[derive(Bundle)]
struct SnakeSegment {
    direction: Direction,
    turns: Turns,
    trans: Transform,
    segment: Primitive,
    segment_number: SegmentNumber,
}

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins,))
        .add_systems(Startup, setup);
    app.add_systems(
        Update,
        (move_snake_segment, turn_snake_segment, keyboard_input),
    );
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
        Head,
    ));
}

fn move_snake_segment(mut query: Query<(&mut Transform, &Direction)>) {
    query
        .iter_mut()
        .for_each(|(mut trans, direction)| trans.translation += direction.0 * SPEED);
}

fn turn_snake_segment(mut query: Query<(&mut Turns, &mut Direction)>) {
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

fn keyboard_input(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Turns, &SegmentNumber)>,
) {
    keyboard_input.get_pressed().for_each(|key_code| {
        // println!("Key pressed: {:?}", key_code);
        query.iter_mut().for_each(|(mut turns, segment_number)| {
            let mut timer = Timer::from_seconds(SPEED * segment_number.0 as f32, TimerMode::Once);
            timer.tick(time.delta());
            // println!("HELLO HELLO HELLO\n\n\n\n\n\n\nHELLO HELLO HELLO");
            match key_code {
                KeyCode::KeyW => turns.0.push_back(Turn {
                    direction: Direction::NORTH,
                    timer,
                }),
                KeyCode::KeyD => turns.0.push_back(Turn {
                    direction: Direction::EAST,
                    timer,
                }),
                KeyCode::KeyS => turns.0.push_back(Turn {
                    direction: Direction::SOUTH,
                    timer,
                }),
                KeyCode::KeyA => turns.0.push_back(Turn {
                    direction: Direction::WEST,
                    timer,
                }),
                _ => (),
            }
        });
    });
}
