use bevy::prelude::*;
use bevy_fluent::prelude::*;
use unic_langid::langid;

pub struct I18nPlugin;
impl Plugin for I18nPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Locale::new(langid!("ru-RU")).with_default(langid!("en-US")))
            .insert_resource(I18n(Localization::new()))
            .add_plugins(FluentPlugin);
    }
}

#[derive(Resource, Deref)]
pub struct I18n(pub Localization);
