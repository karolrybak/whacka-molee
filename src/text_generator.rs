use bevy::prelude::*;
use rand::seq::IndexedRandom;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use serde_json; 
use std::{collections::HashMap, fs, path::PathBuf};
use crate::localization::{CurrentLang, LanguageChangeRequest, LocalizationSystemSet};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Dictionaries {
    #[serde(flatten)]
    dictionaries: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Templates {
    #[serde(rename = "TAGLINE_TEMPLATES")]
    tagline_templates: Vec<String>,
    #[serde(rename = "TEAM_NAME_TEMPLATES")]
    team_name_templates: Vec<String>,
    #[serde(rename = "TERRAIN_NAME_TEMPLATES")]
    terrain_name_templates: Vec<String>,
}

#[derive(Resource, Debug, Clone)]
pub struct WhackaMoleeGenerator {
    dictionaries: Dictionaries,
    templates: Templates,
}

impl WhackaMoleeGenerator {
    pub fn new(
        base_locales_path_str: &str,
        current_lang_id_str: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        info!(
            "Initializing WhackaMoleeGenerator for lang: {} from path: {}",
            current_lang_id_str, base_locales_path_str
        );

        let lang_path = PathBuf::from(base_locales_path_str)
            .join(current_lang_id_str.to_lowercase());

        let dictionaries_path = lang_path.join("dictionaries.json");
        let templates_path = lang_path.join("templates.json");

        let dictionaries_content = fs::read_to_string(dictionaries_path)?;
        let dictionaries: Dictionaries = serde_json::from_str(&dictionaries_content)?;

        let templates_content = fs::read_to_string(templates_path)?;
        let templates: Templates = serde_json::from_str(&templates_content)?;


        Ok(Self {
            dictionaries,
            templates,
        })
    }

    fn get_random_from_dict(&self, dict_name: &str) -> Option<String> {
        self.dictionaries
            .dictionaries
            .get(dict_name)
            .and_then(|items| items.choose(&mut rand::rng()).cloned())
    }

    fn pluralize(&self, word: &str) -> String {
        if word.ends_with('y') && !["ay", "ey", "iy", "oy", "uy"].iter().any(|s| word.ends_with(s)) {
            format!("{}ies", &word[0..word.len() - 1])
        } else if word.ends_with("ch")
            || word.ends_with('s')
            || word.ends_with("sh")
            || word.ends_with('x')
            || word.ends_with('z')
        {
            format!("{}es", word)
        } else {
            format!("{}s", word)
        }
    }

    fn process_template(&self, template: &str) -> String {
        let re = Regex::new(r"([A-Z_]+)(_PLURAL)?").unwrap();
        let mut result = template.to_string();
        
        for _ in 0..3 {
            let prev_result = result.clone();
            result = re
                .replace_all(&result, |caps: &Captures| {
                    let dict_name = caps.get(1).unwrap().as_str();
                    let is_plural = caps.get(2).is_some();

                    if let Some(word) = self.get_random_from_dict(dict_name) {
                        if is_plural {
                            self.pluralize(&word)
                        } else {
                            word
                        }
                    } else {
                        warn!("TextGen: Dictionary key {} not found for template.", dict_name);
                        caps.get(0).unwrap().as_str().to_string()
                    }
                })
                .to_string();
            if result == prev_result {
                break;
            }
        }
        result
    }

    pub fn generate_tagline(&self) -> String {
        if let Some(template) = self.templates.tagline_templates.choose(&mut rand::rng()) {
            self.process_template(template)
        } else {
            warn!("TextGen: No tagline templates available.");
            "ERR_NO_TAGLINE_TEMPLATES".to_string()
        }
    }

    pub fn generate_team_name(&self) -> String {
        if let Some(template) = self.templates.team_name_templates.choose(&mut rand::rng()) {
            self.process_template(template)
        } else {
            warn!("TextGen: No team name templates available.");
            "ERR_NO_TEAM_NAME_TEMPLATES".to_string()
        }
    }

    pub fn generate_terrain_name(&self) -> String {
        if let Some(template) = self.templates.terrain_name_templates.choose(&mut rand::rng()) {
            self.process_template(template)
        } else {
            warn!("TextGen: No terrain name templates available.");
            "ERR_NO_TERRAIN_NAME_TEMPLATES".to_string()
        }
    }
}

pub struct TextGeneratorPlugin;

impl Plugin for TextGeneratorPlugin {
    fn build(&self, app: &mut App) {
        let initial_lang_id_str = app.world().get_resource::<CurrentLang>()
            .map_or_else(
                || {
                    warn!("TextGeneratorPlugin: CurrentLang resource not found during setup, defaulting to 'en'. Ensure LocalizationPlugin runs before TextGeneratorPlugin.");
                    "en".to_string()
                }, 
                |lang| lang.0.to_string()
            );

        match WhackaMoleeGenerator::new("assets/locales", &initial_lang_id_str) {
            Ok(generator) => {
                app.insert_resource(generator);
                info!("WhackaMoleeGenerator initialized for language: {}", initial_lang_id_str);
            }
            Err(e) => {
                error!("Failed to initialize WhackaMoleeGenerator: {:?}", e);
            }
        }
        
        app.add_systems(Update, reload_text_generator_on_lang_change.after(LocalizationSystemSet::LanguageProcessing));
    }
}

fn reload_text_generator_on_lang_change(
    current_lang: Res<CurrentLang>,
    text_generator_res: Option<ResMut<WhackaMoleeGenerator>>,
    mut lang_changed_event: EventReader<LanguageChangeRequest>, 
    mut commands: Commands,
) {
    
    let mut needs_reload = false;
    if !lang_changed_event.is_empty() {
        needs_reload = true;
        lang_changed_event.clear(); 
    } else if text_generator_res.is_none() && current_lang.is_added() {
        needs_reload = true;
    } else if current_lang.is_changed() { 
        needs_reload = true;
    }


    if needs_reload {
        let lang_code = current_lang.0.to_string();
        info!("Attempting to reload/initialize WhackaMoleeGenerator for language: {}", lang_code);
        match WhackaMoleeGenerator::new("assets/locales", &lang_code) {
            Ok(new_gen) => {
                if let Some(mut generator_instance) = text_generator_res {
                    *generator_instance = new_gen;
                    info!("WhackaMoleeGenerator reloaded for language: {}", lang_code);
                } else {
                    commands.insert_resource(new_gen);
                    info!("WhackaMoleeGenerator initialized (on demand) for language: {}", lang_code);
                }
            },
            Err(e) => error!("Failed to reload/initialize WhackaMoleeGenerator for {}: {:?}", lang_code, e),
        }
    }
}


