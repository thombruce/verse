use bevy::prelude::*;

pub mod transitions;

#[derive(States, Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
pub enum GameState {
    #[default]
    Loading,
    StartMenu,
    GameCreate,
    Active,
    Paused,
    Credits,
}
impl GameState {
    pub const IN_MENU_STATE: &[GameState; 2] = &[GameState::StartMenu, GameState::Credits];
    pub const IN_GAME_STATE: &[GameState; 3] =
        &[GameState::GameCreate, GameState::Active, GameState::Paused];
}

pub fn is_in_menu_state(state: Res<State<GameState>>) -> bool {
    GameState::IN_MENU_STATE.contains(state.get())
}

pub fn is_in_game_state(state: Res<State<GameState>>) -> bool {
    GameState::IN_GAME_STATE.contains(state.get())
}

/// Component to tag an entity as only needed in some of the states
#[derive(Component, Debug)]
pub struct ForState<T> {
    pub states: Vec<T>,
}
