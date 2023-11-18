use bevy::{prelude::*, time::Stopwatch};

#[derive(Resource, Deref)]
pub struct GameTime(pub Stopwatch);

#[derive(Resource, Deref)]
pub struct PlayTime(pub Stopwatch);

pub struct GameTimePlugin;
impl Plugin for GameTimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameTime(Stopwatch::new()));
        app.insert_resource(PlayTime(Stopwatch::new()));
    }
}

pub(crate) fn tick_game_time(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    // 1 day = 72 minutes
    // game_time.0.tick(20 * time.delta());

    // 1 month = 12 minutes
    game_time.0.tick(60 * 60 * time.delta());
    // This is about as fast as I would ever want the core game time progression
    // to be. Planets are noticably moving, but not so fast that it *feels*
    // too unnatural. It feels suitable for the game as an arcade shooter,
    // so this is probably where we'll land for v0.1.
}

pub(crate) fn tick_play_time(time: Res<Time>, mut play_time: ResMut<PlayTime>) {
    play_time.0.tick(time.delta());
}
