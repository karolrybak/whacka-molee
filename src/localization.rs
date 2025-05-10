use fluent::{FluentBundle, FluentResource, FluentArgs, FluentValue};
use unic_langid::{LanguageIdentifier};
use std::cell::RefCell;
use std::rc::Rc;
use std::fs;
use std::path::Path;
use macroquad::prelude::{info, warn, error}; 

thread_local! {
    pub static CURRENT_BUNDLE: RefCell<Option<Rc<FluentBundle<FluentResource>>>> = RefCell::new(None);
    static AVAILABLE_LANGS: RefCell<Vec<LanguageIdentifier>> = RefCell::new(Vec::new());
    static BASE_LOCALE_PATH: RefCell<String> = RefCell::new(String::new());
}

fn read_ftl_file(base_path_str: &str, lang_id: &LanguageIdentifier, filename: &str) -> Result<String, String> {
    let path_str = format!("{}/{}/{}", base_path_str, lang_id.to_string().to_lowercase(), filename);
    let path = Path::new(&path_str);
    fs::read_to_string(path)
        .map_err(|e| format!("Failed to read FTL file '{}': {}", path.display(), e))
}

fn load_bundle_for_language(base_path: &str, lang_id: &LanguageIdentifier) -> Result<Rc<FluentBundle<FluentResource>>, Vec<String>> {
    let mut bundle = FluentBundle::new(vec![lang_id.clone()]);
    let mut errors = Vec::new();
    let ftl_files = ["game.ftl"]; 

    for ftl_filename in &ftl_files {
        match read_ftl_file(base_path, lang_id, ftl_filename) {
            Ok(ftl_string) => {
                let resource = FluentResource::try_new(ftl_string)
                    .map_err(|(_res, errs)| errs.into_iter().map(|e| format!("FTL parse error in {}: {:?}", ftl_filename, e)).collect::<Vec<String>>())?;
                if let Err(ftl_errors) = bundle.add_resource(resource) {
                    for error in ftl_errors {
                        errors.push(format!("Error adding FTL resource from {}: {:?}", ftl_filename, error));
                    }
                }
            }
            Err(read_err) => {
                warn!("FTL file read warning for lang '{}', file '{}': {}", lang_id, ftl_filename, read_err);
            }
        }
    }
    
    if !errors.is_empty() {
        Err(errors)
    } else {
        Ok(Rc::new(bundle))
    }
}

pub fn init_localization(base_locales_path: &str, initial_lang_id_str: &str, supported_langs_str: &[&str]) {
    info!("Initializing localization with base path: '{}', initial lang: '{}'", base_locales_path, initial_lang_id_str);
    BASE_LOCALE_PATH.with(|bp_cell| *bp_cell.borrow_mut() = base_locales_path.to_string());
    
    let langs: Vec<LanguageIdentifier> = supported_langs_str.iter().map(|s| s.parse().expect("Invalid supported language ID")).collect();
    AVAILABLE_LANGS.with(|al_cell| *al_cell.borrow_mut() = langs);

    match initial_lang_id_str.parse::<LanguageIdentifier>() {
        Ok(lang_id) => {
            if let Err(errors) = set_current_language(&lang_id.to_string()) { 
                error!("Failed to load initial language '{}': {:?}", initial_lang_id_str, errors);
                
                CURRENT_BUNDLE.with(|cell| *cell.borrow_mut() = Some(Rc::new(FluentBundle::new(vec!["en-US".parse().expect("Parsing failed.")]))));
                error!("Initialized with an empty fallback bundle for en-US due to errors.");
            }
        }
        Err(e) => {
            error!("Invalid initial language ID '{}': {}. Using empty fallback bundle.", initial_lang_id_str, e);
            CURRENT_BUNDLE.with(|cell| *cell.borrow_mut() = Some(Rc::new(FluentBundle::new(vec!["en-US".parse().expect("Parsing failed.")]))));
        }
    }
}

pub fn set_current_language(lang_id_str: &str) -> Result<(), Vec<String>> {
    let lang_id: LanguageIdentifier = lang_id_str.parse().map_err(|e| vec![format!("Invalid language ID '{}': {}", lang_id_str, e)])?;
    
    let base_path = BASE_LOCALE_PATH.with(|bp_cell| bp_cell.borrow().clone());
    let is_supported = AVAILABLE_LANGS.with(|al_cell| al_cell.borrow().contains(&lang_id));

    if !is_supported {
        let err_msg = format!("Language '{}' is not in the supported list.", lang_id_str);
        error!("{}", err_msg);
        return Err(vec![err_msg]);
    }
    info!("Attempting to set current language to: {}", lang_id_str);
    match load_bundle_for_language(&base_path, &lang_id) {
        Ok(bundle) => {
            CURRENT_BUNDLE.with(|cell| {
                *cell.borrow_mut() = Some(bundle);
            });
            info!("Successfully set language to: {}", lang_id_str);
            Ok(())
        }
        Err(errors) => {
            error!("Failed to load bundle for language '{}': {:?}", lang_id_str, errors);
            Err(errors)
        }
    }
}

#[macro_export]
macro_rules! t {
    ($key:expr) => {
        $crate::localization::CURRENT_BUNDLE.with(|cell| {
            match cell.borrow().as_ref() {
                Some(bundle) => {
                    let msg = bundle.get_message($key);
                    if msg.is_none() {
                        warn!("Localization: Missing message for key '{}'", $key);
                        return format!("MISSING_KEY:{}", $key);
                    }
                    let pattern = msg.unwrap().value();
                    if pattern.is_none() {
                        warn!("Localization: Message for key '{}' has no value", $key);
                        return format!("NO_VALUE:{}", $key);
                    }
                    let mut errors = Vec::new();
                    let value = bundle.format_pattern(pattern.unwrap(), None, &mut errors);
                    if !errors.is_empty() {
                        error!("Fluent format error for key '{}': {:?}", $key, errors);
                    }
                    value.into_owned()
                }
                None => {
                    error!("Localization: CURRENT_BUNDLE is None when accessing key '{}'. Was init_localization called?", $key);
                    format!("UNINIT_LOC:{}", $key)
                }
            }
        })
    };
    ($key:expr, $($arg_name:ident = $arg_value:expr),* $(,)?) => {
        $crate::localization::CURRENT_BUNDLE.with(|cell| {
            match cell.borrow().as_ref() {
                Some(bundle) => {
                    let mut args = fluent_bundle::FluentArgs::new();
                    $(
                        
                        
                        args.set(stringify!($arg_name).replace('_', "-"), Into::<fluent_bundle::FluentValue>::into($arg_value.clone()));
                    )*
                    let msg = bundle.get_message($key);
                     if msg.is_none() {
                        warn!("Localization: Missing message for key '{}'", $key);
                        return format!("MISSING_KEY:{}", $key);
                    }
                    let pattern = msg.unwrap().value();
                    if pattern.is_none() {
                        warn!("Localization: Message for key '{}' has no value", $key);
                        return format!("NO_VALUE:{}", $key);
                    }
                    let mut errors = Vec::new();
                    let value = bundle.format_pattern(pattern.unwrap(), Some(&args), &mut errors);
                    if !errors.is_empty() {
                        error!("Fluent format error for key '{}' with args: {:?}", $key, errors);
                    }
                    value.into_owned()
                }
                None => {
                    error!("Localization: CURRENT_BUNDLE is None when accessing key '{}'. Was init_localization called?", $key);
                    format!("UNINIT_LOC:{}", $key)
                }
            }
        })
    };
}


