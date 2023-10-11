use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::ExternalImpulse;
use bevy_spatial::{kdtree::KDTree2, SpatialAccess};

use crate::{core::resources::state::GameState, world::spatial::KDNode};

use super::ship::Ship;

pub struct DynamicOrbitPlugin;
impl Plugin for DynamicOrbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            dynamic_orbital_positioning_system.run_if(in_state(GameState::Active)),
        );
    }
}

pub fn dynamic_orbital_positioning_system(
    tree: Res<KDTree2<KDNode>>,
    mut query: Query<(&Transform, &mut ExternalImpulse), With<Ship>>,
) {
    let (transform, mut impulse) = query.single_mut();

    // From Nav
    let player_translation = transform.translation.xy();

    if let Some((pos, _entity)) = tree.nearest_neighbour(player_translation) {
        impulse.impulse += (pos - player_translation).normalize() * 2000.;
    }
}
