// src/main.rs
// version:0.0.2
// ----START OF FILE----
use bevy::prelude::*;

mod game_states;
mod localization;
mod text_generator;
mod ui;

use crate::ui::main_menu::MainMenuPlugin;
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
        .add_plugins(MainMenuPlugin)
        .add_systems(Startup, initial_setup_system)
        .run();
}

fn initial_setup_system(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::MainMenu);
}
// ----END OF FILE----
// src/main.rs
// version:0.0.2