use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    astronomy::{planetary_system::PlanetarySystemPlugin, starfield::StarfieldPlugin},
    resources::spatial::SpatialPlugin,
    ship::ShipPlugin,
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
