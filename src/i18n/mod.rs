use bevy::prelude::*;
use bevy_fluent::prelude::*;

pub mod locales;

use locales::*;

pub struct I18nPlugin;
impl Plugin for I18nPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Locale::new(ru::RU).with_default(en::US))
            .insert_resource(I18n(Localization::new()))
            .add_plugins(FluentPlugin);
    }
}

#[derive(Resource, Deref)]
pub struct I18n(pub Localization);
