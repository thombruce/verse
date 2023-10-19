use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

use crate::ships::ship::Player;

/// UI Speed component
#[derive(Component)]
pub struct UISpeed;

#[derive(Bundle)]
pub struct SpeedometerBundle {
    text: TextBundle,
    marker: UISpeed,
    name: Name,
}
impl SpeedometerBundle {
    pub fn use_font(font: Handle<Font>) -> Self {
        Self {
            text: TextBundle {
                style: Style {
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
                        font: font,
                        font_size: 25.0,
                        color: Color::rgb_u8(0xAA, 0xAA, 0x33),
                        ..default()
                    },
                ),
                ..default()
            },
            marker: UISpeed,
            name: Name::new("Speedometer"),
        }
    }
}

pub fn hud_speedometer(
    mut query: Query<&mut Text, With<UISpeed>>,
    mut player: Query<&Velocity, With<Player>>,
) {
    let velocity = player.single_mut();

    for mut text in query.iter_mut() {
        text.sections[0].value = format!(
            "{} m/s",
            ((velocity.linvel.x.powf(2.0) + velocity.linvel.y.powf(2.0)).sqrt()).trunc()
        )
        .to_ascii_uppercase();
    }
}
