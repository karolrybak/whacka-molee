use crate::game_states::AppState;
use crate::localization::{CurrentLang, FluentBundleResource, LanguageChangeRequest, t};
use bevy::prelude::*;

use crate::ui_styles::{self, UiTheme};

pub struct OptionsMenuPlugin;

impl Plugin for OptionsMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::OptionsMenu), setup_options_menu)
            .add_systems(
                Update,
                handle_options_menu_buttons.run_if(in_state(AppState::OptionsMenu)),
            )
            .add_systems(OnExit(AppState::OptionsMenu), cleanup_options_menu);
    }
}

#[derive(Component)]
struct OptionsMenuUIRoot;
#[derive(Component)]
struct EnglishButton;
#[derive(Component)]
struct PolishButton;
#[derive(Component)]
struct SpanishButton;
#[derive(Component)]
struct BackButton;

fn setup_options_menu(
    mut commands: Commands,
    theme: Res<UiTheme>,
    bundle_res: Res<FluentBundleResource>,
) {
    info!("Entering OptionsMenu state. Setting up UI.");

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                background_color: Color::srgba(0.1, 0.1, 0.1, 0.85).into(),
                z_index: ZIndex::Global(10),
                ..default()
            },
            OptionsMenuUIRoot,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(400.0),
                        padding: UiRect::all(Val::Px(20.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(15.0),

                        ..default()
                    },
                    ..default()
                })
                .with_children(|options_panel| {
                    options_panel.spawn(
                        TextBundle::from_section(
                            t!(bundle_res, "options-title"),
                            ui_styles::get_title_text_style(&theme),
                        )
                        .with_style(Style {
                            margin: UiRect::bottom(Val::Px(20.0)),
                            ..default()
                        }),
                    );

                    options_panel.spawn(
                        TextBundle::from_section(
                            t!(bundle_res, "options-language-select"),
                            ui_styles::get_label_text_style(&theme),
                        )
                        .with_style(Style {
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        }),
                    );

                    options_panel
                        .spawn((
                            ui_styles::get_button_bundle_from_image(&theme),
                            EnglishButton,
                        ))
                        .with_children(|button_content| {
                            button_content.spawn(TextBundle::from_section(
                                "English",
                                ui_styles::get_main_text_style(&theme),
                            ));
                        });

                    options_panel
                        .spawn((
                            ui_styles::get_button_bundle_from_image(&theme),
                            PolishButton,
                        ))
                        .with_children(|button_content| {
                            button_content.spawn(TextBundle::from_section(
                                "Polski",
                                ui_styles::get_main_text_style(&theme),
                            ));
                        });

                    options_panel
                        .spawn((
                            ui_styles::get_button_bundle_from_image(&theme),
                            SpanishButton,
                        ))
                        .with_children(|button_content| {
                            button_content.spawn(TextBundle::from_section(
                                "Español",
                                ui_styles::get_main_text_style(&theme),
                            ));
                        });

                    options_panel
                        .spawn((
                            ui_styles::get_button_bundle_from_image(&theme).with_style(Style {
                                margin: UiRect::top(Val::Px(30.0)),
                                ..default()
                            }),
                            BackButton,
                        ))
                        .with_children(|button_content| {
                            button_content.spawn(TextBundle::from_section(
                                t!(bundle_res, "options-back-button"),
                                ui_styles::get_main_text_style(&theme),
                            ));
                        });
                });
        });
}

fn handle_options_menu_buttons(
    mut next_state: ResMut<NextState<AppState>>,
    mut lang_change_event_writer: EventWriter<LanguageChangeRequest>,
    q_english_interaction: Query<
        &Interaction,
        (Changed<Interaction>, With<EnglishButton>, With<Button>),
    >,
    q_polish_interaction: Query<
        &Interaction,
        (Changed<Interaction>, With<PolishButton>, With<Button>),
    >,
    q_spanish_interaction: Query<
        &Interaction,
        (Changed<Interaction>, With<SpanishButton>, With<Button>),
    >,
    q_back_interaction: Query<&Interaction, (Changed<Interaction>, With<BackButton>, With<Button>)>,
) {
    if let Ok(Interaction::Pressed) = q_english_interaction.get_single() {
        info!("Requesting language change to: en");
        lang_change_event_writer.send(LanguageChangeRequest("en".to_string()));
    }
    if let Ok(Interaction::Pressed) = q_polish_interaction.get_single() {
        info!("Requesting language change to: pl");
        lang_change_event_writer.send(LanguageChangeRequest("pl".to_string()));
    }
    if let Ok(Interaction::Pressed) = q_spanish_interaction.get_single() {
        info!("Requesting language change to: es");
        lang_change_event_writer.send(LanguageChangeRequest("es".to_string()));
    }
    if let Ok(Interaction::Pressed) = q_back_interaction.get_single() {
        info!("Back button pressed");
        next_state.set(AppState::MainMenu);
    }
}

fn cleanup_options_menu(
    mut commands: Commands,
    root_query: Query<Entity, With<OptionsMenuUIRoot>>,
) {
    info!("Exiting OptionsMenu state. Cleaning up UI.");
    if let Ok(root_entity) = root_query.get_single() {
        commands.entity(root_entity).despawn_recursive();
    }
}
