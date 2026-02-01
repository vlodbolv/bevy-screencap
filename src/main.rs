use bevy::prelude::*;

mod animation;
mod plugin;

fn main() {
    App::new()
        // 1. Core Bevy Setup
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy 0.18 Recorder".into(),
                // Use constants defined in plugin.rs
                resolution: (plugin::WIDTH, plugin::HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        // 2. Add the Recorder Infrastructure
        .add_plugins(plugin::RecorderPlugin)
        // 3. Add the Animation Content
        .add_plugins(animation::DemoScenePlugin)
        .run();
}
