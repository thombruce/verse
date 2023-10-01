use bevy::prelude::*;

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
