use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
    window::Cursor,
};

use crate::core::resources::assets::AudioAssets;

use super::{ForState, GameState};

pub(crate) fn game_setup(
    mut commands: Commands,
    audios: Res<AudioAssets>,
    mut window: Query<&mut Window>,
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

    // TODO: Reenable cursor when returning to StartMenu
    //       or when entering a menu interface.
    // TODO: Ideally also reenable cursor for debug menu
    //       in development mode.
    window.single_mut().cursor = Cursor {
        visible: false,
        ..default()
    };

    next_state.set(GameState::Active);
}

pub(crate) fn state_enter_despawn<T: States>(
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
