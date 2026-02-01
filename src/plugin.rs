use bevy::{
    prelude::*,
    render::view::screenshot::{save_to_disk, Screenshot},
    window::PrimaryWindow,
};
use std::collections::VecDeque;

// Constants exposed to the rest of the app
pub const WIDTH: u32 = 1024;
pub const HEIGHT: u32 = 768;
pub const RECORDING_FPS: f64 = 30.0;

#[derive(Resource)]
pub struct RecordingState {
    pub is_recording: bool,
    pub frame_count: u32,
    pub session_dir: String,
}

#[derive(Resource)]
pub struct EnvironmentInfo {
    pub name: String,
}

pub struct RecorderPlugin;

impl Plugin for RecorderPlugin {
    fn build(&self, app: &mut App) {
        let env_name = detect_environment();

        app.insert_resource(EnvironmentInfo { name: env_name })
            .insert_resource(RecordingState {
                is_recording: false,
                frame_count: 0,
                session_dir: String::new(),
            })
            .add_systems(Startup, setup_recorder_ui)
            .add_systems(
                Update,
                (manage_input_system, update_fps_display, save_frame_system).chain(),
            );
    }
}

// --- Helpers for Deterministic Animation ---

pub fn get_time_step(is_recording: bool, real_delta: f32) -> f32 {
    if is_recording {
        1.0 / RECORDING_FPS as f32
    } else {
        real_delta
    }
}

pub fn get_elapsed_time(is_recording: bool, frame_count: u32, real_elapsed: f32) -> f32 {
    if is_recording {
        frame_count as f32 * (1.0 / RECORDING_FPS as f32)
    } else {
        real_elapsed
    }
}

fn detect_environment() -> String {
    if std::env::var("CONTAINER_ID").is_ok() {
        "Distrobox".into()
    } else {
        "Native".into()
    }
}

// --- Internal Components ---

#[derive(Component)]
struct FpsCounter {
    samples: VecDeque<f32>,
    last_update: f32,
}

#[derive(Component)]
struct RecordingIndicator;

// --- Systems ---

fn setup_recorder_ui(mut commands: Commands, env_info: Res<EnvironmentInfo>) {
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
            println!("Stopped recording. Captured {} frames.", state.frame_count);
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

            if let Err(e) = std::fs::create_dir_all(&state.session_dir) {
                eprintln!("Failed to create dir: {}", e);
                state.is_recording = false;
                return;
            }

            println!("Recording STARTED to {}/", state.session_dir);
            for mut text in &mut indicator {
                text.0 = "● REC".to_string();
            }
        }
    }
}

fn save_frame_system(
    mut commands: Commands,
    mut state: ResMut<RecordingState>,
    main_window: Query<Entity, With<PrimaryWindow>>,
) {
    if state.is_recording {
        // Bevy 0.18: Use .single() which returns a Result
        if main_window.single().is_ok() {
            let path = format!("{}/frame_{:05}.png", state.session_dir, state.frame_count);

            // Spawn Screenshot command with Observer
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
