use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierConfiguration;
use bevy_ui_navigation::prelude::*;
use fluent_content::Content;
use leafwing_input_manager::prelude::{ActionState, InputManagerPlugin};

use crate::{
    core::{effects::blink::DrawBlinkTimer, resources::assets::UiAssets},
    i18n::I18n,
    inputs::{
        menu::MenuAction,
        pause::{pause_input_map, PauseAction},
    },
    systems::states::{ForState, GameState},
};

pub struct PausePlugin;
impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PauseAction>::default());
    }
}

#[derive(Component)]
pub enum MenuButton {
    Continue,
    ExitGame,
}

pub(crate) fn setup_pause_systems(mut commands: Commands) {
    commands.insert_resource(pause_input_map());
    commands.insert_resource(ActionState::<PauseAction>::default());
}

pub(crate) fn pause_system(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    pause_action_state: Res<ActionState<PauseAction>>,
) {
    if pause_action_state.just_pressed(PauseAction::Pause) {
        match state.get() {
            GameState::Active => {
                next_state.set(GameState::Paused);
            }
            GameState::Paused => {
                next_state.set(GameState::Active);
            }
            _ => {}
        }
    }
}

pub(crate) fn toggle_physics_on(mut rapier_configuration: ResMut<RapierConfiguration>) {
    rapier_configuration.physics_pipeline_active = true;
}

pub(crate) fn toggle_physics_off(mut rapier_configuration: ResMut<RapierConfiguration>) {
    rapier_configuration.physics_pipeline_active = false;
}

pub(crate) fn pause_screen(mut commands: Commands, ui: Res<UiAssets>, i18n: Res<I18n>) {
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
                        i18n.content("pause").unwrap().to_ascii_uppercase(),
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

            for (string, marker) in [
                ("continue", MenuButton::Continue),
                ("exit-game", MenuButton::ExitGame),
            ] {
                parent.spawn((
                    TextBundle {
                        text: Text::from_section(
                            i18n.content(string).unwrap().to_ascii_uppercase(),
                            TextStyle {
                                font: ui.font.clone(),
                                font_size: 25.0,
                                color: Color::rgb_u8(0x00, 0x88, 0x88),
                            },
                        ),
                        style: Style {
                            margin: UiRect::top(Val::Px(25.)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    }, // DrawBlinkTimer(Timer::from_seconds(0.65, TimerMode::Repeating)),
                    Focusable::default(),
                    marker,
                ));
            }
        });
}

pub(crate) fn pause_input_system(
    mut next_state: ResMut<NextState<GameState>>,
    inputs: Res<ActionState<MenuAction>>,
    mut requests: EventWriter<NavRequest>,
    mut buttons: Query<&mut MenuButton>,
    mut events: EventReader<NavEvent>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut buttons,
        |mut button| match &mut *button {
            MenuButton::Continue => {
                next_state.set(GameState::Active);
            }
            MenuButton::ExitGame => {
                next_state.set(GameState::Reset);
            }
        },
    );

    if inputs.just_pressed(MenuAction::Select) {
        requests.send(NavRequest::Action);
    }
}
