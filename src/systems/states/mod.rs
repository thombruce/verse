use bevy::prelude::*;

pub mod transitions;

#[derive(States, Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
pub enum GameState {
    #[default]
    Loading,
    LoadingTranslations,
    // Menu States
    StartMenu,
    Credits,
    // Game States
    GameCreate,
    Active,
    Paused,
    GameOver,
    Reset,
}
impl GameState {
    pub const IN_ANY_STATE: &[GameState; 9] = &[
        GameState::Loading,
        GameState::LoadingTranslations,
        GameState::StartMenu,
        GameState::Credits,
        GameState::GameCreate,
        GameState::Active,
        GameState::Paused,
        GameState::GameOver,
        GameState::Reset,
    ];
    pub const IN_MENU_STATE: &[GameState; 4] = &[
        GameState::StartMenu,
        GameState::Credits,
        GameState::Paused,
        GameState::GameOver,
    ];
    pub const IN_GAME_STATE: &[GameState; 4] = &[
        GameState::GameCreate,
        GameState::Active,
        GameState::Paused,
        GameState::GameOver,
    ];
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
