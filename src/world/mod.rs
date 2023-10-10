use bevy::prelude::*;

pub mod astronomy;
pub mod spatial;

use self::{
    astronomy::{planetary_system::PlanetarySystemPlugin, starfield::StarfieldPlugin},
    spatial::SpatialPlugin,
};

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((StarfieldPlugin, PlanetarySystemPlugin, SpatialPlugin));
    }
}
