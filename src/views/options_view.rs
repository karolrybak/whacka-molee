use crate::game_states::{GameContext, Transition, View};
use crate::localization;
use crate::t;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

pub struct OptionsView {
    language_changed_to: Option<String>,
}

impl OptionsView {
    pub fn new(_context: &GameContext) -> Self {
        Self {
            language_changed_to: None,
        }
    }

    fn attempt_language_change(&mut self, lang_code: &str, _context: &GameContext) {
        match localization::set_current_language(lang_code) {
            Ok(_) => {
                info!("Language set to {}", lang_code);
                self.language_changed_to = Some(lang_code.to_string());
            }
            Err(e) => {
                error!("Failed to set language to {}: {:?}", lang_code, e);
                self.language_changed_to = None;
            }
        }
    }
}

impl View for OptionsView {
    fn on_enter(&mut self, _context: &GameContext) {
        info!("Entered Options View.");
        self.language_changed_to = None;
    }

    fn update_and_handle_input(&mut self, _dt: f32, context: &GameContext) -> Transition {
        let mut transition_request = Transition::None;

        let window_width = 350.0;
        let window_height = 300.0;
        let window_x = context.screen_width / 2.0 - window_width / 2.0;
        let window_y = context.screen_height / 2.0 - window_height / 2.0;

        widgets::Window::new(
            hash!(),
            vec2(window_x, window_y),
            vec2(window_width, window_height),
        )
        .label(t!("options-title").as_str())
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, t!("options-language-select").as_str());
            ui.separator();

            if ui.button(None, "English") {
                self.attempt_language_change("en", context);
            }
            if ui.button(None, "Polski") {
                self.attempt_language_change("pl", context);
            }
            if ui.button(None, "Español") {
                self.attempt_language_change("es", context);
            }

            ui.separator();

            if ui.button(None, t!("options-back-button").as_str()) {
                if let Some(new_lang) = self.language_changed_to.take() {
                    transition_request = Transition::LanguageChanged(new_lang);
                } else {
                    transition_request = Transition::Pop;
                }
            }
        });

        transition_request
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
        let logo_target_height = context.screen_height * 0.25;
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

        draw_rectangle(
            0.0,
            0.0,
            context.screen_width,
            context.screen_height,
            Color::from_rgba(0, 0, 0, 150),
        );
    }
}
