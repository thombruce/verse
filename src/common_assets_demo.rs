use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy_common_assets::ron::RonAssetPlugin;

use crate::core::resources::state::GameState;

pub struct CommonAssetsDemoPlugin;
impl Plugin for CommonAssetsDemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<Level>::new(&["config.ron"]))
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (spawn_level, spawn_level_again).run_if(in_state(GameState::StartMenu)),
            );
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let level = LevelHandle(asset_server.load("verse.config.ron"));
    commands.insert_resource(level);
}

fn spawn_level(
    // mut commands: Commands,
    level: Res<LevelHandle>,
    mut levels: ResMut<Assets<Level>>,
) {
    if let Some(level) = levels.remove(level.0.id()) {
        for position in level.positions {
            println!("{}", position[0]);
        }
    }
}

// It only works the FIRST time. I think because .remove() removes
// the data, it cannot be found again by a later system.
fn spawn_level_again(
    // mut commands: Commands,
    level: Res<LevelHandle>,
    mut levels: ResMut<Assets<Level>>,
) {
    if let Some(level) = levels.remove(level.0.id()) {
        for position in level.positions {
            println!("{}", position[0]);
        }
    }
}

#[derive(serde::Deserialize, TypeUuid, TypePath)]
#[uuid = "bdb624ed-62bc-447f-9f89-f361ed58748c"]
struct Level {
    positions: Vec<[f32; 3]>,
}

#[derive(Resource)]
struct LevelHandle(Handle<Level>);
