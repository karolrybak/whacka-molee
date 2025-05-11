// karolrybak/whacka-molee/whacka-molee-351ea95a23ebe18bf0ffbcd3437412c5de79bebd/src/text_generator.rs
// version:0.2.1
// ----START OF FILE----
use macroquad::prelude::{info, warn};
use macroquad::rand::ChooseRandom; // Dodany import
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
// Usunięto: use rand::seq::SliceRandom;
// Usunięto: use rand::thread_rng;

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

#[derive(Debug)]
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

        let lang_path = PathBuf::from(base_locales_path_str).join(current_lang_id_str.to_lowercase());

        let dictionaries_path = lang_path.join("dictionaries.json");
        let templates_path = lang_path.join("templates.json");

        let dictionaries_content = fs::read_to_string(&dictionaries_path).map_err(|e| {
            format!(
                "Failed to read dictionaries file '{}': {}",
                dictionaries_path.display(),
                e
            )
        })?;
        let dictionaries: Dictionaries = serde_json::from_str(&dictionaries_content)?;

        let templates_content = fs::read_to_string(&templates_path).map_err(|e| {
            format!(
                "Failed to read templates file '{}': {}",
                templates_path.display(),
                e
            )
        })?;
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
            .and_then(|items| items.choose().cloned())
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
                        warn!("Dictionary key {} not found or empty.", dict_name);
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
        if let Some(template) = self.templates.tagline_templates.choose() {
            self.process_template(template)
        } else {
            warn!("No tagline templates available.");
            "ERR_NO_TAGLINE_TEMPLATES".to_string()
        }
    }

    pub fn generate_team_name(&self) -> String {
        if let Some(template) = self.templates.team_name_templates.choose() {
            self.process_template(template)
        } else {
            warn!("No team name templates available.");
            "ERR_NO_TEAM_NAME_TEMPLATES".to_string()
        }
    }

    pub fn generate_terrain_name(&self) -> String {
        if let Some(template) = self.templates.terrain_name_templates.choose() {
            self.process_template(template)
        } else {
            warn!("No terrain name templates available.");
            "ERR_NO_TERRAIN_NAME_TEMPLATES".to_string()
        }
    }

    pub fn generate_examples(
        &self,
        generator_func: fn(&Self) -> String,
        count: usize,
    ) -> Vec<String> {
        (0..count).map(|_| generator_func(self)).collect()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GeneratorOutput {
    pub taglines: Vec<String>,
    pub team_names: Vec<String>,
    pub terrain_names: Vec<String>,
}

pub fn generate(
    base_locales_path: &str,
    lang_id: &str,
    count: usize,
) -> Result<GeneratorOutput, Box<dyn std::error::Error>> {
    let generator = WhackaMoleeGenerator::new(base_locales_path, lang_id)?;

    Ok(GeneratorOutput {
        taglines: generator.generate_examples(WhackaMoleeGenerator::generate_tagline, count),
        team_names: generator.generate_examples(WhackaMoleeGenerator::generate_team_name, count),
        terrain_names: generator.generate_examples(WhackaMoleeGenerator::generate_terrain_name, count),
    })
}
// ----END OF FILE----
// karolrybak/whacka-molee/whacka-molee-351ea95a23ebe18bf0ffbcd3437412c5de79bebd/src/text_generator.rs
// version:0.2.1