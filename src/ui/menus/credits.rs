use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
};
use bevy_common_assets::ron::RonAssetPlugin;

use crate::{
    core::resources::{
        assets::{DataAssets, UiAssets},
        state::{ForState, GameState},
    },
    ui::resources::top::Top,
};

pub struct CreditsPlugin;
impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<Credits>::new(&["credits.ron"]));
        app.add_systems(OnEnter(GameState::Credits), setup);
        app.add_systems(Update, credits_system.run_if(in_state(GameState::Credits)));
    }
}

#[derive(serde::Deserialize, TypeUuid, TypePath)]
#[uuid = "6763db47-17ca-4530-b604-94492c3a4c58"]
pub struct Credits {
    developed_by: String,
    title_font: Credit,
    art: Vec<Credit>,
    music: Vec<Credit>,
    audio: Vec<Credit>,
}

#[derive(serde::Deserialize)]
pub struct Credit {
    credit_title: String,
    credit_meta: String,
}

#[derive(Component)]
pub struct Scrolling;

fn setup(
    mut commands: Commands,
    ui: Res<UiAssets>,
    data: Res<DataAssets>,
    credits: ResMut<Assets<Credits>>,
) {
    let credits = credits.get(&data.credits.clone()).unwrap();

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
                states: vec![GameState::Credits],
            },
            Name::new("Credits"),
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0),
                            // height: Val::Percent(100.0),
                            align_items: AlignItems::Center,
                            justify_content: JustifyContent::Center,
                            flex_direction: FlexDirection::Column,
                            position_type: PositionType::Absolute,
                            ..default()
                        },
                        ..default()
                    },
                    Scrolling,
                    // TODO: Initial value should be window Y + 25.0
                    Top(1000.0),
                ))
                .with_children(|parent| {
                    parent.spawn((TextBundle {
                        text: Text::from_section(
                            "Developed by".to_ascii_uppercase(),
                            TextStyle {
                                font: ui.font.clone(),
                                font_size: 24.0,
                                color: Color::rgb_u8(0x00, 0x88, 0x88),
                            },
                        ),
                        ..default()
                    },));
                    parent.spawn((TextBundle {
                        style: Style {
                            margin: UiRect::top(Val::Px(25.)),
                            ..default()
                        },
                        text: Text::from_section(
                            &credits.developed_by,
                            TextStyle {
                                font: ui.font.clone(),
                                font_size: 20.0,
                                color: Color::rgb_u8(0xCC, 0xCC, 0xCC),
                            },
                        ),
                        ..default()
                    },));
                    parent.spawn((TextBundle {
                        style: Style {
                            margin: UiRect::top(Val::Px(50.)),
                            ..default()
                        },
                        text: Text::from_section(
                            "Title Font".to_ascii_uppercase(),
                            TextStyle {
                                font: ui.font.clone(),
                                font_size: 24.0,
                                color: Color::rgb_u8(0x00, 0x88, 0x88),
                            },
                        ),
                        ..default()
                    },));
                    parent.spawn((TextBundle {
                        style: Style {
                            margin: UiRect::top(Val::Px(25.)),
                            ..default()
                        },
                        text: Text::from_section(
                            &credits.title_font.credit_title,
                            TextStyle {
                                font: ui.font.clone(),
                                font_size: 20.0,
                                color: Color::rgb_u8(0xCC, 0xCC, 0xCC),
                            },
                        ),
                        ..default()
                    },));
                    parent.spawn((TextBundle {
                        text: Text::from_section(
                            &credits.title_font.credit_meta,
                            TextStyle {
                                font: ui.font.clone(),
                                font_size: 14.0,
                                color: Color::rgb_u8(0xAA, 0xAA, 0xAA),
                            },
                        ),
                        ..default()
                    },));
                    parent.spawn((TextBundle {
                        style: Style {
                            margin: UiRect::top(Val::Px(50.)),
                            ..default()
                        },
                        text: Text::from_section(
                            "Art".to_ascii_uppercase(),
                            TextStyle {
                                font: ui.font.clone(),
                                font_size: 24.0,
                                color: Color::rgb_u8(0x00, 0x88, 0x88),
                            },
                        ),
                        ..default()
                    },));
                    for credit in credits.art.iter() {
                        parent.spawn((TextBundle {
                            style: Style {
                                margin: UiRect::top(Val::Px(25.)),
                                ..default()
                            },
                            text: Text::from_section(
                                &credit.credit_title,
                                TextStyle {
                                    font: ui.font.clone(),
                                    font_size: 20.0,
                                    color: Color::rgb_u8(0xCC, 0xCC, 0xCC),
                                },
                            ),
                            ..default()
                        },));
                        parent.spawn((TextBundle {
                            text: Text::from_section(
                                &credit.credit_meta,
                                TextStyle {
                                    font: ui.font.clone(),
                                    font_size: 14.0,
                                    color: Color::rgb_u8(0xAA, 0xAA, 0xAA),
                                },
                            ),
                            ..default()
                        },));
                    }
                    parent.spawn((TextBundle {
                        style: Style {
                            margin: UiRect::top(Val::Px(50.)),
                            ..default()
                        },
                        text: Text::from_section(
                            "Music".to_ascii_uppercase(),
                            TextStyle {
                                font: ui.font.clone(),
                                font_size: 24.0,
                                color: Color::rgb_u8(0x00, 0x88, 0x88),
                            },
                        ),
                        ..default()
                    },));
                    for credit in credits.music.iter() {
                        parent.spawn((TextBundle {
                            style: Style {
                                margin: UiRect::top(Val::Px(25.)),
                                ..default()
                            },
                            text: Text::from_section(
                                &credit.credit_title,
                                TextStyle {
                                    font: ui.font.clone(),
                                    font_size: 20.0,
                                    color: Color::rgb_u8(0xCC, 0xCC, 0xCC),
                                },
                            ),
                            ..default()
                        },));
                        parent.spawn((TextBundle {
                            text: Text::from_section(
                                &credit.credit_meta,
                                TextStyle {
                                    font: ui.font.clone(),
                                    font_size: 14.0,
                                    color: Color::rgb_u8(0xAA, 0xAA, 0xAA),
                                },
                            ),
                            ..default()
                        },));
                    }
                    parent.spawn((TextBundle {
                        style: Style {
                            margin: UiRect::top(Val::Px(50.)),
                            ..default()
                        },
                        text: Text::from_section(
                            "Audio".to_ascii_uppercase(),
                            TextStyle {
                                font: ui.font.clone(),
                                font_size: 24.0,
                                color: Color::rgb_u8(0x00, 0x88, 0x88),
                            },
                        ),
                        ..default()
                    },));
                    for credit in credits.audio.iter() {
                        parent.spawn((TextBundle {
                            style: Style {
                                margin: UiRect::top(Val::Px(25.)),
                                ..default()
                            },
                            text: Text::from_section(
                                &credit.credit_title,
                                TextStyle {
                                    font: ui.font.clone(),
                                    font_size: 20.0,
                                    color: Color::rgb_u8(0xCC, 0xCC, 0xCC),
                                },
                            ),
                            ..default()
                        },));
                        parent.spawn((TextBundle {
                            text: Text::from_section(
                                &credit.credit_meta,
                                TextStyle {
                                    font: ui.font.clone(),
                                    font_size: 14.0,
                                    color: Color::rgb_u8(0xAA, 0xAA, 0xAA),
                                },
                            ),
                            ..default()
                        },));
                    }
                });
        });
}

fn credits_system(
    time: Res<Time>,
    mut query: Query<(&mut Style, &mut Top, &Node), With<Scrolling>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (mut style, mut top, node) in query.iter_mut() {
        top.0 -= 50.0 * time.delta_seconds();
        style.top = Val::Px(top.0);

        // Returns to StartMenu when credits have rolled
        // outside of the upper-bounds of the screen
        if top.0 <= -(node.size().y + 25.0) {
            next_state.set(GameState::StartMenu);
        }
    }
}
