use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::ExternalImpulse;
use bevy_spatial::{kdtree::KDTree2, SpatialAccess};

use crate::world::{
    astronomy::orbit::{Mass, GRAVITATIONAL_CONSTANT},
    spatial::KDNode,
};

#[derive(Component)]
pub struct Gravitable;

#[allow(dead_code)]
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

pub fn better_dynamic_orbital_positioning_system(
    mut query: Query<(&Transform, &mut ExternalImpulse), With<Gravitable>>,
    masses: Query<(&Mass, &Transform), With<KDNode>>,
) {
    for (transform, mut impulse) in query.iter_mut() {
        // From Nav
        let ship_translation = transform.translation.truncate();

        for (mass, transform) in masses.iter() {
            let mass_translation = transform.translation.truncate();
            let mut m = mass.0;
            let r = mass_translation.distance(ship_translation);

            // GM/r^2
            if r > 1_500.0 {
                // TODO: Set r according to actual visible radius of gravitational bodies
                //       maybe multiplied by some factor, e.g. 1.5
                m = m.powf(1.0 / 1.5);
                impulse.impulse += (mass_translation - ship_translation).normalize()
                    * (GRAVITATIONAL_CONSTANT * m / r.powf(2.0)).min(10_000.0);
            } else if r > 0.0 {
                impulse.impulse += (mass_translation - ship_translation).normalize()
                    * (GRAVITATIONAL_CONSTANT * m / r.powf(2.0)).min(12_500.0);
            }
            // TODO: Explore setting gravitation by kind (star, planet, moon).
        }
    }
}
