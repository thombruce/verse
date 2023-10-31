use bevy::prelude::*;

#[allow(unused_imports)]
use crate::shaders::{
    chromatic_aberration::{ChromaticAberrationPlugin, ChromaticAberrationSettings},
    pixelate::{PixelatePlugin, PixelateSettings},
};

use crate::ships::player::Player;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, _app: &mut App) {
        // app.add_plugins(PixelatePlugin);
        // app.add_plugins(ChromaticAberrationPlugin);
    }
}

pub(crate) fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            // TODO: This should always be matched to the player position on load
            //       Otherwise, we see a quick flash of the Sun at (0., 0.)
            //       before the follow_player logic kicks in.
            transform: Transform {
                translation: Vec3::new(0., -6000., 100.0),
                ..default()
            },
            projection: OrthographicProjection {
                scale: 1.5,
                ..default()
            },
            ..default()
        },
        PixelateSettings {
            block_size: 3.25,
            ..default()
        },
        ChromaticAberrationSettings {
            intensity: 0.001,
            ..default()
        },
        Name::new("Main Camera"),
    ));
}

pub(crate) fn follow_player(
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    player: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let mut camera_transform = camera.single_mut();

    if let Ok(player_transform) = player.get_single() {
        let pos = player_transform.translation;

        camera_transform.translation.x = pos.x;
        camera_transform.translation.y = pos.y;
    }
}
