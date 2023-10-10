use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod ship;

use self::ship::ShipPlugin;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0),
            ShipPlugin,
        ));

        app.add_systems(Startup, setup);
    }
}

/// The setup function
fn setup(mut rapier_configuration: ResMut<RapierConfiguration>) {
    rapier_configuration.gravity = Vec2::ZERO;
}
