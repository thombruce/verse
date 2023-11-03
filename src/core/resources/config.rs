use bevy::audio::VolumeLevel;
use bevy::prelude::*;
use bevy::window::WindowMode;
use bevy_fluent::Locale;
use serde::{Deserialize, Serialize};
use std::fs;
use unic_langid::LanguageIdentifier;

use crate::i18n::locales::en;

pub struct ConfigPlugin;
impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameConfig::default());
    }
}

// TODO: LanguageIdentifier does not implement the Copy trait and therefore
//       cannot be copied/borrowed/referenced as we'd like when setting
//       values in the GameConfig resource. Refactor usage of LanguageIdentifier
//       to improve handling of locale loading.

#[derive(Deserialize, Serialize, Resource)]
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

pub(crate) fn load_config(mut game_config: ResMut<GameConfig>, mut locale: ResMut<Locale>) {
    if let Ok(file_contents) = fs::read_to_string("verse.config.ron") {
        let config: GameConfig = ron::from_str(&file_contents).unwrap();

        game_config.window_mode = config.window_mode;
        game_config.master_volume = config.master_volume;
        // game_config.locale = config.locale;
        locale.requested = config.locale;
    } else {
        locale.requested = en::US;
        // TODO: This saves the default config. It can be generalised to save the GameConfig resource
        //       at any time, and reused whenever we need to save the config.
        //       However, we must be able to update the GameConfig locale LanguageIdentifier value
        //       before doing this; see above.
        if let Ok(string) =
            ron::ser::to_string_pretty(&GameConfig::default(), ron::ser::PrettyConfig::default())
        {
            let _ = fs::write("verse.config.ron", string);
        }
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
