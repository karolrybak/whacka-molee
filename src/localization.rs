use bevy::prelude::*;
use fluent::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use std::{collections::HashMap, fs, path::Path, sync::RwLock};
use unic_langid::LanguageIdentifier;

#[derive(Resource, Clone, Debug)]
pub struct CurrentLang(pub LanguageIdentifier);

#[derive(Resource)]
pub struct FluentBundleResource(pub RwLock<FluentBundle<FluentResource>>);

#[derive(Resource, Debug)]
struct AvailableLangs(Vec<LanguageIdentifier>);

#[derive(Resource, Debug)]
struct BaseLocalePath(String);

pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        let base_locales_path = "assets/locales".to_string();
        let initial_lang_id_str = "en";
        let supported_langs_str = ["en", "pl", "es"];

        app.insert_resource(BaseLocalePath(base_locales_path.clone()));

        let langs: Vec<LanguageIdentifier> = supported_langs_str
            .iter()
            .map(|s| s.parse().expect("Invalid supported language ID"))
            .collect();
        app.insert_resource(AvailableLangs(langs));

        let initial_lang_id: LanguageIdentifier = initial_lang_id_str
            .parse()
            .expect("Invalid initial language ID");

        app.insert_resource(CurrentLang(initial_lang_id.clone()));
        app.add_event::<LanguageChangeRequest>();

        match load_bundle_for_language(&base_locales_path, &initial_lang_id) {
            Ok(bundle) => {
                app.insert_resource(FluentBundleResource(RwLock::new(bundle)));
                info!(
                    "Localization initialized for language: {}",
                    initial_lang_id_str
                );
            }
            Err(errors) => {
                error!(
                    "Failed to load initial language '{}': {:?}. Using an empty fallback.",
                    initial_lang_id_str, errors
                );
                let fallback_bundle = FluentBundle::new(vec!["en-US".parse().unwrap()]);
                app.insert_resource(FluentBundleResource(RwLock::new(fallback_bundle)));
            }
        }

        app.add_systems(
            Update,
            on_language_change_request.in_set(LocalizationSystemSet::LanguageProcessing),
        );
    }
}

fn read_ftl_file(
    base_path_str: &str,
    lang_id: &LanguageIdentifier,
    filename: &str,
) -> Result<String, String> {
    let path_str = format!(
        "{}/{}/{}",
        base_path_str,
        lang_id.to_string().to_lowercase(),
        filename
    );
    let path = Path::new(&path_str);
    fs::read_to_string(path)
        .map_err(|e| format!("Failed to read FTL file '{}': {}", path.display(), e))
}

fn load_bundle_for_language(
    base_path: &str,
    lang_id: &LanguageIdentifier,
) -> Result<FluentBundle<FluentResource>, Vec<String>> {
    let mut bundle = FluentBundle::new(vec![lang_id.clone()]);
    let mut load_errors = Vec::new();
    let ftl_files = ["game.ftl"];

    for ftl_filename in &ftl_files {
        match read_ftl_file(base_path, lang_id, ftl_filename) {
            Ok(ftl_string) => {
                let resource = FluentResource::try_new(ftl_string).map_err(|(_res, errs)| {
                    errs.into_iter()
                        .map(|e| format!("FTL parse error in {}: {:?}", ftl_filename, e))
                        .collect::<Vec<String>>()
                })?;

                if let Err(ftl_errors) = bundle.add_resource_overriding(resource) {
                    for error_val in ftl_errors {
                        load_errors.push(format!(
                            "Error adding FTL resource from {}: {:?}",
                            ftl_filename, error_val
                        ));
                    }
                }
            }
            Err(read_err) => {
                let err_msg = format!(
                    "FTL file read error for lang '{}', file '{}': {}",
                    lang_id, ftl_filename, read_err
                );
                warn!("{}", err_msg);
                load_errors.push(err_msg);
            }
        }
    }

    if !load_errors.is_empty() {
        Err(load_errors)
    } else {
        Ok(bundle)
    }
}

#[derive(Event, Debug)]
pub struct LanguageChangeRequest(pub String);

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LocalizationSystemSet {
    LanguageProcessing,
}

fn on_language_change_request(
    mut ev_lang_change: EventReader<LanguageChangeRequest>,
    base_path: Res<BaseLocalePath>,
    available_langs: Res<AvailableLangs>,
    mut current_lang_res: ResMut<CurrentLang>,
    bundle_res: ResMut<FluentBundleResource>,
) {
    for event in ev_lang_change.read() {
        let lang_id_str = &event.0;
        match lang_id_str.parse::<LanguageIdentifier>() {
            Ok(new_lang_id) => {
                if available_langs.0.contains(&new_lang_id) {
                    if current_lang_res.0 != new_lang_id {
                        match load_bundle_for_language(&base_path.0, &new_lang_id) {
                            Ok(new_bundle) => {
                                let mut bundle_guard = bundle_res
                                    .0
                                    .write()
                                    .expect("Failed to write FluentBundleResource");
                                *bundle_guard = new_bundle;
                                current_lang_res.0 = new_lang_id;
                                info!("Successfully set language to: {}", lang_id_str);
                            }
                            Err(errors) => {
                                error!(
                                    "Failed to load bundle for language '{}': {:?}",
                                    lang_id_str, errors
                                );
                            }
                        }
                    } else {
                        info!("Language {} is already set.", lang_id_str);
                    }
                } else {
                    error!("Attempted to set unsupported language: {}", lang_id_str);
                }
            }
            Err(e) => {
                error!(
                    "Invalid language ID in LanguageChangeRequest '{}': {}",
                    lang_id_str, e
                );
            }
        }
    }
}

#[macro_export]
macro_rules! t {
    ($bundle_res:expr, $key:expr) => {
        $crate::localization::translate_with_bundle_res_macro(&$bundle_res, $key, None)
    };
    ($bundle_res:expr, $key:expr, {$($name:ident = $value:expr),* $(,)?}) => {
        {
            let mut args = fluent::FluentArgs::new();
            $(
                args.set(stringify!($name).replace("_", "-"), fluent::FluentValue::from($value.clone()));
            )*
            $crate::localization::translate_with_bundle_res_macro(&$bundle_res, $key, Some(args))
        }
    };
}

pub fn translate_with_bundle_res_macro(
    bundle_res: &Res<FluentBundleResource>,
    key: &str,
    args: Option<fluent::FluentArgs<'_>>,
) -> String {
    let bundle_guard = match bundle_res.0.read() {
        Ok(guard) => guard,
        Err(_) => {
            error!(
                "Localization: Failed to acquire read lock on FluentBundleResource for key '{}'",
                key
            );
            return format!("LOCK_ERR:{}", key);
        }
    };
    let bundle = &*bundle_guard;

    let msg = bundle.get_message(key);
    if msg.is_none() {
        return format!("MISSING_KEY:{}", key);
    }
    let pattern = msg.unwrap().value();
    if pattern.is_none() {
        return format!("NO_VALUE:{}", key);
    }
    let mut errors = Vec::new();
    let value = bundle.format_pattern(pattern.unwrap(), args.as_ref(), &mut errors);
    if !errors.is_empty() {
        error!(
            "Fluent format error for key '{}' with args {:?}: {:?}",
            key,
            args.map(|a| a.iter().collect::<Vec<_>>()),
            errors
        );
    }
    value.into_owned()
}
