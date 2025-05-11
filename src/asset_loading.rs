use crate::ui_styles::UiTheme;
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct MainMenuSpecificAssets {}

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MainMenuSpecificAssets>()
            .init_resource::<UiTheme>()
            .add_systems(Startup, setup_core_assets);
    }
}

fn setup_core_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(UiTheme {
        font_arbutus: asset_server.load("fonts/Arbutus-Regular.ttf"),
        font_lato: asset_server.load("fonts/Lato-Regular.ttf"),
        button_image_normal: asset_server.load("textures/btn_normal.png"),
        dice_button_image: asset_server.load("textures/button_dice.png"),
        background_image: asset_server.load("textures/main_menu_background.png"),
        logo_image: asset_server.load("textures/logo.png"),
    });

    info!("Core UI assets (fonts, common textures) loading initiated.");
}
