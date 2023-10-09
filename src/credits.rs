use bevy::prelude::*;

use crate::resources::{
    assets::UiAssets,
    state::{ForState, GameState},
};

pub struct CreditsPlugin;
impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Credits), setup);
    }
}

fn setup(mut commands: Commands, ui: Res<UiAssets>) {
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
            parent.spawn((TextBundle {
                text: Text::from_section(
                    "Developed by",
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
                    "Thom Bruce",
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
                    "Title Font",
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
                    "Edge of the Galaxy by Quinn Davis Type",
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
                    "FontSpace, Public Domain",
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
                    "Art",
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
                    "Space Shooter Redux by Kenney",
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
                    "kenney.nl, CC0",
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
                    margin: UiRect::top(Val::Px(25.)),
                    ..default()
                },
                text: Text::from_section(
                    "Pixel Planets by Deep-Fold",
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
                    "itch.io, MIT",
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
                    "Music",
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
                    "Lightspeed by Beat Mekanik",
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
                    "Free Music Archive, CC BY",
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
                    margin: UiRect::top(Val::Px(25.)),
                    ..default()
                },
                text: Text::from_section(
                    "Space Dust by Kirk Osamayo",
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
                    "Free Music Archive, CC BY",
                    TextStyle {
                        font: ui.font.clone(),
                        font_size: 14.0,
                        color: Color::rgb_u8(0xAA, 0xAA, 0xAA),
                    },
                ),
                ..default()
            },));
        });
}
