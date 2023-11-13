use bevy::asset::{LoadState, LoadedFolder};
use bevy::prelude::*;
use bevy_fluent::prelude::*;
use bevy_fluent::LocalizationBuilder;

use crate::systems::states::GameState;

pub mod locales;

use locales::*;

pub struct I18nPlugin;
impl Plugin for I18nPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Locale::new(en::US).with_default(en::US))
            .insert_resource(I18n(Localization::new()))
            .insert_resource(Locales(vec![de::DE, en::US, ru::RU]))
            .add_plugins(FluentPlugin);
    }
}

#[derive(Resource, Deref)]
pub struct I18n(pub Localization);

pub(crate) fn load_translations(
    localization_builder: LocalizationBuilder,
    asset_server: Res<AssetServer>,
    mut handle: Local<Option<Handle<LoadedFolder>>>,
    mut i18n: ResMut<I18n>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let handle = &*handle.get_or_insert_with(|| asset_server.load_folder("locales"));

    let load_state = asset_server.get_load_state(handle);
    if let Some(LoadState::Loaded) = load_state {
        i18n.0 = localization_builder.build(handle);
        next_state.set(GameState::StartMenu);
    }
}
