use bevy::{math::Vec3Swizzles, prelude::*};

use crate::ships::player::Player;

#[derive(Component, Clone, Debug)]
pub struct Indicated {
    pub color: Color,
}

#[derive(Component, Clone, Debug)]
pub struct Indicator {
    entity: Entity,
}

#[derive(Component, Clone, Debug)]
pub struct Bounds {}

pub(crate) fn spawn_indicators(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    entities_query: Query<(Entity, &Indicated)>,
) {
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
            Name::new("Indicators"),
            Bounds {},
        ))
        .with_children(|parent| {
            for (entity, indicated) in entities_query.iter() {
                parent.spawn((
                    ImageBundle {
                        image: UiImage::new(asset_server.load("images/grey_arrowUpWhite.png")),
                        style: Style {
                            position_type: PositionType::Absolute,
                            width: Val::Px(15.0),
                            height: Val::Px(10.0),
                            ..default()
                        },
                        background_color: BackgroundColor(indicated.color.with_a(0.75)),
                        ..default()
                    },
                    Indicator { entity: entity },
                ));
            }
        });
}

pub(crate) fn indicators_system(
    mut query: Query<(&mut Transform, &mut Style, &Indicator, &mut BackgroundColor)>,
    player_query: Query<&Transform, (With<Player>, Without<Indicator>)>,
    entity_query: Query<(&Transform, &ComputedVisibility, &Indicated), Without<Indicator>>,
    bounds_query: Query<&Node, (With<Bounds>, Without<Indicator>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_translation = player_transform.translation.xy();

        for (mut indicator_transform, mut indicator_style, indicator, mut indicator_color) in
            &mut query
        {
            if let Ok((entity_transform, entity_visibility, indicated)) =
                entity_query.get(indicator.entity)
            {
                if entity_visibility.is_visible() {
                    indicator_style.display = Display::None;
                    continue;
                }

                indicator_style.display = Display::DEFAULT;

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
                let extents = Vec2::from(bounds / 2.0);

                /* Indicator transparency */
                indicator_color.0 = indicated
                    .color
                    .with_a((real_to_entity.length_recip() * 5000.0).clamp(0.01, 1.0));

                /* Circular indicators */
                let normalized = real_to_entity.normalize() * 200.0;
                indicator_style.left = Val::Px(extents.x + normalized.x);
                indicator_style.top = Val::Px(extents.y - normalized.y);

                /* Edge-of-screen indicators */
                // reposition indicator relative to the direction of the entity
                // we're using the non-normalized real_to_entity for this in order
                // to get the actual size of the player-entity vector's x and y values
                // match real_to_entity.x > 0. {
                //     true => {
                //         indicator_style.right = Val::Px((extents.x - real_to_entity.x).max(0.));
                //         indicator_style.left = Val::Auto;
                //     }
                //     false => {
                //         indicator_style.left = Val::Px((extents.x + real_to_entity.x).max(0.));
                //         indicator_style.right = Val::Auto;
                //     }
                // }
                // match real_to_entity.y > 0. {
                //     true => {
                //         indicator_style.top = Val::Px((extents.y - real_to_entity.y).max(0.));
                //         indicator_style.bottom = Val::Auto;
                //     }
                //     false => {
                //         indicator_style.bottom = Val::Px((extents.y + real_to_entity.y).max(0.));
                //         indicator_style.top = Val::Auto;
                //     }
                // }
            }
        }
    }
}

pub(crate) fn despawn_indicators_system(
    mut commands: Commands,
    mut query: Query<(Entity, &Indicator)>,
    entity_query: Query<(&Transform, &ComputedVisibility), (With<Indicated>, Without<Indicator>)>,
) {
    for (entity, indicator) in &mut query {
        if let Err(_) = entity_query.get(indicator.entity) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
