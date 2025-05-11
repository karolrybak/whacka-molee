use bevy::prelude::*;

mod game_states;
mod localization;
mod text_generator;
use game_states::AppState;
use localization::LocalizationPlugin;
use text_generator::TextGeneratorPlugin;

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
        .add_systems(Startup, setup)
        .run();
}

// Initialize the game and transition to MainMenu once assets are loaded
fn setup(mut app_state: ResMut<NextState<AppState>>) {
    // In a real game, you would load assets here before transitioning
    // For now, we'll just transition directly to the main menu
    app_state.set(AppState::MainMenu);
}