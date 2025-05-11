use bevy::prelude::*;

#[derive(Resource, Debug, Clone)]
pub struct UiTheme {
    pub font_arbutus: Handle<Font>,
    pub font_lato: Handle<Font>,
    pub button_image_normal: Handle<Image>,
    pub dice_button_image: Handle<Image>,
    pub background_image: Handle<Image>,
    pub logo_image: Handle<Image>,
}

pub fn get_main_text_style(theme: &Res<UiTheme>) -> TextStyle {
    TextStyle {
        font: theme.font_arbutus.clone(),
        font_size: 28.0,
        color: Color::WHITE,
    }
}

pub fn get_label_text_style(theme: &Res<UiTheme>) -> TextStyle {
    TextStyle {
        font: theme.font_lato.clone(),
        font_size: 24.0,
        color: Color::WHITE,
    }
}

pub fn get_title_text_style(theme: &Res<UiTheme>) -> TextStyle {
    TextStyle {
        font: theme.font_arbutus.clone(),
        font_size: 36.0,
        color: Color::WHITE,
    }
}

pub fn get_input_text_style(theme: &Res<UiTheme>) -> TextStyle {
    TextStyle {
        font: theme.font_lato.clone(),
        font_size: 22.0,
        color: Color::BLACK,
    }
}

pub fn get_button_bundle_from_image(theme: &Res<UiTheme>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(384.0 * 0.75),
            height: Val::Px(141.0 * 0.75),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::all(Val::Px(5.0)),
            ..default()
        },
        image: UiImage::new(theme.button_image_normal.clone()),
        background_color: Color::NONE.into(),
        ..default()
    }
}

pub fn get_dice_button_bundle(theme: &Res<UiTheme>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(40.0),
            height: Val::Px(40.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            margin: UiRect::new(Val::Px(10.0), Val::ZERO, Val::ZERO, Val::ZERO),
            ..default()
        },
        image: UiImage::new(theme.dice_button_image.clone()),
        background_color: Color::NONE.into(),
        ..default()
    }
}
