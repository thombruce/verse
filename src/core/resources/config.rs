use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::window::WindowMode;
use bevy_common_assets::ron::RonAssetPlugin;

pub struct ConfigPlugin;
impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<GameConfig>::new(&["config.ron"]));
    }
}

#[derive(serde::Deserialize, TypeUuid, TypePath)]
#[uuid = "bdb624ed-62bc-447f-9f89-f361ed58748c"]
pub struct GameConfig {
    pub(crate) window_mode: WindowMode,
    pub(crate) master_volume: f32,
}
