// karolrybak/whacka-molee/whacka-molee-351ea95a23ebe18bf0ffbcd3437412c5de79bebd/src/views/main_menu_view.rs
// version:0.2.6
// ----START OF FILE----
use crate::game_states::{GameContext, GameViewType, Transition, View};
use crate::t;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Skin};

pub struct MainMenuView {
    seed_input_text: String,
}

impl MainMenuView {
    pub fn new(_context: &GameContext) -> Self {
        Self {
            seed_input_text: String::from("default_seed"),
        }
    }
}

impl View for MainMenuView {
    fn on_enter(&mut self, _context: &GameContext) {
        info!("Entered Main Menu View.");
    }

    fn update_and_handle_input(&mut self, _dt: f32, context: &GameContext) -> Transition {
        let mut transition: Transition = Transition::None;
        
        let panel_left_x = context.screen_width * 0.05;
        let panel_top_y = context.screen_height * 0.4;
        let input_width = context.screen_width * 0.3;
        let input_height = 40.0;
        let dice_button_size = 40.0;
        let spacing = 10.0;

        root_ui().label(Some(vec2(panel_left_x, panel_top_y)), t!("main-menu-seed-label").as_str());
        
        // Używamy widgets::InputText dla lepszej kontroli nad pozycją i rozmiarem
        widgets::InputText::new(hash!()) // ID jest pierwszym argumentem
            .label("") // Etykieta może być pusta, jeśli już ją mamy osobno
            .position(vec2(panel_left_x, panel_top_y + 30.0))
            .size(vec2(input_width, input_height))
            .ui(&mut *root_ui(), &mut self.seed_input_text); // Przekazujemy UI i bufor

        let dice_texture = context.asset_manager.get_texture("button_dice.png");
        if widgets::Button::new(dice_texture)
            .position(vec2(panel_left_x + input_width + spacing, panel_top_y + 30.0 + (input_height - dice_button_size) / 2.0))
            .size(vec2(dice_button_size, dice_button_size))
            .ui(&mut *root_ui()) {
            self.seed_input_text = context.text_generator.generate_terrain_name();
            info!("Random Seed button clicked. New seed: {}", self.seed_input_text);
        }

        let btn_common_width = 384.0 * 0.75;
        let btn_common_height = 141.0 * 0.75;
        
        let main_buttons_start_x = context.screen_width * 0.95 - btn_common_width;
        let main_buttons_total_height = (btn_common_height * 3.0) + (spacing * 2.0);
        let main_buttons_start_y = context.screen_height / 2.0 - main_buttons_total_height / 2.0;

        if widgets::Button::new(t!("main-menu-start-game").as_str())
            .position(vec2(main_buttons_start_x, main_buttons_start_y))
            .size(vec2(btn_common_width, btn_common_height))
            .ui(&mut *root_ui()) {
            info!("'Start Game' button clicked.");
            transition = Transition::Switch(GameViewType::Match);
        }

        if widgets::Button::new(t!("main-menu-options").as_str())
            .position(vec2(main_buttons_start_x, main_buttons_start_y + btn_common_height + spacing))
            .size(vec2(btn_common_width, btn_common_height))
            .ui(&mut *root_ui()) {
            info!("'Options' button clicked.");
            transition = Transition::Push(GameViewType::Options);
        }

        if widgets::Button::new(t!("main-menu-quit-game").as_str())
            .position(vec2(main_buttons_start_x, main_buttons_start_y + (btn_common_height + spacing) * 2.0))
            .size(vec2(btn_common_width, btn_common_height))
            .ui(&mut *root_ui()) {
            info!("'Quit Game' button clicked.");
            transition = Transition::ExitGame;
        }
        
        transition
    }

    fn draw(&self, context: &GameContext) {
        let bg_tex = context
            .asset_manager
            .get_texture("main_menu/background.png");
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

        let logo_target_height = context.screen_height * 0.20;
        let logo_aspect_ratio = logo_tex.width() / logo_tex.height();
        let logo_h = logo_target_height;
        let logo_w = logo_h * logo_aspect_ratio;

        draw_texture_ex(
            &logo_tex,
            context.screen_width / 2.0 - logo_w / 2.0,
            context.screen_height * 0.05, 
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(logo_w, logo_h)),
                ..Default::default()
            },
        );

        let panel_left_x = context.screen_width * 0.05;
        let panel_top_y = context.screen_height * 0.4;
        let input_width = context.screen_width * 0.3;
        let input_height = 40.0;
        let dice_button_size = 40.0;
        let spacing = 10.0;
        let preview_x = panel_left_x;
        let preview_y = panel_top_y + 30.0 + input_height + spacing;
        let preview_width = input_width + spacing + dice_button_size;
        let preview_height = context.screen_height * 0.3;
        
        draw_rectangle(preview_x, preview_y, preview_width, preview_height, Color::from_rgba(50,50,50,150));
    }
}
// ----END OF FILE----
// karolrybak/whacka-molee/whacka-molee-351ea95a23ebe18bf0ffbcd3437412c5de79bebd/src/views/main_menu_view.rs
// version:0.2.6