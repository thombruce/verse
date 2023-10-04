use bevy::prelude::*;

use crate::game_time::GameTime;

const ORBITAL_PERIOD_SCALING_FACTOR: f32 = 1.0;

#[derive(Component, Clone, Debug)]
pub struct Orbit {
    pub semi_major_axis: f32,
    // pub eccentricity: f32,
    // pub argument_of_periapsis: f32,
    // pub initial_mean_anomaly: f32,
}

/// Really basic circular motion around (0., 0.) or parent entity
pub fn orbital_positioning_system(
    game_time: Res<GameTime>,
    mut orbits: Query<(&Orbit, &mut Transform)>,
) {
    for (orbit, mut transform) in orbits.iter_mut() {
        transform.translation.x = (game_time.elapsed_secs() / orbit.semi_major_axis.sqrt()
            * ORBITAL_PERIOD_SCALING_FACTOR)
            .cos()
            * orbit.semi_major_axis;
        transform.translation.y = (game_time.elapsed_secs() / orbit.semi_major_axis.sqrt()
            * ORBITAL_PERIOD_SCALING_FACTOR)
            .sin()
            * orbit.semi_major_axis;
    }
}
