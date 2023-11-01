use bevy::{
    asset::LoadState,
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use bevy_fluent::{AssetServerExt, BundleAsset, LocalizationBuilder};
use fluent_content::Content;
use leafwing_input_manager::prelude::{ActionState, InputManagerPlugin};

use crate::{
    core::{
        effects::blink::DrawBlinkTimer,
        resources::assets::{AudioAssets, UiAssets},
    },
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

pub(crate) fn load_start_menu_translations(
    localization_builder: LocalizationBuilder,
    asset_server: Res<AssetServer>,
    mut handles: Local<Option<Vec<Handle<BundleAsset>>>>,
    mut i18n: ResMut<I18n>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let handles =
        handles.get_or_insert_with(|| asset_server.load_glob("locales/**/main.ftl.ron").unwrap());

    let load_state = asset_server.get_group_load_state(handles.iter().map(Handle::id));
    if let LoadState::Loaded = load_state {
        i18n.0 = localization_builder.build(&*handles);
        next_state.set(GameState::StartMenu);
    }
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
                        ..default()
                    },
                    ..default()
                },
                Name::new("Title"),
            ));
            parent.spawn((
                TextBundle {
                    style: Style {
                        margin: UiRect::top(Val::Px(50.)),
                        ..default()
                    },
                    text: Text::from_section(
                        i18n.content("press-start").unwrap().to_ascii_uppercase(),
                        TextStyle {
                            font: ui.font.clone(),
                            font_size: 35.0,
                            color: Color::rgb_u8(0x00, 0x88, 0x88),
                        },
                    ),
                    ..default()
                },
                DrawBlinkTimer(Timer::from_seconds(0.65, TimerMode::Repeating)),
            ));
        });

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::FlexEnd,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                ..default()
            },
            ForState {
                states: vec![GameState::StartMenu],
            },
            Name::new("Hint"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    style: Style {
                        justify_content: JustifyContent::FlexEnd,
                        margin: UiRect {
                            left: Val::Px(10.0),
                            right: Val::Px(10.0),
                            top: Val::Px(10.0),
                            bottom: Val::Px(10.0),
                        },
                        ..default()
                    },
                    text: Text::from_section(
                        i18n.content("credits-prompt").unwrap(),
                        TextStyle {
                            font: ui.font.clone(),
                            font_size: 25.0,
                            color: Color::rgb_u8(0xAA, 0xAA, 0x33),
                            ..default()
                        },
                    ),
                    ..default()
                },
                Name::new("Hint Text"),
            ));
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
