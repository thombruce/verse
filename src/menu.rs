use bevy::{audio::PlaybackMode, prelude::*};
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    Actionlike,
};

use crate::state::{is_in_menu_state, AppState};

use crate::effects::DrawBlinkTimer;

#[derive(Component, Debug)]
pub struct StartMenuScreen {}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum MenuAction {
    Start,
    Credits,
}

pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::StartMenu), setup)
            .add_systems(Update, menu_input_system.run_if(is_in_menu_state))
            .add_systems(OnExit(AppState::StartMenu), despawn);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut input_map = InputMap::<MenuAction>::new([
        (KeyCode::Return, MenuAction::Start),
        (KeyCode::C, MenuAction::Credits),
    ]);
    input_map.insert(GamepadButtonType::Start, MenuAction::Start);
    input_map.insert(GamepadButtonType::North, MenuAction::Credits);

    commands.insert_resource(input_map.build());
    commands.insert_resource(ActionState::<MenuAction>::default());

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
            StartMenuScreen {},
        ))
        .with_children(|parent| {
            parent.spawn((ImageBundle {
                image: UiImage::new(asset_server.load("verse.png")),
                style: Style {
                    width: Val::Px(350.0),
                    height: Val::Px(169.4),
                    ..default()
                },
                ..default()
            },));
            parent.spawn((
                TextBundle {
                    style: Style {
                        margin: UiRect::top(Val::Px(50.)),
                        ..default()
                    },
                    text: Text::from_section(
                        "PRESS START",
                        TextStyle {
                            font: asset_server.load("fonts/kenvector_future.ttf"),
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
            StartMenuScreen {},
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
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
                    "Press 'C' for Credits",
                    TextStyle {
                        font: asset_server.load("fonts/kenvector_future.ttf"),
                        font_size: 25.0,
                        color: Color::rgb_u8(0xAA, 0xAA, 0x33),
                        ..default()
                    },
                ),
                ..default()
            },));
        });

    commands.spawn((
        AudioBundle {
            source: asset_server.load("sound/Beat Mekanik - Lightspeed.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
        },
        StartMenuScreen {},
    ));
}

fn despawn(mut commands: Commands, query: Query<Entity, With<StartMenuScreen>>) {
    for entity in &mut query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn menu_input_system(
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    inputs: Res<ActionState<MenuAction>>,
) {
    if inputs.just_pressed(MenuAction::Start) {
        next_state.set(AppState::GameCreate);
    }
    if inputs.just_released(MenuAction::Credits) {
        match state.get() {
            AppState::StartMenu => {
                next_state.set(AppState::Credits);
            }
            AppState::Credits => {
                next_state.set(AppState::StartMenu);
            }
            _ => {}
        }
    }
}
