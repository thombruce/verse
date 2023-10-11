use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod dynamic_orbit;
pub mod ship;

use self::{dynamic_orbit::DynamicOrbitPlugin, ship::ShipPlugin};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0),
            ShipPlugin,
            DynamicOrbitPlugin,
        ));

        app.add_systems(Startup, setup);
    }
}

/// The setup function
fn setup(mut rapier_configuration: ResMut<RapierConfiguration>) {
    rapier_configuration.gravity = Vec2::ZERO;
}
