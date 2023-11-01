use bevy::audio::VolumeLevel;
use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::window::WindowMode;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_fluent::Locale;
use unic_langid::LanguageIdentifier;

use crate::i18n::locales::en;

use super::assets::DataAssets;

pub struct ConfigPlugin;
impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<GameConfig>::new(&["config.ron"]));

        app.insert_resource(GameConfig::default());
    }
}

#[derive(serde::Deserialize, TypeUuid, TypePath, Resource)]
#[uuid = "bdb624ed-62bc-447f-9f89-f361ed58748c"]
pub struct GameConfig {
    pub(crate) window_mode: WindowMode,
    pub(crate) master_volume: f32,
    pub(crate) locale: LanguageIdentifier,
}
impl Default for GameConfig {
    fn default() -> Self {
        Self {
            window_mode: WindowMode::Windowed,
            master_volume: 1.0,
            locale: en::US,
        }
    }
}

pub(crate) fn load_config(
    data: Res<DataAssets>,
    mut configs: ResMut<Assets<GameConfig>>,
    mut game_config: ResMut<GameConfig>,
    mut locale: ResMut<Locale>,
) {
    if let Some(config) = configs.remove(data.config.id()) {
        game_config.window_mode = config.window_mode;
        game_config.master_volume = config.master_volume;
        // game_config.locale = config.locale;
        locale.requested = config.locale;
    }
}

pub(crate) fn apply_config(
    config: Res<GameConfig>,
    mut window: Query<&mut Window>,
    mut volume: ResMut<GlobalVolume>,
) {
    window.single_mut().mode = config.window_mode;
    volume.volume = VolumeLevel::new(config.master_volume);
}
