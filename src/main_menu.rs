// src/main_menu.rs

use bevy::app::AppExit; // For the Quit button
use bevy::prelude::*;

use crate::game_states::AppState; // Make sure this path is correct for your project

// Plugin for the main menu
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Systems to run when entering the MainMenu state
            .add_systems(OnEnter(AppState::MainMenu), setup_main_menu_ui)
            // Systems to run continuously while in the MainMenu state (for button interactions)
            .add_systems(
                Update,
                (button_interaction_visuals, main_menu_action_system)
                    .run_if(in_state(AppState::MainMenu)),
            )
            // System to run when exiting the MainMenu state
            .add_systems(OnExit(AppState::MainMenu), cleanup_main_menu_ui);
    }
}

// Component to tag entities specifically on the main menu screen for easy cleanup
#[derive(Component)]
struct MainMenuScreenTag;

// Enum to define actions for the buttons in the main menu
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum MainMenuButtonAction {
    Play,
    Options,
    Quit,
}

// --- UI constants ---
const NORMAL_BUTTON_BG_COLOR: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_BUTTON_BG_COLOR: Color = Color::rgb(0.35, 0.35, 0.35);
const PRESSED_BUTTON_BG_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);
const BUTTON_TEXT_COLOR: Color = Color::WHITE;
const BUTTON_BORDER_COLOR: Color = Color::BLACK;
const BUTTON_HOVERED_BORDER_COLOR: Color = Color::WHITE;

// Style for the buttons
fn get_button_style() -> Style {
    Style {
        width: Val::Px(280.0),
        height: Val::Px(70.0),
        margin: UiRect::all(Val::Px(10.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(3.0)),
        ..default()
    }
}

// Style for the text within buttons
fn get_button_text_style(asset_server: &AssetServer) -> TextStyle {
    TextStyle {
        // You can load a custom font here if you have one in your assets folder
        // font: asset_server.load("fonts/your_cool_font.ttf"),
        font_size: 36.0,
        color: BUTTON_TEXT_COLOR,
        ..default()
    }
}

// --- System to set up the Main Menu UI ---
fn setup_main_menu_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Setting up Main Menu UI.");

    // Root node for the menu UI, centered on the screen
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(15.0), // Gap between title and first button, and between buttons
                    ..default()
                },
                // You can set a background color for the whole menu screen if desired
                // background_color: BackgroundColor(Color::BLACK.with_a(0.5)),
                ..default()
            },
            MainMenuScreenTag, // Tag for easy cleanup
        ))
        .with_children(|parent| {
            // Game Title
            parent.spawn(TextBundle::from_section(
                "Whacka Molee", // Your game title
                TextStyle {
                    // font: asset_server.load("fonts/your_title_font.ttf"), // Placeholder for title font
                    font_size: 80.0,
                    color: Color::WHITE,
                    ..default()
                },
            )
            .with_style(Style {
                margin: UiRect::bottom(Val::Px(60.0)), // Space below the title
                ..default()
            }));

            // --- Buttons ---
            // New Game Button
            spawn_main_menu_button(parent, "New Game", MainMenuButtonAction::Play, &asset_server);

            // Options Button
            spawn_main_menu_button(parent, "Options", MainMenuButtonAction::Options, &asset_server);

            // Quit Button
            spawn_main_menu_button(parent, "Quit", MainMenuButtonAction::Quit, &asset_server);
        });
}

// Helper function to spawn a consistent menu button
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
            action, // Attach the action enum to this button entity
        ))
        .with_children(|btn_parent| {
            btn_parent.spawn(TextBundle::from_section(
                text,
                get_button_text_style(asset_server),
            ));
        });
}

// --- System for Button Visual Interactions (hover, click feedback) ---
fn button_interaction_visuals(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<Button>, With<MainMenuButtonAction>), // Ensure we only act on our menu buttons
    >,
) {
    for (interaction, mut bg_color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = PRESSED_BUTTON_BG_COLOR.into();
                *border_color = BUTTON_HOVERED_BORDER_COLOR.into(); // Keep border highlighted when pressed
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

// --- System to handle menu actions when buttons are pressed ---
fn main_menu_action_system(
    interaction_query: Query<(&Interaction, &MainMenuButtonAction), (Changed<Interaction>, With<Button>)>,
    mut app_state_next: ResMut<NextState<AppState>>,
    mut app_exit_events: EventWriter<AppExit>, // Used to signal the app to close
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MainMenuButtonAction::Play => {
                    info!("'New Game' button pressed. Transitioning to AppState::InGame.");
                    // Assuming you want to go directly to InGame.
                    // If you have asset loading specifically for the game,
                    // you might transition to a AppState::LoadingGameAssets first.
                    app_state_next.set(AppState::InGame);
                }
                MainMenuButtonAction::Options => {
                    info!("'Options' button pressed. Transitioning to AppState::OptionsMenu.");
                    app_state_next.set(AppState::OptionsMenu);
                }
                MainMenuButtonAction::Quit => {
                    info!("'Quit' button pressed. Sending AppExit event.");
                    app_exit_events.send(AppExit); // This will close the application
                }
            }
        }
    }
}

// --- System to clean up the Main Menu UI when the state is exited ---
fn cleanup_main_menu_ui(mut commands: Commands, query: Query<Entity, With<MainMenuScreenTag>>) {
    info!("Cleaning up Main Menu UI.");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive(); // Despawn the root UI node and all its children
    }
}