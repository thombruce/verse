use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::state::AppState;

use crate::Ship;

/// UI Speed component
#[derive(Component)]
pub struct UISpeed {}

pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameCreate), setup);
        app.add_systems(Update, hud_speedometer.run_if(in_state(AppState::Active)));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((NodeBundle {
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
        },))
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
                        "0",
                        TextStyle {
                            font: asset_server.load("fonts/kenvector_future.ttf"),
                            font_size: 25.0,
                            color: Color::rgb_u8(0xAA, 0xAA, 0x33),
                            ..default()
                        },
                    ),
                    ..default()
                },
                UISpeed {},
            ));
        });

    commands
        .spawn((NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::FlexEnd,
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        },))
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
                    "In Space",
                    TextStyle {
                        font: asset_server.load("fonts/kenvector_future.ttf"),
                        font_size: 25.0,
                        color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                        ..default()
                    },
                ),
                ..default()
            },));
        });

    // TODO: Move me.
    commands.spawn(AudioBundle {
        source: asset_server.load("sound/Kirk Osamayo - Space Dust.ogg"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            volume: Volume::new_absolute(0.5),
            ..default()
        },
    });
}

pub fn hud_speedometer(
    mut query: Query<&mut Text, With<UISpeed>>,
    mut player: Query<&Velocity, With<Ship>>,
) {
    let velocity = player.single_mut();

    for mut text in query.iter_mut() {
        text.sections[0].value = format!(
            "{} m/s",
            ((velocity.linvel.x.powf(2.0) + velocity.linvel.y.powf(2.0)).sqrt()).trunc()
        );
    }
}
