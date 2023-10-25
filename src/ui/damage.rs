use bevy::prelude::*;

use crate::{
    core::resources::{assets::UiAssets, state::GameState},
    ships::{
        bullet::BulletShipContactEvent,
        ship::{AttackSet, Ship},
    },
};

pub struct UiDamagePlugin;
impl Plugin for UiDamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (ui_spawn_damage, ui_update_damage)
                .after(AttackSet)
                .run_if(in_state(GameState::Active)),
        );
    }
}

fn ui_spawn_damage(
    mut commands: Commands,
    mut bullet_ship_contact_events: EventReader<BulletShipContactEvent>,
    ship_transform: Query<&Transform, With<Ship>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    ui: Res<UiAssets>,
) {
    for event in bullet_ship_contact_events.iter() {
        if let Ok(transform) = ship_transform.get(event.ship) {
            // Use camera.world_to_viewport() and camera GlobalTransform to translate
            // a world position into UI coordinates
            let (camera, camera_transform) = camera_query.single();
            let coords = camera
                .world_to_viewport(camera_transform, transform.translation)
                .unwrap();

            commands.spawn(TextBundle {
                text: Text::from_section(
                    "100",
                    TextStyle {
                        font: ui.font.clone(),
                        font_size: 25.0,
                        color: Color::RED,
                        ..default()
                    },
                ),
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(coords.y),
                    left: Val::Px(coords.x),
                    ..default()
                },
                ..default()
            });
        }
    }
}

fn ui_update_damage() {
    // TODO: Update damage text position
    // TODO: Despawn damage on a timer (maybe reuse the bullet despawn timer for this?)
}
