use bevy::{prelude::*, time::Stopwatch};

#[derive(Resource, Deref)]
pub struct GameTime(Stopwatch);

pub struct GameTimePlugin;
impl Plugin for GameTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTime(Stopwatch::new()));
    }
}

pub(crate) fn tick_game_time(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    game_time.0.tick(time.delta());
}
