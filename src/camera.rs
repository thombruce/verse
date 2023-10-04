use bevy::prelude::*;

#[allow(unused_imports)]
use crate::shader::{PixelateSettings, ShaderPlugin};

use crate::{ship::Ship, state::AppState};

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        // app.add_plugins(ShaderPlugin);
        app.add_systems(Startup, setup);
        app.add_systems(Update, follow_player.run_if(in_state(AppState::Active)));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),
        PixelateSettings {
            block_size: 3.25,
            ..default()
        },
        Name::new("Main Camera"),
    ));
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
