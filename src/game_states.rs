use crate::asset_manager::AssetManager;
use std::any::Any;

pub struct GameContext {
    pub asset_manager: AssetManager,
    pub screen_width: f32,
    pub screen_height: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GameViewType {
    MainMenu,
    Match,
}

pub enum Transition {
    None,
    Push(GameViewType),
    Pop,
    PopWithResult(Box<dyn Any>),
    Switch(GameViewType),
    ExitGame,
}

impl PartialEq for Transition {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Transition::None, Transition::None) => true,
            (Transition::Push(a), Transition::Push(b)) => a == b,
            (Transition::Pop, Transition::Pop) => true,
            (Transition::PopWithResult(_), Transition::PopWithResult(_)) => true,
            (Transition::Switch(a), Transition::Switch(b)) => a == b,
            (Transition::ExitGame, Transition::ExitGame) => true,
            _ => false,
        }
    }
}

pub trait View {
    fn on_enter(&mut self, _context: &GameContext) {}
    fn on_exit(&mut self, _context: &GameContext) {}
    fn on_resume_with_data(&mut self, _context: &GameContext, _data: Box<dyn Any>) {}
    fn update_and_handle_input(&mut self, dt: f32, context: &GameContext) -> Transition;
    fn draw(&self, context: &GameContext);
}
