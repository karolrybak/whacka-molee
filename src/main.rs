// src/main.rs
use bevy::prelude::*;

mod game_states;
mod localization;
mod text_generator;
mod main_menu; // <--- ADD THIS

use game_states::AppState;
use localization::LocalizationPlugin;
use text_generator::TextGeneratorPlugin;
use main_menu::MainMenuPlugin; // <--- ADD THIS

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Whacka Molee - Bevy Edition!".into(),
                resolution: (1280., 720.).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .init_state::<AppState>()
        .add_plugins(LocalizationPlugin)
        .add_plugins(TextGeneratorPlugin)
        .add_plugins(MainMenuPlugin) // <--- ADD THIS
        .add_systems(Startup, initial_setup_system) // Renamed for clarity
        .run();
}

// Renamed to avoid conflict if 'setup' is a common name
fn initial_setup_system(mut app_state: ResMut<NextState<AppState>>) {
    // This correctly transitions to MainMenu after initial setup
    app_state.set(AppState::MainMenu);
}