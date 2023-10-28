use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::ExternalImpulse;
use bevy_spatial::{kdtree::KDTree2, SpatialAccess};

use crate::{
    core::resources::state::GameState,
    world::{astronomy::orbit::Mass, spatial::KDNode},
};

#[derive(Component)]
pub struct Gravitable;

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
    mut query: Query<(&Transform, &mut ExternalImpulse), With<Gravitable>>,
    masses: Query<&Mass, With<KDNode>>,
) {
    for (transform, mut impulse) in query.iter_mut() {
        // From Nav
        let ship_translation = transform.translation.xy();

        if let Some((pos, entity)) = tree.nearest_neighbour(ship_translation) {
            let mass = masses.get(entity.unwrap());
            if pos.distance(ship_translation) > 1. && pos.distance(ship_translation) < 1500. {
                impulse.impulse += (pos - ship_translation).normalize()
                    * (mass.unwrap().0.powf(1.0 / 7.0).min(9000.0));
                // TODO: That's some clumsy handling of Mass; justify it or change it.
            }
        }
    }
}
