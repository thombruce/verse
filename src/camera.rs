use bevy::prelude::*;

use super::Ship;

pub fn follow_player(
    mut camera: Query<&mut Transform, (With<Camera>, Without<Ship>)>,
    player: Query<&Transform, (With<Ship>, Without<Camera>)>,
) {
    let mut camera_transform = camera.single_mut();
    let player_transform = player.single();

    let pos = player_transform.translation;

    camera_transform.translation.x = pos.x;
    camera_transform.translation.y = pos.y;
}
