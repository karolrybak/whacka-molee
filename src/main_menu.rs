use crate::game_states::AppState;
use crate::localization::{FluentBundleResource, t};
use bevy::prelude::*;

use crate::text_generator::WhackaMoleeGenerator;
use crate::ui_styles::{self, UiTheme};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SeedValue>()
            .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(
                Update,
                main_menu_ui_interaction.run_if(in_state(AppState::MainMenu)),
            )
            .add_systems(OnExit(AppState::MainMenu), cleanup_main_menu);
    }
}

#[derive(Component)]
struct MainMenuUIRoot;
#[derive(Component)]
struct SeedInputTextDisplay;
#[derive(Component)]
struct StartGameButton;
#[derive(Component)]
struct OptionsButton;
#[derive(Component)]
struct QuitButton;
#[derive(Component)]
struct RandomSeedButton;

#[derive(Resource, Default, Debug)]
struct SeedValue(pub String);

fn setup_main_menu(
    mut commands: Commands,
    mut seed_val: ResMut<SeedValue>,
    theme: Res<UiTheme>,
    bundle_res: Res<FluentBundleResource>,
) {
    info!("Entering MainMenu state. Setting up UI.");
    if seed_val.0.is_empty() || seed_val.0 == "default_seed_bevy" || seed_val.0 == "bevy_rocks!" {
        seed_val.0 = "KlingonMenuSeed".to_string();
    }

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
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            },
            MainMenuUIRoot,
        ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                image: UiImage::new(theme.background_image.clone()),
                z_index: ZIndex::Local(-10),
                ..default()
            });

            parent
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::axes(Val::Percent(2.0), Val::Percent(5.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|main_column| {
                    main_column.spawn(ImageBundle {
                        style: Style {
                            width: Val::Auto,
                            height: Val::Percent(20.0),
                            margin: UiRect::bottom(Val::Percent(10.0)),
                            ..default()
                        },
                        image: UiImage::new(theme.logo_image.clone()),
                        ..default()
                    });

                    main_column
                        .spawn(NodeBundle {
                            style: Style {
                                width: Val::Percent(90.0),
                                flex_grow: 1.0,
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::FlexStart,
                                column_gap: Val::Px(20.0),
                                ..default()
                            },
                            ..default()
                        })
                        .with_children(|horizontal_panels_container| {
                            horizontal_panels_container
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::FlexStart,
                                        width: Val::Percent(45.0),
                                        padding: UiRect::all(Val::Px(10.0)),
                                        row_gap: Val::Px(10.0),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|left_panel| {
                                    left_panel.spawn(TextBundle::from_section(
                                        t!(bundle_res, "main-menu-seed-label"),
                                        ui_styles::get_label_text_style(&theme),
                                    ));

                                    left_panel
                                        .spawn(NodeBundle {
                                            style: Style {
                                                flex_direction: FlexDirection::Row,
                                                align_items: AlignItems::Center,
                                                width: Val::Percent(100.0),
                                                column_gap: Val::Px(10.0),
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|input_row| {
                                            input_row
                                                .spawn((
                                                    NodeBundle {
                                                        style: Style {
                                                            width: Val::Percent(80.0),
                                                            padding: UiRect::all(Val::Px(10.0)),
                                                            background_color: Color::srgba(
                                                                0.8, 0.8, 0.8, 0.7,
                                                            )
                                                            .into(),
                                                            ..default()
                                                        },
                                                        ..default()
                                                    },
                                                    SeedInputTextDisplay,
                                                ))
                                                .with_children(|text_container| {
                                                    text_container.spawn(TextBundle::from_section(
                                                        seed_val.0.clone(),
                                                        ui_styles::get_input_text_style(&theme),
                                                    ));
                                                });

                                            input_row.spawn((
                                                ui_styles::get_dice_button_bundle(&theme),
                                                RandomSeedButton,
                                            ));
                                        });

                                    left_panel
                                        .spawn(NodeBundle {
                                            style: Style {
                                                width: Val::Percent(100.0),
                                                height: Val::Px(200.0),
                                                background_color: Color::srgba(0.2, 0.2, 0.2, 0.5)
                                                    .into(),
                                                justify_content: JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                margin: UiRect::top(Val::Px(10.0)),
                                                ..default()
                                            },
                                            ..default()
                                        })
                                        .with_children(|preview_area| {
                                            preview_area.spawn(TextBundle::from_section(
                                                t!(bundle_res, "main-menu-terrain-preview-title"),
                                                ui_styles::get_label_text_style(&theme),
                                            ));
                                        });
                                });

                            horizontal_panels_container
                                .spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        width: Val::Percent(45.0),
                                        row_gap: Val::Px(15.0),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .with_children(|right_panel| {
                                    right_panel
                                        .spawn((
                                            ui_styles::get_button_bundle_from_image(&theme),
                                            StartGameButton,
                                        ))
                                        .with_children(|button_content| {
                                            button_content.spawn(TextBundle::from_section(
                                                t!(bundle_res, "main-menu-start-game"),
                                                ui_styles::get_main_text_style(&theme),
                                            ));
                                        });

                                    right_panel
                                        .spawn((
                                            ui_styles::get_button_bundle_from_image(&theme),
                                            OptionsButton,
                                        ))
                                        .with_children(|button_content| {
                                            button_content.spawn(TextBundle::from_section(
                                                t!(bundle_res, "main-menu-options"),
                                                ui_styles::get_main_text_style(&theme),
                                            ));
                                        });

                                    right_panel
                                        .spawn((
                                            ui_styles::get_button_bundle_from_image(&theme),
                                            QuitButton,
                                        ))
                                        .with_children(|button_content| {
                                            button_content.spawn(TextBundle::from_section(
                                                t!(bundle_res, "main-menu-quit-game"),
                                                ui_styles::get_main_text_style(&theme),
                                            ));
                                        });
                                });
                        });
                });
        });
}

