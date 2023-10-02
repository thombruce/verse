use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierConfiguration;
use leafwing_input_manager::{
    prelude::{ActionState, InputManagerPlugin, InputMap},
    Actionlike,
};

use crate::{
    assets::UiAssets,
    effects::DrawBlinkTimer,
    state::{is_in_game_state, AppState, ForState},
};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PauseAction {
    Pause,
}

pub struct PausePlugin;
impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PauseAction>::default());

        app.add_systems(OnEnter(AppState::GameCreate), setup)
            .add_systems(Update, pause_system.run_if(is_in_game_state))
            .add_systems(OnEnter(AppState::Paused), pause_screen);
    }
}

fn setup(mut commands: Commands) {
    let mut input_map = InputMap::<PauseAction>::new([
        (KeyCode::Escape, PauseAction::Pause),
        (KeyCode::P, PauseAction::Pause),
    ]);
    input_map.insert(GamepadButtonType::Start, PauseAction::Pause);

    commands.insert_resource(input_map.build());
    commands.insert_resource(ActionState::<PauseAction>::default());
}

fn pause_system(
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    pause_action_state: Res<ActionState<PauseAction>>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
) {
    if pause_action_state.just_pressed(PauseAction::Pause) {
        match state.get() {
            AppState::Active => {
                next_state.set(AppState::Paused);
                rapier_configuration.physics_pipeline_active = false;
            }
            AppState::Paused => {
                next_state.set(AppState::Active);
                rapier_configuration.physics_pipeline_active = true;
            }
            _ => {}
        }
    }
}

fn pause_screen(mut commands: Commands, ui: Res<UiAssets>) {
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
                states: vec![AppState::Paused],
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "Pause",
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
