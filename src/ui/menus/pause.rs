use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierConfiguration;
use leafwing_input_manager::prelude::{ActionState, InputManagerPlugin};

use crate::{
    core::{
        effects::blink::DrawBlinkTimer,
        resources::{
            assets::UiAssets,
            state::{ForState, GameState},
        },
    },
    inputs::pause::{pause_input_map, PauseAction},
};

pub struct PausePlugin;
impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PauseAction>::default());
    }
}

pub(crate) fn setup_pause_systems(mut commands: Commands) {
    commands.insert_resource(pause_input_map());
    commands.insert_resource(ActionState::<PauseAction>::default());
}

pub(crate) fn pause_system(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    pause_action_state: Res<ActionState<PauseAction>>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
) {
    if pause_action_state.just_pressed(PauseAction::Pause) {
        match state.get() {
            GameState::Active => {
                next_state.set(GameState::Paused);
                rapier_configuration.physics_pipeline_active = false;
            }
            GameState::Paused => {
                next_state.set(GameState::Active);
                rapier_configuration.physics_pipeline_active = true;
            }
            _ => {}
        }
    }
}

pub(crate) fn pause_screen(mut commands: Commands, ui: Res<UiAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0., 0., 0., 0.65)),
                ..default()
            },
            ForState {
                states: vec![GameState::Paused],
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "Pause".to_ascii_uppercase(),
                        TextStyle {
                            font: ui.font.clone(),
                            font_size: 50.0,
                            color: Color::rgb_u8(0x88, 0x00, 0x00),
                        },
                    ),
                    ..default()
                },
                DrawBlinkTimer(Timer::from_seconds(0.65, TimerMode::Repeating)),
            ));
        });
}