fn main_menu_ui_interaction(
    mut seed_val: ResMut<SeedValue>,
    text_generator: Res<WhackaMoleeGenerator>,
    mut next_state: ResMut<NextState<AppState>>,
    mut app_exit_events: EventWriter<AppExit>,
    q_start_interaction: Query<
        &Interaction,
        (Changed<Interaction>, With<StartGameButton>, With<Button>),
    >,
    q_options_interaction: Query<
        &Interaction,
        (Changed<Interaction>, With<OptionsButton>, With<Button>),
    >,
    q_quit_interaction: Query<&Interaction, (Changed<Interaction>, With<QuitButton>, With<Button>)>,
    q_random_seed_interaction: Query<
        &Interaction,
        (Changed<Interaction>, With<RandomSeedButton>, With<Button>),
    >,
    mut q_seed_text_display: Query<&mut Text, With<SeedInputTextDisplay>>,
) {
    for interaction in q_start_interaction.iter() {
        if *interaction == Interaction::Pressed {
            info!("Start Game button pressed");
            next_state.set(AppState::InGame);
        }
    }
    for interaction in q_options_interaction.iter() {
        if *interaction == Interaction::Pressed {
            info!("Options button pressed");
            next_state.set(AppState::OptionsMenu);
        }
    }
    for interaction in q_quit_interaction.iter() {
        if *interaction == Interaction::Pressed {
            info!("Quit button pressed");
            app_exit_events.send(AppExit);
        }
    }
    for interaction in q_random_seed_interaction.iter() {
        if *interaction == Interaction::Pressed {
            seed_val.0 = text_generator.generate_terrain_name();
            info!("Random Seed button clicked. New seed: {}", seed_val.0);
            for mut text_component in q_seed_text_display.iter_mut() {
                if !text_component.sections.is_empty() {
                    text_component.sections[0].value = seed_val.0.clone();
                }
            }
        }
    }
}

fn cleanup_main_menu(mut commands: Commands, root_query: Query<Entity, With<MainMenuUIRoot>>) {
    info!("Exiting MainMenu state. Cleaning up UI.");
    if let Ok(root_entity) = root_query.get_single() {
        commands.entity(root_entity).despawn_recursive();
    }
}
