use bevy::{asset::LoadState, prelude::*};
use bevy_fluent::prelude::*;
use fluent_content::Content;
use unic_langid::langid;

pub struct I18nPlugin;
impl Plugin for I18nPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Locale::new(langid!("en-US")))
            .add_plugins(FluentPlugin)
            .add_systems(Update, localized_hello_world);
    }
}

fn localized_hello_world(
    asset_server: Res<AssetServer>,
    assets: Res<Assets<BundleAsset>>,
    mut handle: Local<Option<Handle<BundleAsset>>>,
) {
    let handle = &*handle.get_or_insert_with(|| asset_server.load("locales/en-US/main.ftl.yml"));
    if let LoadState::Loaded = asset_server.get_load_state(handle) {
        let bundle = assets.get(handle).unwrap();
        assert!(matches!(bundle.content("hello-world"), Some(content) if content == "hello world"));
    }
}
