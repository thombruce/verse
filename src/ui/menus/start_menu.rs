use bevy::{
    app::AppExit,
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use bevy_ui_navigation::{prelude::*, systems::InputMapping};
use fluent_content::Content;
use leafwing_input_manager::prelude::{ActionState, InputManagerPlugin};

use crate::{
    core::resources::assets::{AudioAssets, UiAssets},
    i18n::I18n,
    inputs::menu::{menu_input_map, MenuAction},
    systems::states::{ForState, GameState},
};

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputManagerPlugin::<MenuAction>::default(),
            DefaultNavigationPlugins,
        ));
    }
}

#[derive(Component)]
pub enum MenuButton {
    NewGame,
    Credits,
    Quit,
}

pub(crate) fn init_start_menu(
    mut commands: Commands,
    audios: Res<AudioAssets>,
    mut input_mapping: ResMut<InputMapping>,
) {
    // TODO: We might prefer to do this part ourselves... maybe.
    input_mapping.keyboard_navigation = true;
    input_mapping.focus_follows_mouse = true;

    commands.insert_resource(menu_input_map());
    commands.insert_resource(ActionState::<MenuAction>::default());

    commands.spawn((
        AudioBundle {
            source: audios.title_music.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                volume: Volume::new_relative(1.0),
                ..default()
            },
        },
        ForState {
            states: GameState::IN_MENU_STATE.to_vec(),
        },
        Name::new("Menu Music"),
    ));
}

pub(crate) fn spawn_start_menu(mut commands: Commands, ui: Res<UiAssets>, i18n: Res<I18n>) {
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
                ..default()
            },
            ForState {
                states: vec![GameState::StartMenu],
            },
            Name::new("Start Menu"),
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    image: UiImage {
                        texture: ui.title.clone(),
                        ..default()
                    },
                    style: Style {
                        width: Val::Px(350.0),
                        height: Val::Px(169.4),
                        margin: UiRect::bottom(Val::Px(25.)),
                        ..default()
                    },
                    ..default()
                },
                Name::new("Title"),
            ));

            for (string, marker) in [
                ("new-game", MenuButton::NewGame),
                ("credits", MenuButton::Credits),
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

pub(crate) fn menu_input_system(
    state: Res<State<GameState>>,
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
            MenuButton::NewGame => {
                next_state.set(GameState::GameCreate);
            }
            MenuButton::Credits => {
                next_state.set(GameState::Credits);
            }
            MenuButton::Quit => {
                exit.send(AppExit);
            }
        },
    );

    if inputs.just_pressed(MenuAction::Select) {
        requests.send(NavRequest::Action);
    }
    if inputs.just_pressed(MenuAction::Start) {
        next_state.set(GameState::GameCreate);
    }
    if inputs.just_released(MenuAction::Credits) {
        match state.get() {
            GameState::StartMenu => {
                next_state.set(GameState::Credits);
            }
            GameState::Credits => {
                next_state.set(GameState::StartMenu);
            }
            _ => {}
        }
    }
}

// TODO: This is very general. It can be moved to a less specific location.
pub(crate) fn menu_focus_system(
    mut interaction_query: Query<(&Focusable, &mut Text), Changed<Focusable>>,
) {
    for (focusable, mut text) in interaction_query.iter_mut() {
        if let FocusState::Focused = focusable.state() {
            text.sections[0].style.color = Color::rgb_u8(0x00, 0x88, 0x88);
        } else {
            text.sections[0].style.color = Color::rgb_u8(0x00, 0x44, 0x44);
        }
    }
}
