mod collision;
mod ingame;

use std::collections::VecDeque;

use bevy::prelude::*;
use collision::{
    detect_collision_player_eat, detect_collision_player_kill, Collider, FoodEaten, PlayerKilled,
};
use ingame::{
    move_snake_segment, spawn_food, turn_snake_segment, Direction, Head, Primitive, SegmentNumber,
    SnakeSegment, Turn, Turns,
};

const SPEED: f32 = 1f32;

// #[derive(Resource, Default)]
// struct GameState {
//     snake_length: i32,
// }

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins,))
        .add_event::<FoodEaten>()
        .add_event::<PlayerKilled>()
        .add_systems(Startup, (setup, spawn_food))
        .add_systems(
            Update,
            (
                move_snake_segment,
                turn_snake_segment,
                keyboard_input,
                detect_collision_player_eat,
                detect_collision_player_kill,
            ),
        )
        .run();
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
        Collider,
        Head,
    ));
}
