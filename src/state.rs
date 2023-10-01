use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

#[derive(States, Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
pub enum AppState {
    #[default]
    StartMenu,
    GameCreate,
    Active,
    Paused,
}
impl AppState {
    pub const IN_GAME_STATE: &[AppState; 3] =
        &[AppState::GameCreate, AppState::Active, AppState::Paused];
}

pub fn is_in_game_state(state: Res<State<AppState>>) -> bool {
    AppState::IN_GAME_STATE.contains(state.get())
}

pub struct PausePlugin;
impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameCreate), game_setup);
    }
}

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    commands.spawn(AudioBundle {
        source: asset_server.load("sound/Kirk Osamayo - Space Dust.ogg"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new_absolute(0.5),
            ..default()
        },
    });

    next_state.set(AppState::Active);
}
