use bevy::prelude::*;

use crate::ships::{player::Player, ship::Health};

/// UI Speed component
#[derive(Component)]
pub struct UIHealth;

#[derive(Bundle)]
pub struct HealthBundle {
    text: TextBundle,
    marker: UIHealth,
    name: Name,
}
impl HealthBundle {
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
                        color: Color::rgb_u8(0x33, 0xAA, 0x33),
                        ..default()
                    },
                ),
                ..default()
            },
            marker: UIHealth,
            name: Name::new("Health"),
        }
    }
}

pub fn hud_health(
    mut query: Query<&mut Text, With<UIHealth>>,
    mut player: Query<&Health, With<Player>>,
) {
    if let Ok(health) = player.get_single_mut() {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{} HP", health.0).to_ascii_uppercase();
        }
    } else {
        for mut text in query.iter_mut() {
            text.sections[0].value = format!("{} HP", 0).to_ascii_uppercase();
            text.sections[0].style.color = Color::rgb_u8(0xAA, 0x33, 0x33)
        }
    }
}
