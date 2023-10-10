use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::Velocity;
use bevy_spatial::{kdtree::KDTree2, SpatialAccess};

use crate::{
    astronomy::orbit::Orbitable, resources::assets::UiAssets, resources::state::GameState,
    ship::Ship,
};

pub mod indicator;

use indicator::IndicatorPlugin;

/// UI Speed component
#[derive(Component)]
pub struct UISpeed {}

/// UI Location component
#[derive(Component)]
pub struct UILocation {}

pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(IndicatorPlugin);
        app.add_systems(OnEnter(GameState::GameCreate), setup);
        app.add_systems(
            Update,
            (hud_speedometer, current_location).run_if(in_state(GameState::Active)),
        );
    }
}

fn setup(mut commands: Commands, ui: Res<UiAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::SpaceBetween,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            Name::new("HUD"),
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
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
                            font: ui.font.clone(),
                            font_size: 25.0,
                            color: Color::rgb_u8(0x00, 0xAA, 0xAA),
                            ..default()
                        },
                    ),
                    ..default()
                },
                UILocation {},
                Name::new("Location"),
            ));

            parent.spawn((
                TextBundle {
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
                            font: ui.font.clone(),
                            font_size: 25.0,
                            color: Color::rgb_u8(0xAA, 0xAA, 0x33),
                            ..default()
                        },
                    ),
                    ..default()
                },
                UISpeed {},
                Name::new("Speedometer"),
            ));
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

pub fn current_location(
    mut query: Query<&mut Text, With<UILocation>>,
    player: Query<&Transform, With<Ship>>,
    tree: Res<KDTree2<Orbitable>>,
    orbitables: Query<&Name, With<Orbitable>>,
) {
    let ship_transform = player.single();

    let player_translation = ship_transform.translation.xy();

    if let Some((_pos, entity)) = tree.nearest_neighbour(player_translation) {
        let orbitable = orbitables.get(entity.unwrap());

        for mut text in query.iter_mut() {
            text.sections[0].value = format!("Near {}", orbitable.unwrap());
        }
    }
}
