use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    LoadingAssets,
    MainMenu,
    OptionsMenu,
    InGame,
}
