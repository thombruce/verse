use unic_langid::{langid, LanguageIdentifier};

pub mod en {
    use super::*;

    pub const US: LanguageIdentifier = langid!("en-US");
}

pub mod ru {
    use super::*;

    pub const RU: LanguageIdentifier = langid!("ru-RU");
}
