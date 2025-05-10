mod asset_manager;
mod game_states;
mod localization;
pub mod physics;

mod views {
    pub mod main_menu_view;
    pub mod match_view;

    pub use main_menu_view::MainMenuView;
    pub use match_view::MatchView;
}

mod game_objects {
    pub mod player;
    pub mod projectile;
    pub mod terrain;
}

use macroquad::prelude::*;

use asset_manager::AssetManager;
use game_states::{GameContext, GameViewType, Transition, View};
use views::MainMenuView;
use views::MatchView;

struct Game {
    game_context: GameContext,
    view_stack: Vec<Box<dyn View>>,
    is_running: bool,
}

impl Game {
    pub async fn new() -> Self {
        localization::init_localization("assets/locales", "en", &["en", "pl", "es", "tlh"]);
        let asset_manager = AssetManager::new("assets").await;
        let initial_view_type = GameViewType::MainMenu;
        let mut game_context = GameContext {
            asset_manager: asset_manager,
            screen_width: screen_width(),
            screen_height: screen_height(),
        };
        let mut initial_view = Self::create_view(initial_view_type.clone(), &game_context);
        initial_view.on_enter(&mut game_context);
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
        }
    }

    pub async fn run_loop(&mut self) {
        while self.is_running {
            if self.view_stack.is_empty() {
                self.is_running = false;
                break;
            }

            let dt = get_frame_time();
            let transition_request = {
                let current_view = self.view_stack.last_mut().unwrap();
                current_view.update_and_handle_input(dt, &self.game_context)
            };

            self.handle_transition(transition_request);

            clear_background(BLACK);
            for view_idx in 0..self.view_stack.len() {
                self.view_stack[view_idx].draw(&self.game_context);
            }

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
                if self.view_stack.is_empty() {
                    self.is_running = false;
                }
            }
            Transition::ExitGame => {
                self.is_running = false;
            }
        }
    }
}

#[macroquad::main("WhackaMolee - Static Assets")]
async fn main() {
    let mut game = Game::new().await;
    game.run_loop().await;
}
