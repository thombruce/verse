use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_spatial::{kdtree::KDTree2, SpatialAccess};
use fluent_content::Content;
use regex::Regex;

use crate::{i18n::I18n, ships::player::Player, world::spatial::KDNode};

/// UI Location component
#[derive(Component)]
pub struct UILocation;

#[derive(Bundle)]
pub struct NavBundle {
    text: TextBundle,
    marker: UILocation,
    name: Name,
}
impl NavBundle {
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
                    // TODO: How do we access translations in a bundle?
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
            marker: UILocation,
            name: Name::new("Location"),
        }
    }
}

pub fn current_location(
    mut query: Query<&mut Text, With<UILocation>>,
    player: Query<&Transform, With<Player>>,
    tree: Res<KDTree2<KDNode>>,
    nodes: Query<&Name, With<KDNode>>,
    i18n: Res<I18n>,
) {
    if let Ok(ship_transform) = player.get_single() {
        let player_translation = ship_transform.translation.xy();

        if let Some((_pos, entity)) = tree.nearest_neighbour(player_translation) {
            let node = nodes.get(entity.unwrap());
            let celestial = &node.unwrap().to_ascii_lowercase();
            let request = format!("near?celestial={}", celestial);

            // NOTE: We need to find and replace the unicode isolation characters added by Fluent
            let regex = Regex::new(r"(\u2068|\u2069)").unwrap();

            for mut text in query.iter_mut() {
                let translation = i18n.content(&request).unwrap();
                let parsed = regex.replace_all(translation.as_str(), "");
                text.sections[0].value = parsed.to_ascii_uppercase();
            }
        }
    }
}
