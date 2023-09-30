use bevy::prelude::*;

use super::state::AppState;

use super::Ship;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, follow_player.run_if(in_state(AppState::Active)));
    }
}

fn setup(mut commands: Commands) {
    // Spawns game camera
    commands.spawn(Camera2dBundle::default());
}

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
