use bevy::prelude::*;

use crate::core::resources::game_time::GameTime;

/// UI Location component
#[derive(Component)]
pub struct UITime;

#[derive(Bundle)]
pub struct TimeBundle {
    text: TextBundle,
    marker: UITime,
    name: Name,
}
impl TimeBundle {
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
                    "In Space",
                    TextStyle {
                        font: font,
                        font_size: 25.0,
                        color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                        ..default()
                    },
                ),
                ..default()
            },
            marker: UITime,
            name: Name::new("Time"),
        }
    }
}

pub fn current_time(game_time: Res<GameTime>, mut query: Query<&mut Text, With<UITime>>) {
    for mut text in query.iter_mut() {
        let elapsed = game_time.elapsed_secs();

        let hours = (elapsed / 3600.).floor();
        let minutes = (elapsed % 3600. / 60.).floor();
        let seconds = (elapsed % 3600. % 60.).floor();

        let hh = if hours < 10. {
            format!("0{hours}")
        } else {
            hours.to_string()
        };
        let mm = if minutes < 10. {
            format!("0{minutes}")
        } else {
            minutes.to_string()
        };
        let ss = if seconds < 10. {
            format!("0{seconds}")
        } else {
            seconds.to_string()
        };

        text.sections[0].value = format!("{hh}:{mm}:{ss}");
    }
}
