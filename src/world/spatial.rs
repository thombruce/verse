use std::time::Duration;

use bevy::prelude::*;
use bevy_spatial::{AutomaticUpdate, SpatialStructure};

#[derive(Component)]
pub struct KDNode;

pub struct SpatialPlugin;
impl Plugin for SpatialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            AutomaticUpdate::<KDNode>::new()
                .with_spatial_ds(SpatialStructure::KDTree2)
                .with_frequency(Duration::from_millis(1)),
        );
    }
}
