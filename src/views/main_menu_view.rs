// src/views/main_menu_view.rs
// version:0.1.3
// ----START OF FILE----
use crate::game_states::{GameContext, GameViewType, Transition, View};
use crate::t;
use macroquad::prelude::*;
use macroquad::ui::widgets::Button;
use macroquad::ui::{hash, root_ui};


pub struct MainMenuView {
    
}

impl MainMenuView {
    pub fn new(context: &GameContext) -> Self {
        // &GameContext
        let button_style = root_ui()
            .style_builder()
            .font_size(30)
            .text_color(BLACK)
            .color(Color::from_rgba(200, 200, 200, 255))
            .color_hovered(Color::from_rgba(220, 220, 220, 255))
            .color_clicked(Color::from_rgba(180, 180, 180, 255))
            .build();

        let mut skin = root_ui().default_skin().clone();
        skin.button_style = button_style;

        Self {
            
        }
    }
}

impl View for MainMenuView {
    fn on_enter(&mut self, _context: &GameContext) {
        info!("Entered Main Menu View.");
    }

    fn update_and_handle_input(&mut self, _dt: f32, context: &GameContext) -> Transition {
        
        let mut transition:Transition = Transition::None;
        // let button_width = 220.0;
        // let screen_w = context.screen_width;
        // let screen_h = context.screen_height;

        if root_ui().button(None, t!("button-new-game")) {
            println!("pushed");
            info!("'Quit' button clicked.");
            transition = Transition::ExitGame;
         }

        transition
    }

    fn draw(&self, context: &GameContext) {
        
        let bg_tex = context.asset_manager.get_texture("main_menu/background.png");
        draw_texture_ex(
            &bg_tex,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(context.screen_width, context.screen_height)),
                ..Default::default()
            },
        );

        let logo_tex = context.asset_manager.get_texture("main_menu/logo.png");

        let logo_target_height = context.screen_height * 0.25;
        let logo_aspect_ratio = logo_tex.width() / logo_tex.height();
        let logo_h = logo_target_height;
        let logo_w = logo_h * logo_aspect_ratio;

        draw_texture_ex(
            &logo_tex,
            context.screen_width / 2.0 - logo_w / 2.0,
            context.screen_height * 0.15,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(logo_w, logo_h)),
                ..Default::default()
            },
        );
    }
}
// ----END OF FILE----
// src/views/main_menu_view.rs
// version:0.1.3
