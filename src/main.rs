use bevy::prelude::*;

mod asset_loading;
mod game_states;
mod localization;
mod main_menu;
mod options_menu;

mod text_generator;
mod ui_styles;

use asset_loading::AssetLoadingPlugin;
use game_states::AppState;
use localization::LocalizationPlugin;
use main_menu::MainMenuPlugin;
use options_menu::OptionsMenuPlugin;

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
        .add_plugins(AssetLoadingPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(OptionsMenuPlugin)
        .run();
}
