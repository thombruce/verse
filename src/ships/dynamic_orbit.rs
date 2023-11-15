use bevy::prelude::*;
use bevy_rapier2d::prelude::ExternalImpulse;

use crate::world::{
    astronomy::orbit::{Mass, GRAVITATIONAL_CONSTANT},
    spatial::KDNode,
};

#[derive(Component)]
pub struct Gravitable;

pub fn dynamic_orbital_positioning_system(
    time: Res<Time>,
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
            let mut max_impulse = 1_500_000.0;

            if r > 1_500.0 {
                // TODO: Set r according to actual visible radius of gravitational bodies
                //       maybe multiplied by some factor, e.g. 1.5
                // TODO: Explore setting gravitation by kind (star, planet, moon).
                m = m.powf(1.0 / 1.5);
                max_impulse = 1_200_000.0;
            }

            // GM/r^2
            if r > 0.0 {
                impulse.impulse += (mass_translation - ship_translation).normalize()
                    * (GRAVITATIONAL_CONSTANT * m / r.powf(2.0)).min(max_impulse)
                    * time.delta_seconds();
            }
        }
    }
}
