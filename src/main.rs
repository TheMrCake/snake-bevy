use bevy::prelude::*;

#[derive(Component)]
enum Direction {
    North,
    East,
    South,
    West,
}

struct Turn {
    position: Position,
    direction: Direction,
}

#[derive(Component)]
struct Turns(Vec<Turn>);

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
    body: Primitive,
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
        direction: Direction::South,
        position: Position { x: 0., y: 0. },
        turns: Turns(vec![]),
        trans: Transform {
            translation: Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            ..default()
        },
        body: Primitive {
            shape: Mesh2d(shape),
            color: MeshMaterial2d::<ColorMaterial>(color),
        },
    });
}

fn move_snake() {}
