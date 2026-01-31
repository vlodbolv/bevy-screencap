use bevy::{
    prelude::*,
    // FIX 1: Import the new Screenshot component and save_to_disk observer helper
    render::view::screenshot::{save_to_disk, Screenshot},
    window::PrimaryWindow,
};
use std::collections::VecDeque;


const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;
const RECORDING_FPS: f64 = 30.0;

fn main() {
    let environment = detect_environment();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("Bevy 0.18 Recorder - {}", environment),
                resolution: (WIDTH, HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(EnvironmentInfo { name: environment })
        .insert_resource(RecordingState {
            is_recording: false,
            frame_count: 0,
            session_dir: String::new(),
        })
        .add_systems(Startup, setup_scene)
        .add_systems(
            Update,
            (
                manage_input_system,
                animate_cube,
                animate_camera,
                update_fps_display,
                save_frame_system,
            )
                .chain(),
        )
        .run();
}

#[derive(Resource)]
struct EnvironmentInfo {
    name: String,
}

#[derive(Resource)]
struct RecordingState {
    is_recording: bool,
    frame_count: u32,
    session_dir: String,
}

#[derive(Component)]
struct AnimatedCube;

#[derive(Component)]
struct OrbitCamera {
    radius: f32,
    speed: f32,
    angle: f32,
}

#[derive(Component)]
struct FpsCounter {
    samples: VecDeque<f32>,
    last_update: f32,
}

#[derive(Component)]
struct RecordingIndicator;

fn detect_environment() -> String {
    if std::env::var("CONTAINER_ID").is_ok() {
        "Distrobox".into()
    } else {
        "Native".into()
    }
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    env_info: Res<EnvironmentInfo>,
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

    // UI
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        })
        .with_children(|ui| {
            ui.spawn(Text::new(format!(
                "System: {}\n[SPACE] Record 30 FPS",
                env_info.name
            )));
            ui.spawn((
                Text::new("FPS: --"),
                TextFont {
                    font_size: 25.0,
                    ..default()
                },
                TextColor(Color::srgb(0.0, 1.0, 0.0)),
                FpsCounter {
                    samples: VecDeque::new(),
                    last_update: 0.0,
                },
            ));
            ui.spawn((
                Text::new(""),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.0, 0.0)),
                RecordingIndicator,
            ));
        });
}

fn manage_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<RecordingState>,
    mut indicator: Query<&mut Text, With<RecordingIndicator>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        if state.is_recording {
            state.is_recording = false;
            println!("Stopped recording.");
            for mut text in &mut indicator {
                text.0 = "".to_string();
            }
        } else {
            state.is_recording = true;
            state.frame_count = 0;
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            state.session_dir = format!("out/rec_{}", timestamp);
            std::fs::create_dir_all(&state.session_dir).unwrap();
            println!("Recording to {}/", state.session_dir);
            for mut text in &mut indicator {
                text.0 = "● REC".to_string();
            }
        }
    }
}

fn animate_cube(
    mut q: Query<&mut Transform, With<AnimatedCube>>,
    time: Res<Time>,
    state: Res<RecordingState>,
) {
    let dt = if state.is_recording {
        1.0 / RECORDING_FPS as f32
    } else {
        time.delta_secs()
    };
    let elapsed = if state.is_recording {
        state.frame_count as f32 * (1.0 / RECORDING_FPS as f32)
    } else {
        time.elapsed_secs()
    };
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
    let dt = if state.is_recording {
        1.0 / RECORDING_FPS as f32
    } else {
        time.delta_secs()
    };
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

// FIX 2: Updated Save System for Bevy 0.18
fn save_frame_system(
    mut commands: Commands,
    mut state: ResMut<RecordingState>,
    main_window: Query<Entity, With<PrimaryWindow>>,
) {
    if state.is_recording {
        // FIX 3: Use .single() instead of .get_single()
        if main_window.single().is_ok() {
            let path = format!("{}/frame_{:05}.png", state.session_dir, state.frame_count);

            // FIX 4: Spawn a Screenshot entity and observe it with save_to_disk
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk(path));

            state.frame_count += 1;
        }
    }
}

fn update_fps_display(time: Res<Time>, mut q: Query<(&mut Text, &mut FpsCounter)>) {
    if time.delta_secs() < 0.001 {
        return;
    }
    for (mut text, mut counter) in &mut q {
        counter.samples.push_back(1.0 / time.delta_secs());
        if counter.samples.len() > 60 {
            counter.samples.pop_front();
        }
        if time.elapsed_secs() - counter.last_update > 0.5 {
            let avg = counter.samples.iter().sum::<f32>() / counter.samples.len() as f32;
            text.0 = format!("FPS: {:.0}", avg);
            counter.last_update = time.elapsed_secs();
        }
    }
}
