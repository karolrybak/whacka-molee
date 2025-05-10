use rand::seq::SliceRandom;
use regex::{Captures, Regex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

// Types for deserializing JSON dictionaries
#[derive(Serialize, Deserialize, Debug)]
struct Dictionaries {
    #[serde(flatten)]
    dictionaries: HashMap<String, Vec<String>>,
}

// Types for deserializing JSON templates
#[derive(Serialize, Deserialize, Debug)]
struct Templates {
    TAGLINE_TEMPLATES: Vec<String>,
    TEAM_NAME_TEMPLATES: Vec<String>,
    TERRAIN_NAME_TEMPLATES: Vec<String>,
}

#[derive(Debug)]
struct WhackaMoleeGenerator {
    dictionaries: Dictionaries,
    templates: Templates,
    rng: rand::rngs::ThreadRng,
}

impl WhackaMoleeGenerator {
    // Create a new generator by loading dictionaries and templates from files
    fn new(dictionaries_path: &str, templates_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut dictionaries_file = File::open(dictionaries_path)?;
        let mut templates_file = File::open(templates_path)?;
        
        let mut dictionaries_content = String::new();
        let mut templates_content = String::new();
        
        dictionaries_file.read_to_string(&mut dictionaries_content)?;
        templates_file.read_to_string(&mut templates_content)?;
        
        let dictionaries: Dictionaries = serde_json::from_str(&dictionaries_content)?;
        let templates: Templates = serde_json::from_str(&templates_content)?;
        
        Ok(Self {
            dictionaries,
            templates,
            rng: rand::thread_rng(),
        })
    }
    
    // Get a random element from a dictionary
    fn get_random_from_dict(&mut self, dict_name: &str) -> Option<&str> {
        self.dictionaries.dictionaries.get(dict_name)
            .and_then(|items| items.choose(&mut self.rng))
            .map(|s| s.as_str())
    }
    
    // Pluralize a word with basic rules
    fn pluralize(&self, word: &str) -> String {
        if word.ends_with("y") && !word.ends_with("ay") && !word.ends_with("ey") && !word.ends_with("oy") && !word.ends_with("uy") {
            format!("{}ies", &word[0..word.len()-1])
        } else if word.ends_with("ch") || word.ends_with("s") || word.ends_with("sh") || word.ends_with("x") || word.ends_with("z") {
            format!("{}es", word)
        } else {
            format!("{}s", word)
        }
    }
    
    // Process a template by filling in placeholders with randomly selected words
    fn process_template(&mut self, template: &str) -> String {
        let re = Regex::new(r"([A-Z_]+)(_PLURAL)?").unwrap();
        
        // Process the template iteratively to ensure multiple passes replace all tokens
        // This helps with templates that might depend on other templates
        let mut result = template.to_string();
        let mut prev_result = String::new();
        
        // Keep replacing until no more changes (or max 3 iterations to prevent infinite loops)
        for _ in 0..3 {
            prev_result = result.clone();
            
            result = re.replace_all(&result, |caps: &Captures| {
                let capture = caps.get(0).unwrap().as_str();
                let dict_name = caps.get(1).unwrap().as_str();
                let is_plural = caps.get(2).is_some();
                
                // If it's a direct dictionary reference
                if let Some(word) = self.get_random_from_dict(dict_name) {
                    if is_plural {
                        self.pluralize(word)
                    } else {
                        word.to_string()
                    }
                } else {
                    // Return the original capture if no replacement found
                    capture.to_string()
                }
            }).to_string();
            
            // If no changes were made, we're done
            if result == prev_result {
                break;
            }
        }
        
        result
    }
    
    // Generate a tagline for the logo
    pub fn generate_tagline(&mut self) -> String {
        let template = self.templates.TAGLINE_TEMPLATES.choose(&mut self.rng).unwrap();
        self.process_template(template)
    }
    
    // Generate a team name
    pub fn generate_team_name(&mut self) -> String {
        let template = self.templates.TEAM_NAME_TEMPLATES.choose(&mut self.rng).unwrap();
        self.process_template(template)
    }
    
    // Generate a terrain seed name
    pub fn generate_terrain_name(&mut self) -> String {
        let template = self.templates.TERRAIN_NAME_TEMPLATES.choose(&mut self.rng).unwrap();
        self.process_template(template)
    }
    
    // Generate multiple examples of a given type
    pub fn generate_examples(&mut self, generator_func: fn(&mut Self) -> String, count: usize) -> Vec<String> {
        (0..count).map(|_| generator_func(self)).collect()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new generator
    let mut generator = WhackaMoleeGenerator::new("dictionaries.json", "templates.json")?;
    
    // Generate and print examples
    println!("=== LOGO TAGLINES ===");
    let taglines = generator.generate_examples(WhackaMoleeGenerator::generate_tagline, 10);
    for (i, tagline) in taglines.iter().enumerate() {
        println!("{}. {}", i + 1, tagline);
    }
    
    println!("\n=== TEAM NAMES ===");
    let team_names = generator.generate_examples(WhackaMoleeGenerator::generate_team_name, 10);
    for (i, team_name) in team_names.iter().enumerate() {
        println!("{}. {}", i + 1, team_name);
    }
    
    println!("\n=== TERRAIN SEED NAMES ===");
    let terrain_names = generator.generate_examples(WhackaMoleeGenerator::generate_terrain_name, 10);
    for (i, terrain_name) in terrain_names.iter().enumerate() {
        println!("{}. {}", i + 1, terrain_name);
    }
    
    Ok(())
}

// To support command-line interface and easier integration with other systems
#[derive(Serialize, Deserialize, Debug)]
struct GeneratorOutput {
    taglines: Vec<String>,
    team_names: Vec<String>,
    terrain_names: Vec<String>
}

// Library function that can be imported elsewhere in your Rust project
pub fn generate(count: usize) -> Result<GeneratorOutput, Box<dyn std::error::Error>> {
    let mut generator = WhackaMoleeGenerator::new("dictionaries.json", "templates.json")?;
    
    Ok(GeneratorOutput {
        taglines: generator.generate_examples(WhackaMoleeGenerator::generate_tagline, count),
        team_names: generator.generate_examples(WhackaMoleeGenerator::generate_team_name, count),
        terrain_names: generator.generate_examples(WhackaMoleeGenerator::generate_terrain_name, count)
    })
}