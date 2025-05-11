use macroquad::prelude::{
    Font, Texture2D, debug, error, info, load_string, load_texture, load_ttf_font, warn,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct AssetManager {
    textures: HashMap<String, Texture2D>,
    fonts: HashMap<String, Font>,
}

impl AssetManager {
    pub async fn new(base_path_str: &str) -> Self {
        let base_path = PathBuf::from(base_path_str);
        let manifest_path = base_path.join("assets.txt");

        let mut textures = HashMap::new();
        let mut fonts = HashMap::new();

        info!(
            "Attempting to preload assets from manifest: {}",
            manifest_path.display()
        );

        let manifest_content = match load_string(manifest_path.to_str().unwrap()).await {
            Ok(content) => content,
            Err(err) => {
                error!(
                    "FATAL: Error opening or reading asset manifest file '{}': {}. Ensure the file exists and is readable.",
                    manifest_path.display(),
                    err
                );
                panic!("Failed to load asset manifest!");
            }
        };

        for line in manifest_content.lines() {
            let asset_key_str = line.trim();
            if asset_key_str.is_empty() {
                continue;
            }

            let asset_path = Path::new(asset_key_str);
            let full_asset_path = base_path.join(asset_path);

            let full_path_str = match full_asset_path.to_str() {
                Some(s) => s,
                None => {
                    error!(
                        "Invalid non-UTF8 path for asset: {}",
                        full_asset_path.display()
                    );
                    continue;
                }
            };

            match asset_path.extension().and_then(|ext| ext.to_str()) {
                Some("ttf") | Some("otf") => match load_ttf_font(full_path_str).await {
                    Ok(font_data) => {
                        fonts.insert(asset_key_str.to_string(), font_data);
                        info!("Loaded font '{}' from '{}'", asset_key_str, full_path_str);
                    }
                    Err(err) => {
                        error!(
                            "Failed to load font '{}' from '{}': {}",
                            asset_key_str, full_path_str, err
                        );
                    }
                },
                Some("png") | Some("jpg") | Some("jpeg") => {
                    match load_texture(full_path_str).await {
                        Ok(texture_data) => {
                            textures.insert(asset_key_str.to_string(), texture_data);
                            info!(
                                "Loaded texture '{}' from '{}'",
                                asset_key_str, full_path_str
                            );
                        }
                        Err(err) => {
                            error!(
                                "Failed to load texture '{}' from '{}': {}",
                                asset_key_str, full_path_str, err
                            );
                        }
                    }
                }
                Some(other_ext) => {
                    warn!(
                        "Unsupported asset extension '{}' for asset: {}. Skipping.",
                        other_ext, asset_key_str
                    );
                }
                None => {
                    warn!(
                        "Asset '{}' has no extension. Assuming texture for backward compatibility (consider adding extension).",
                        asset_key_str
                    );

                    match load_texture(full_path_str).await {
                        Ok(texture_data) => {
                            textures.insert(asset_key_str.to_string(), texture_data);
                            info!(
                                "Loaded (assumed) texture '{}' from '{}'",
                                asset_key_str, full_path_str
                            );
                        }
                        Err(err) => {
                            error!(
                                "Failed to load (assumed) texture '{}' from '{}': {}",
                                asset_key_str, full_path_str, err
                            );
                        }
                    }
                }
            }
        }

        debug!("AssetManager initialized.");
        AssetManager { textures, fonts }
    }

    pub fn get_texture(&self, id: &str) -> Texture2D {
        self.textures.get(id).cloned().unwrap_or_else(|| {
            error!(
                "AssetManager: Texture with id '{}' not found. Check assets.txt and file path.",
                id
            );
            panic!("Texture not found: {}", id);
        })
    }

    pub fn get_font(&self, id: &str) -> Font {
        self.fonts.get(id).cloned().unwrap_or_else(|| {
            error!(
                "AssetManager: Font with id '{}' not found. Check assets.txt (e.g., 'fonts/MyFont.ttf') and file path.",
                id
            );
            panic!("Font not found: {}", id);
        })
    }
}
