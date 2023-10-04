use bevy::{prelude::*, time::Stopwatch};

use crate::state::AppState;

#[derive(Resource, Deref)]
pub struct GameTime(Stopwatch);

pub struct GameTimePlugin;
impl Plugin for GameTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTime(Stopwatch::new()))
            .add_systems(Update, tick_game_time.run_if(in_state(AppState::Active)));
    }
}

fn tick_game_time(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    game_time.0.tick(time.delta());
}