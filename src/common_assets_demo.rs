use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy_common_assets::ron::RonAssetPlugin;

use crate::core::resources::{assets::DataAssets, state::GameState};

pub struct CommonAssetsDemoPlugin;
impl Plugin for CommonAssetsDemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<Config>::new(&["config.ron"]))
            .add_systems(OnEnter(GameState::StartMenu), spawn_config);
    }
}

fn spawn_config(data: Res<DataAssets>, configs: ResMut<Assets<Config>>) {
    if let Some(config) = configs.get(&data.config.clone()) {
        for position in &config.positions {
            println!("{}", position[0]);
        }
    }
}

#[derive(serde::Deserialize, TypeUuid, TypePath)]
#[uuid = "bdb624ed-62bc-447f-9f89-f361ed58748c"]
pub struct Config {
    positions: Vec<[f32; 3]>,
}
