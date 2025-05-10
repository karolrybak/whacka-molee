use image::ImageFormat;



use macroquad::prelude::{FilterMode, Texture2D, debug, error, info, load_texture, warn}; 

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetInfo {
    pub id: String,
    pub asset_type: String,
    pub path: String,
    pub description: String,
}

pub struct AssetManager {
    textures: HashMap<String, Texture2D>,
}

impl AssetManager {
    pub async fn new(base_path_str: &str) -> Self {
        let base_path = PathBuf::from(base_path_str);
        let manifest_path = base_path.join("assets.txt");
        let mut textures: HashMap<String, Texture2D> = HashMap::new();

        info!(
            "Attempting to preload assets from manifest: {}",
            manifest_path.display()
        );
        let file = match File::open(manifest_path) {
            Ok(file) => file,
            Err(err) => {
                panic!("Error opening file: {}", err);
            }
        };
        let reader = BufReader::new(file);

        for line_result in reader.lines() {
            match line_result {
                Ok(line) => {
                    println!("{:?}", line); 
                    let file_path = base_path.join(line.clone());
                    let texture = match load_texture(file_path.to_str().unwrap()).await {
                    Ok(tex) => tex,
                        Err(err) => {
                            error!("Failed to load texture: {}", err);
                            continue;
                        }
                    };
                    textures.insert(line.clone(), texture);
                    println!("Loaded texture {}", line);
                }
                Err(err) => {
                    panic!("Error reading line: {}", err);
                }
            }
        }

        debug!("AssetManager::new called with base_path: {}", base_path_str);
        AssetManager {
            textures: textures, 
        }
    }

    pub fn get_texture(&self, id: &str) -> Texture2D {
        self.textures.get(id).cloned().unwrap_or_else(|| {
            panic!("AssetManager: Texture with id '{}' not found or not loaded. Ensure it's in assets.json and the file exists.", id);
        })
    }
}



