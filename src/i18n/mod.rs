use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_fluent::prelude::*;
use bevy_fluent::{AssetServerExt, BundleAsset, LocalizationBuilder};

use crate::systems::states::GameState;

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

pub(crate) fn load_translations(
    localization_builder: LocalizationBuilder,
    asset_server: Res<AssetServer>,
    mut handles: Local<Option<Vec<Handle<BundleAsset>>>>,
    mut i18n: ResMut<I18n>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let handles =
        handles.get_or_insert_with(|| asset_server.load_glob("locales/**/main.ftl.ron").unwrap());

    let load_state = asset_server.get_group_load_state(handles.iter().map(Handle::id));
    if let LoadState::Loaded = load_state {
        i18n.0 = localization_builder.build(&*handles);
        next_state.set(GameState::StartMenu);
    }
}
