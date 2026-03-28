use bevy::prelude::*;
use shared::lifecycle::AppLifecyclePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Hello 3D".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AppLifecyclePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_shapes, orbit_camera))
        .run();
}

#[derive(Component)]
struct Rotating {
    axis: Vec3,
    speed: f32,
}

#[derive(Component)]
struct OrbitalCamera {
    radius: f32,
    speed: f32,
    height: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            perceptual_roughness: 0.9,
            ..default()
        })),
    ));

    // Central cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.2, 0.2),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Rotating {
            axis: Vec3::Y,
            speed: 1.0,
        },
    ));

    // Orbiting sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.4))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.4, 0.9),
            metallic: 0.8,
            perceptual_roughness: 0.2,
            ..default()
        })),
        Transform::from_xyz(3.0, 1.0, 0.0),
        Rotating {
            axis: Vec3::Y,
            speed: 0.5,
        },
    ));

    // Torus
    commands.spawn((
        Mesh3d(meshes.add(Torus::new(0.3, 0.8))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.9, 0.7, 0.1),
            metallic: 0.5,
            ..default()
        })),
        Transform::from_xyz(-2.0, 1.2, -1.5),
        Rotating {
            axis: Vec3::new(1.0, 1.0, 0.0).normalize(),
            speed: 1.5,
        },
    ));

    // Capsule
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::new(0.3, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.6, 0.2, 0.8),
            ..default()
        })),
        Transform::from_xyz(2.0, 0.8, -2.5),
        Rotating {
            axis: Vec3::Z,
            speed: 2.0,
        },
    ));

    // Directional light (sun)
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.8, 0.4, 0.0)),
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.6, 0.7, 1.0),
        brightness: 200.0,
    });

    // Orbiting camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 4.0, 5.0).looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
        OrbitalCamera {
            radius: 8.0,
            speed: 0.3,
            height: 4.0,
        },
    ));
}

fn rotate_shapes(time: Res<Time>, mut query: Query<(&mut Transform, &Rotating)>) {
    for (mut transform, rotating) in &mut query {
        transform.rotate(Quat::from_axis_angle(
            rotating.axis,
            rotating.speed * time.delta_secs(),
        ));
    }
}

fn orbit_camera(time: Res<Time>, mut query: Query<(&mut Transform, &OrbitalCamera)>) {
    let Ok((mut transform, camera)) = query.single_mut() else {
        return;
    };

    let elapsed = time.elapsed_secs();
    let angle = elapsed * camera.speed;

    transform.translation = Vec3::new(
        angle.cos() * camera.radius,
        camera.height,
        angle.sin() * camera.radius,
    );
    transform.look_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y);
}
