use bevy::prelude::*;
use bevy_fluent::prelude::*;
use unic_langid::langid;

pub struct I18nPlugin;
impl Plugin for I18nPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Locale::new(langid!("en-US")))
            .add_plugins(FluentPlugin);
    }
}
