mod asset_manager;
mod game_states;
mod localization;
pub mod physics;
mod ui_styles;

mod views {
    pub mod main_menu_view;
    pub mod match_view;
    pub mod options_view;

    pub use main_menu_view::MainMenuView;
    pub use match_view::MatchView;
    pub use options_view::OptionsView;
}

mod game_objects {
    pub mod player;
    pub mod projectile;
    pub mod terrain;
}

mod text_generator;

use macroquad::prelude::*;

use asset_manager::AssetManager;
use game_states::{GameContext, GameViewType, Transition, View};
use text_generator::WhackaMoleeGenerator;
use ui_styles::create_global_skin;
use views::{MainMenuView, MatchView, OptionsView};
use macroquad::ui::root_ui;
struct Game {
    game_context: GameContext,
    view_stack: Vec<Box<dyn View>>,
    is_running: bool,
}

impl Game {
    pub async fn new() -> Self {
        let initial_lang_id_str = "en";
        localization::init_localization(
            "assets/locales",
            initial_lang_id_str,
            &["en", "pl", "es", "tlh"],
        );

        let asset_manager = AssetManager::new("assets").await;

        let text_generator = WhackaMoleeGenerator::new("assets/locales", initial_lang_id_str)
            .expect("Failed to create WhackaMoleeGenerator");

        let ui_skin = create_global_skin();

        let game_context = GameContext {
            asset_manager,
            screen_width: screen_width(),
            screen_height: screen_height(),
            text_generator,
            ui_skin,
        };

        let initial_view_type = GameViewType::MainMenu;
        let mut initial_view = Self::create_view(initial_view_type.clone(), &game_context);
        initial_view.on_enter(&game_context);

        Self {
            game_context,
            view_stack: vec![initial_view],
            is_running: true,
        }
    }

    fn create_view(view_type: GameViewType, context: &GameContext) -> Box<dyn View> {
        match view_type {
            GameViewType::MainMenu => Box::new(MainMenuView::new(context)),
            GameViewType::Match => Box::new(MatchView::new(context)),
            GameViewType::Options => Box::new(OptionsView::new(context)),
        }
    }

    pub async fn run_loop(&mut self) {
        while self.is_running {
            if self.view_stack.is_empty() {
                self.is_running = false;
                break;
            }

            let dt = get_frame_time();

            root_ui().push_skin(&self.game_context.ui_skin);

            let transition_request = {
                let current_view = self.view_stack.last_mut().unwrap();
                current_view.update_and_handle_input(dt, &self.game_context)
            };

            self.handle_transition(transition_request);

            clear_background(BLACK);
            let game_context_ref = &self.game_context;

            for view_idx in 0..self.view_stack.len() {
                self.view_stack[view_idx].draw(game_context_ref);
            }
            root_ui().pop_skin();

            next_frame().await;
        }
    }

    fn handle_transition(&mut self, request: Transition) {
        match request {
            Transition::None => {}
            Transition::Push(new_view_type) => {
                if let Some(current_view) = self.view_stack.last_mut() {
                    current_view.on_exit(&self.game_context);
                }
                let mut new_view = Self::create_view(new_view_type, &self.game_context);
                new_view.on_enter(&self.game_context);
                self.view_stack.push(new_view);
            }
            Transition::Pop => {
                if let Some(mut old_view) = self.view_stack.pop() {
                    old_view.on_exit(&self.game_context);
                    if let Some(resumed_view) = self.view_stack.last_mut() {
                        resumed_view.on_resume_with_data(&self.game_context, Box::new(()));
                    }
                }
                if self.view_stack.is_empty() {
                    self.is_running = false;
                }
            }
            Transition::LanguageChanged(new_lang_code) => {
                if let Some(mut old_view) = self.view_stack.pop() {
                    old_view.on_exit(&self.game_context);
                }
                match WhackaMoleeGenerator::new("assets/locales", &new_lang_code) {
                    Ok(new_generator) => {
                        self.game_context.text_generator = new_generator;
                        info!("Text generator reloaded for language: {}", new_lang_code);
                    }
                    Err(e) => {
                        error!(
                            "Failed to recreate WhackaMoleeGenerator for language {}: {:?}",
                            new_lang_code, e
                        );
                    }
                }

                if self.view_stack.is_empty() {
                    self.is_running = false;
                } else {
                    if let Some(resumed_view) = self.view_stack.last_mut() {
                        resumed_view.on_resume_with_data(&self.game_context, Box::new(()));
                    }
                }
            }
            Transition::PopWithResult(data) => {
                if let Some(mut old_view) = self.view_stack.pop() {
                    old_view.on_exit(&self.game_context);
                }
                if let Some(resumed_view) = self.view_stack.last_mut() {
                    resumed_view.on_resume_with_data(&self.game_context, data);
                }
                if self.view_stack.is_empty() {
                    self.is_running = false;
                }
            }
            Transition::Switch(new_view_type) => {
                if let Some(mut old_view) = self.view_stack.pop() {
                    old_view.on_exit(&self.game_context);
                }
                let mut new_view = Self::create_view(new_view_type, &self.game_context);
                new_view.on_enter(&self.game_context);
                self.view_stack.push(new_view);
            }
            Transition::ExitGame => {
                self.is_running = false;
            }
        }
    }
}

#[macroquad::main("WhackaMolee")]
async fn main() {
    let mut game = Game::new().await;
    game.run_loop().await;
}
