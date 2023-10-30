use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use super::assets::AudioAssets;

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

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameCreate), game_setup);
        for state in GameState::variants() {
            app.add_systems(OnEnter(state), state_enter_despawn::<GameState>);
        }
    }
}

fn game_setup(
    mut commands: Commands,
    audios: Res<AudioAssets>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        AudioBundle {
            source: audios.ambience.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new_relative(0.5),
                ..default()
            },
        },
        Name::new("Ambient Music"),
    ));

    next_state.set(GameState::Active);
}

fn state_enter_despawn<T: States>(
    mut commands: Commands,
    state: ResMut<State<T>>,
    query: Query<(Entity, &ForState<T>)>,
) {
    for (entity, for_state) in &mut query.iter() {
        if !for_state.states.contains(state.get()) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
