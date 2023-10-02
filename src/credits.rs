use bevy::prelude::*;

use crate::state::{AppState, ForState};

pub struct CreditsPlugin;
impl Plugin for CreditsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Credits), setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                states: vec![AppState::Credits],
            },
            Name::new("Credits"),
        ))
        .with_children(|parent| {
            parent.spawn((TextBundle {
                text: Text::from_section(
                    "Developed by",
                    TextStyle {
                        font: asset_server.load("fonts/kenvector_future.ttf"),
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
                        font: asset_server.load("fonts/kenvector_future.ttf"),
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
                        font: asset_server.load("fonts/kenvector_future.ttf"),
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
                        font: asset_server.load("fonts/kenvector_future.ttf"),
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
                    "Art",
                    TextStyle {
                        font: asset_server.load("fonts/kenvector_future.ttf"),
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
                        font: asset_server.load("fonts/kenvector_future.ttf"),
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
                    "Music",
                    TextStyle {
                        font: asset_server.load("fonts/kenvector_future.ttf"),
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
                        font: asset_server.load("fonts/kenvector_future.ttf"),
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
                        font: asset_server.load("fonts/kenvector_future.ttf"),
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
                        font: asset_server.load("fonts/kenvector_future.ttf"),
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
                        font: asset_server.load("fonts/kenvector_future.ttf"),
                        font_size: 14.0,
                        color: Color::rgb_u8(0xAA, 0xAA, 0xAA),
                    },
                ),
                ..default()
            },));
        });
}
