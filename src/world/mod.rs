use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod astronomy;
pub mod ship;
pub mod spatial;

use self::{
    astronomy::{planetary_system::PlanetarySystemPlugin, starfield::StarfieldPlugin},
    ship::ShipPlugin,
    spatial::SpatialPlugin,
};

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0),
            StarfieldPlugin,
            ShipPlugin,
            PlanetarySystemPlugin,
            SpatialPlugin,
        ));

        app.add_systems(Startup, setup);
    }
}

/// The setup function
fn setup(mut rapier_configuration: ResMut<RapierConfiguration>) {
    rapier_configuration.gravity = Vec2::ZERO;
}
