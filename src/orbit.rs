use bevy::prelude::*;

use crate::game_time::GameTime;

const ORBITAL_PERIOD_SCALING_FACTOR: f32 = 1.0;

#[derive(Component, Clone, Debug)]
pub struct Orbit {
    pub parent: Option<Entity>,
    pub semi_major_axis: f32,
    // pub eccentricity: f32,
    // pub argument_of_periapsis: f32,
    // pub initial_mean_anomaly: f32,
}

/// Really basic circular motion around a parent body or [0., 0.]
pub fn orbital_positioning_system(
    game_time: Res<GameTime>,
    mut orbits: Query<(&Orbit, &mut Transform)>,
    // TODO: Orbiting bodies should be able to orbit other bodies
    //       This might call for use of a ParamSet,
    //       except... I also can't borrow the ParamSet more than once.
    //       Technically, the thing I want below is also in the above,
    //       so it's a question of how do I borrow from the above twice?
    //       This is what copy and clone are for. Look into this.
    entity_query: Query<&Transform, Without<Orbit>>,
) {
    for (orbit, mut transform) in orbits.iter_mut() {
        let mut entity_translation = Vec3::ZERO;

        if let Some(parent) = orbit.parent {
            let entity_transform = entity_query.get(parent);
            entity_translation = match entity_transform {
                Ok(transform) => transform.translation,
                Err(_) => Vec3::ZERO,
            };
        }

        transform.translation.x = entity_translation.x
            + (game_time.elapsed_secs() / orbit.semi_major_axis.sqrt()
                * ORBITAL_PERIOD_SCALING_FACTOR)
                .cos()
                * orbit.semi_major_axis;
        transform.translation.y = entity_translation.y
            + (game_time.elapsed_secs() / orbit.semi_major_axis.sqrt()
                * ORBITAL_PERIOD_SCALING_FACTOR)
                .sin()
                * orbit.semi_major_axis;
    }
}
