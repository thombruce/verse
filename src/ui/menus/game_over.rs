use bevy::{app::AppExit, prelude::*};
use bevy_ui_navigation::prelude::*;
use fluent_content::Content;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    core::{effects::blink::DrawBlinkTimer, resources::assets::UiAssets},
    i18n::I18n,
    inputs::menu::MenuAction,
    systems::states::{ForState, GameState},
};

#[derive(Component)]
pub enum MenuButton {
    ExitGame,
    Quit,
}

pub(crate) fn game_over_screen(mut commands: Commands, ui: Res<UiAssets>, i18n: Res<I18n>) {
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
                states: vec![GameState::GameOver],
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        i18n.content("game-over").unwrap().to_ascii_uppercase(),
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
                ("exit-game", MenuButton::ExitGame),
                ("quit", MenuButton::Quit),
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

pub(crate) fn game_over_input_system(
    mut next_state: ResMut<NextState<GameState>>,
    inputs: Res<ActionState<MenuAction>>,
    mut requests: EventWriter<NavRequest>,
    mut buttons: Query<&mut MenuButton>,
    mut events: EventReader<NavEvent>,
    mut exit: EventWriter<AppExit>,
) {
    events.nav_iter().activated_in_query_foreach_mut(
        &mut buttons,
        |mut button| match &mut *button {
            MenuButton::ExitGame => {
                next_state.set(GameState::Reset);
            }
            MenuButton::Quit => {
                exit.send(AppExit);
            }
        },
    );

    if inputs.just_pressed(MenuAction::Select) {
        requests.send(NavRequest::Action);
    }
}
