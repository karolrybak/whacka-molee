use bevy::prelude::*;
use fluent::FluentArgs;

#[derive(Resource, Default)]
pub struct LocalizationResource;

#[derive(Resource, Clone, Debug)]
pub struct CurrentLang(pub String);

impl Default for CurrentLang {
    fn default() -> Self {
        Self("en".to_string())
    }
}

// Empty resource just to satisfy the API
#[derive(Resource, Default)]
pub struct FluentBundleResource;

#[derive(Event)]
pub struct LanguageChangeRequest(pub String);

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum LocalizationSystemSet {
    LanguageProcessing,
}

/// Macro `t!` for retrieving translated texts
#[macro_export]
macro_rules! t {
    ($key:expr) => {
        $key.to_string()
    };
    ($key:expr, $( $k:expr => $v:expr ),+ $(,)?) => {
        $key.to_string()
    };
    ($bundle_res:expr, $key:expr) => {
        $key.to_string()
    };
    ($bundle_res:expr, $key:expr, $( $k:expr => $v:expr ),+ $(,)?) => {
        $key.to_string()
    };
}

/// Function to translate text with optional Fluent arguments
pub fn translate(key: &str, _args: Option<FluentArgs>) -> String {
    key.to_string()
}

/// Localization plugin
pub struct LocalizationPlugin;

impl Plugin for LocalizationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LocalizationResource>()
            .init_resource::<CurrentLang>()
            .init_resource::<FluentBundleResource>()
            .add_event::<LanguageChangeRequest>()
            .configure_sets(Update, LocalizationSystemSet::LanguageProcessing)
            .add_systems(Startup, setup_localization)
            .add_systems(Update, handle_language_change.in_set(LocalizationSystemSet::LanguageProcessing));
    }
}

/// Set up language and initialize global localization
fn setup_localization(current_lang: Res<CurrentLang>) {
    info!("Setting up localization with language: {}", current_lang.0);
}

/// Handle language change requests
fn handle_language_change(
    mut events: EventReader<LanguageChangeRequest>,
    mut current_lang: ResMut<CurrentLang>,
) {
    for event in events.read() {
        info!("Changing language to: {}", event.0);
        current_lang.0 = event.0.clone();
    }
}
