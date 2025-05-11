// src/ui/main_menu.rs
// version:0.0.1
// ----START OF FILE----
use bevy::app::AppExit;
use bevy::prelude::*;

use crate::game_states::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu_ui)
            .add_systems(
                Update,
                (button_interaction_visuals, main_menu_action_system)
                    .run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), cleanup_main_menu_ui);
    }
}

#[derive(Component)]
struct MainMenuUITag;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum MainMenuButtonAction {
    StartGame,
    Options,
    Quit,
}

const NORMAL_BUTTON_BG_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_BUTTON_BG_COLOR: Color = Color::rgb(0.35, 0.35, 0.35);
const PRESSED_BUTTON_BG_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
const BUTTON_TEXT_COLOR: Color = Color::WHITE;
const BUTTON_BORDER_COLOR: Color = Color::BLACK;
const BUTTON_HOVERED_BORDER_COLOR: Color = Color::WHITE;

fn get_button_style() -> Style {
    Style {
        width: Val::Px(250.0),
        height: Val::Px(65.0),
        margin: UiRect::bottom(Val::Px(10.0)), // Margin between buttons
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(2.0)),
        ..default()
    }
}

fn get_button_text_style(_asset_server: &AssetServer) -> TextStyle {
    TextStyle {
        font_size: 30.0,
        color: BUTTON_TEXT_COLOR,
        ..default()
    }
}

fn setup_main_menu_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::FlexStart, // Align container to top
                    align_items: AlignItems::FlexEnd,      // Align container to right
                    flex_direction: FlexDirection::Column, // Main axis for screen
                    ..default()
                },
                ..default()
            },
            MainMenuUITag,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::FlexEnd, // Align buttons to the right if they had varying width
                        padding: UiRect::all(Val::Px(30.0)), // Padding for the button group
                        ..default()
                    },
                    // background_color: Color::rgba(0.0, 0.0, 1.0, 0.1).into(), // Optional: for debugging layout
                    ..default()
                })
                .with_children(|button_container| {
                    spawn_main_menu_button(
                        button_container,
                        "Start Game",
                        MainMenuButtonAction::StartGame,
                        &asset_server,
                    );
                    spawn_main_menu_button(
                        button_container,
                        "Options",
                        MainMenuButtonAction::Options,
                        &asset_server,
                    );
                    spawn_main_menu_button(
                        button_container,
                        "Quit",
                        MainMenuButtonAction::Quit,
                        &asset_server,
                    );
                });
        });
}

fn spawn_main_menu_button(
    parent: &mut ChildBuilder,
    text: &str,
    action: MainMenuButtonAction,
    asset_server: &Res<AssetServer>,
) {
    parent
        .spawn((
            ButtonBundle {
                style: get_button_style(),
                border_color: BorderColor(BUTTON_BORDER_COLOR),
                background_color: NORMAL_BUTTON_BG_COLOR.into(),
                ..default()
            },
            action,
        ))
        .with_children(|btn_parent| {
            btn_parent.spawn(TextBundle::from_section(
                text,
                get_button_text_style(asset_server),
            ));
        });
}

fn button_interaction_visuals(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>, With<MainMenuButtonAction>),
    >,
) {
    for (interaction, mut bg_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = PRESSED_BUTTON_BG_COLOR.into();
                *border_color = BUTTON_HOVERED_BORDER_COLOR.into();
            }
            Interaction::Hovered => {
                *bg_color = HOVERED_BUTTON_BG_COLOR.into();
                *border_color = BUTTON_HOVERED_BORDER_COLOR.into();
            }
            Interaction::None => {
                *bg_color = NORMAL_BUTTON_BG_COLOR.into();
                *border_color = BUTTON_BORDER_COLOR.into();
            }
        }
    }
}

fn main_menu_action_system(
    interaction_query: Query<(&Interaction, &MainMenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_state_next: ResMut<NextState<AppState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MainMenuButtonAction::StartGame => {
                    app_state_next.set(AppState::InGame);
                }
                MainMenuButtonAction::Options => {
                    app_state_next.set(AppState::OptionsMenu);
                }
                MainMenuButtonAction::Quit => {
                    app_exit_events.send(AppExit);
                }
            }
        }
    }
}

fn cleanup_main_menu_ui(mut commands: Commands, query: Query<Entity, With<MainMenuUITag>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
// ----END OF FILE----
// src/ui/main_menu.rs
// version:0.0.1