use bevy::prelude::*;
use bevy::window::PrimaryWindow;

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_WIDTH: f32 = 292.0;
const PLAYER_HEIGHT: f32 = 208.0;
const NUM_ENEMIES: usize = 4;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemies)
        .add_startup_system(spawn_camera)
        .add_system(player_movement)
        .add_system(confine_player)
        .run();
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("sprites\\norm.png"),
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0)
                .with_scale(Vec3::new(0.5, 0.5, 1.0)),
            ..default()
        },
        Player {},
    ));
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::Y;
        }
        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::Y;
        }
        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::X;
        }
        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::X;
        }
        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        transform.translation += direction * time.delta_seconds() * PLAYER_SPEED;
    }
}

fn confine_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let (player_width, player_height) = (
            PLAYER_WIDTH * transform.scale.x,
            PLAYER_HEIGHT * transform.scale.y,
        );
        let (min_x, min_y) = (player_width / 2.0, player_height / 2.0);
        let (max_x, max_y) = (
            window.width() - player_width / 2.0,
            window.height() - player_height / 2.0,
        );
        if transform.translation.x < min_x {
            transform.translation.x = min_x;
        }
        if transform.translation.x > max_x {
            transform.translation.x = max_x;
        }
        if transform.translation.y < min_y {
            transform.translation.y = min_y;
        }
        if transform.translation.y > max_y {
            transform.translation.y = max_y;
        }
    }
}

fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for _ in 0..NUM_ENEMIES {
        let x = rand::random::<f32>() * window.width();
        let y = rand::random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("sprites\\morn.png"),
                transform: Transform::from_xyz(x, y, 0.0).with_scale(Vec3::new(0.5, 0.5, 1.0)),
                ..default()
            },
            Enemy {},
        ));
    }
}
