use bevy::prelude::*;
use shared::lifecycle::AppLifecyclePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hello 2D".into(),
                resolution: (800.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AppLifecyclePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, animate_sprite))
        .run();
}

#[derive(Component)]
#[require(Transform, Visibility)]
struct Player;

#[derive(Component)]
struct Velocity(Vec2);

#[derive(Component)]
struct AnimationTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2d);

    // Spawn a player sprite
    commands.spawn((
        Player,
        Sprite {
            image: asset_server.load("icon.png"),
            custom_size: Some(Vec2::new(64.0, 64.0)),
            ..default()
        },
        Velocity(Vec2::ZERO),
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));

    // Spawn some decorative text
    commands.spawn((
        Text2d::new("Hello, Bevy 2D!"),
        TextFont {
            font_size: 40.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.2)),
        Transform::from_xyz(0.0, 200.0, 0.0),
    ));
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Player>>,
) {
    let Ok((mut transform, mut velocity)) = query.single_mut() else {
        return;
    };

    let mut direction = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
    }

    let speed = 300.0;
    let acceleration = 1200.0;
    let friction = 0.9;

    if direction != Vec2::ZERO {
        velocity.0 += direction.normalize() * acceleration * time.delta_secs();
        velocity.0 = velocity.0.clamp_length_max(speed);
    } else {
        velocity.0 *= friction;
        if velocity.0.length() < 1.0 {
            velocity.0 = Vec2::ZERO;
        }
    }

    transform.translation.x += velocity.0.x * time.delta_secs();
    transform.translation.y += velocity.0.y * time.delta_secs();
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut Transform, &Velocity), With<Player>>,
) {
    let Ok((mut timer, mut transform, velocity)) = query.single_mut() else {
        return;
    };

    timer.0.tick(time.delta());

    if timer.0.just_finished() {
        // Gentle bobbing animation based on movement
        let speed = velocity.0.length();
        if speed > 10.0 {
            let bob = (time.elapsed_secs() * 12.0).sin() * 0.05;
            transform.scale = Vec3::new(1.0 + bob, 1.0 - bob, 1.0);
        } else {
            transform.scale = Vec3::ONE;
        }
    }
}
