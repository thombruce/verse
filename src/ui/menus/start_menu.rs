use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
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
        app.add_plugins(InputManagerPlugin::<MenuAction>::default());
    }
}

pub(crate) fn init_start_menu(mut commands: Commands, audios: Res<AudioAssets>) {
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

            for string in ["new-game", "settings", "credits", "quit"] {
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            margin: UiRect::top(Val::Px(25.)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BackgroundColor(Color::Rgba {
                            red: 0.,
                            green: 0.,
                            blue: 0.,
                            alpha: 0.,
                        }),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(
                                i18n.content(string).unwrap().to_ascii_uppercase(),
                                TextStyle {
                                    font: ui.font.clone(),
                                    font_size: 25.0,
                                    color: Color::rgb_u8(0x00, 0x88, 0x88),
                                },
                            ), // DrawBlinkTimer(Timer::from_seconds(0.65, TimerMode::Repeating)),
                        );
                    });
            }
        });
}

pub(crate) fn menu_input_system(
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    inputs: Res<ActionState<MenuAction>>,
) {
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
