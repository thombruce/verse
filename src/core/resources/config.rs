use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::window::WindowMode;
use bevy_common_assets::ron::RonAssetPlugin;

use super::{assets::DataAssets, state::GameState};

pub struct ConfigPlugin;
impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<GameConfig>::new(&["config.ron"]));

        app.insert_resource(GameConfig {
            window_mode: WindowMode::Fullscreen,
            master_volume: 1.0,
        });

        app.add_systems(OnExit(GameState::Loading), load_config);
    }
}

#[derive(serde::Deserialize, TypeUuid, TypePath, Resource)]
#[uuid = "bdb624ed-62bc-447f-9f89-f361ed58748c"]
pub struct GameConfig {
    pub(crate) window_mode: WindowMode,
    pub(crate) master_volume: f32,
}

fn load_config(
    data: Res<DataAssets>,
    mut configs: ResMut<Assets<GameConfig>>,
    mut game_config: ResMut<GameConfig>,
) {
    if let Some(config) = configs.remove(data.config.id()) {
        game_config.window_mode = config.window_mode;
        game_config.master_volume = config.master_volume;
    }
}
