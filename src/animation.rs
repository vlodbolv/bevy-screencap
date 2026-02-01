use crate::plugin::{self, RecordingState};
use bevy::prelude::*;

pub struct DemoScenePlugin;

impl Plugin for DemoScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene)
            .add_systems(Update, (animate_cube, animate_camera));
    }
}

#[derive(Component)]
struct AnimatedCube;

#[derive(Component)]
struct OrbitCamera {
    radius: f32,
    speed: f32,
    angle: f32,
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn((
        Camera3d::default(),
        OrbitCamera {
            radius: 10.0,
            speed: 0.5,
            angle: 0.0,
        },
        Transform::from_xyz(0.0, 6.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.1, 0.6, 1.0),
            metallic: 0.8,
            perceptual_roughness: 0.2,
            ..default()
        })),
        AnimatedCube,
    ));

    // Light
    commands.spawn((
        PointLight {
            intensity: 5_000_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn animate_cube(
    mut q: Query<&mut Transform, With<AnimatedCube>>,
    time: Res<Time>,
    state: Res<RecordingState>,
) {
    // Use helper from plugin.rs to determine if we use Fixed Time (0.033s) or Real Time
    let dt = plugin::get_time_step(state.is_recording, time.delta_secs());
    let elapsed =
        plugin::get_elapsed_time(state.is_recording, state.frame_count, time.elapsed_secs());

    for mut t in &mut q {
        t.rotate_y(dt * 1.5);
        t.translation.y = (elapsed * 2.0).sin() * 0.5;
    }
}

fn animate_camera(
    mut q: Query<(&mut Transform, &mut OrbitCamera)>,
    time: Res<Time>,
    state: Res<RecordingState>,
) {
    let dt = plugin::get_time_step(state.is_recording, time.delta_secs());

    for (mut t, mut orbit) in &mut q {
        orbit.angle += dt * orbit.speed;
        t.translation = Vec3::new(
            orbit.angle.cos() * orbit.radius,
            6.0,
            orbit.angle.sin() * orbit.radius,
        );
        t.look_at(Vec3::ZERO, Vec3::Y);
    }
}
