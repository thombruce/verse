use bevy::prelude::*;

use crate::core::resources::score::Score;

/// UI Score component
#[derive(Component)]
pub struct UIScore;

#[derive(Bundle)]
pub struct ScoreBundle {
    text: TextBundle,
    marker: UIScore,
    name: Name,
}
impl ScoreBundle {
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
                    format!("{:010}", 0),
                    TextStyle {
                        font: font,
                        font_size: 25.0,
                        color: Color::rgb_u8(0xCC, 0xCC, 0xCC),
                        ..default()
                    },
                ),
                ..default()
            },
            marker: UIScore,
            name: Name::new("Score"),
        }
    }
}

pub fn current_score(score: Res<Score>, mut query: Query<&mut Text, With<UIScore>>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("{:010}", score.0);
    }
}
