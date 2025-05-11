// src/main.rs
// version:0.0.2
// ----START OF FILE----
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

fn setup(mut app_state_next_state: ResMut<NextState<AppState>>) {
    app_state_next_state.set(AppState::MainMenu);
}
// ----END OF FILE----
// src/main.rs
// version:0.0.2