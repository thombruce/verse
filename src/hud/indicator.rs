use bevy::{math::Vec3Swizzles, prelude::*};

use crate::{planet::Planet, ship::Ship, state::AppState};

pub struct IndicatorPlugin;
impl Plugin for IndicatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameCreate), setup);
        app.add_systems(Update, indicators_system.run_if(in_state(AppState::Active)));
    }
}

#[derive(Component, Clone, Debug)]
struct Indicator {}

#[derive(Component, Clone, Debug)]
struct Bounds {}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            Name::new("HUD"),
            Bounds {},
        ))
        .with_children(|parent| {
            parent.spawn((
                ImageBundle {
                    image: UiImage::new(asset_server.load("grey_arrowUpWhite.png")),
                    style: Style {
                        position_type: PositionType::Absolute,
                        width: Val::Px(15.0),
                        height: Val::Px(10.0),
                        ..default()
                    },
                    ..default()
                },
                Indicator {},
            ));
        });
}

fn indicators_system(
    mut query: Query<(&mut Transform, &mut Style), With<Indicator>>,
    player_query: Query<&Transform, (With<Ship>, Without<Indicator>)>,
    entity_query: Query<&Transform, (With<Planet>, Without<Indicator>)>,
    bounds_query: Query<&Node, (With<Bounds>, Without<Indicator>)>,
) {
    let player_transform = player_query.single();
    let player_translation = player_transform.translation.xy();

    for (mut indicator_transform, mut indicator_style) in &mut query {
        let entity_transform = entity_query.single();
        let entity_translation = entity_transform.translation.xy();

        // get the vector from the entity to the player ship in 2D and normalize it.
        let real_to_entity = entity_translation - player_translation;
        let to_entity = real_to_entity.normalize();

        // get the quaternion to rotate from the indicator pointing direction to the direction
        // of the entity
        let rotate_to_entity = Quat::from_rotation_arc_2d(to_entity, Vec2::Y);

        // rotate the indicator towards the entity
        indicator_transform.rotation = rotate_to_entity;

        // get the extents of the bounding UI node (size of window)
        let bounds = bounds_query.single().size();
        let extents = Vec3::from((bounds / 2.0, 0.0));

        // reposition indicator relative to the direction of the entity
        // we're using the non-normalized real_to_entity for this in order
        // to get the actual size of the player-entity vector's x and y values
        match real_to_entity.x > 0. {
            true => {
                indicator_style.right = Val::Px((extents.x - real_to_entity.x).max(0.));
                indicator_style.left = Val::Auto;
            }
            false => {
                indicator_style.left = Val::Px((extents.x + real_to_entity.x).max(0.));
                indicator_style.right = Val::Auto;
            }
        }
        match real_to_entity.y > 0. {
            true => {
                indicator_style.top = Val::Px((extents.y - real_to_entity.y).max(0.));
                indicator_style.bottom = Val::Auto;
            }
            false => {
                indicator_style.bottom = Val::Px((extents.y + real_to_entity.y).max(0.));
                indicator_style.top = Val::Auto;
            }
        }
    }
}
