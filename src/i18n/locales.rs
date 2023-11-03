use bevy::prelude::*;
use unic_langid::{langid, LanguageIdentifier};

/// Locales
#[derive(Resource)]
pub struct Locales(pub Vec<LanguageIdentifier>);

pub mod en {
    use super::*;

    pub const US: LanguageIdentifier = langid!("en-US");
}

pub mod ru {
    use super::*;

    pub const RU: LanguageIdentifier = langid!("ru-RU");
}

pub mod de {
    use super::*;

    pub const DE: LanguageIdentifier = langid!("de-DE");
}
