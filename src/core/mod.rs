use bevy::prelude::*;

pub mod effects;
pub mod resources;

use self::resources::{game_time::GameTimePlugin, score::ScorePlugin};

pub struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameTimePlugin, ScorePlugin));
    }
}
