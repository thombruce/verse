use bevy::prelude::*;

use crate::resources::{game_time::GameTime, state::GameState};

const ORBITAL_PERIOD_SCALING_FACTOR: f32 = 1.0;

#[derive(Component, Clone, Debug)]
pub struct Orbit {
    pub parent: Option<Entity>,
    pub semi_major_axis: f32,
    // pub eccentricity: f32,
    // pub argument_of_periapsis: f32,
    // pub initial_mean_anomaly: f32,
}

#[derive(Component, Clone, Debug)]
pub struct Orbitable(pub Vec3);
impl Orbitable {
    pub const ZERO: Self = Self(Vec3::ZERO);
}
impl Default for Orbitable {
    fn default() -> Self {
        Self::ZERO
    }
}

pub struct OrbitPlugin;
impl Plugin for OrbitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (orbitable_update_system, orbital_positioning_system)
                .run_if(in_state(GameState::Active)),
        );
    }
}

/// Really basic circular motion around a parent body or [0., 0.]
pub fn orbital_positioning_system(
    game_time: Res<GameTime>,
    mut orbits: Query<(&Orbit, &mut Transform)>,
    orbitables: Query<&Orbitable>,
) {
    for (orbit, mut transform) in orbits.iter_mut() {
        let mut entity_translation = Vec3::ZERO;

        if let Some(parent) = orbit.parent {
            let orbitable = orbitables.get(parent);

            entity_translation = match orbitable {
                Ok(orb) => orb.0,
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

pub fn orbitable_update_system(mut orbitables: Query<(&mut Orbitable, &Transform)>) {
    for (mut orbitable, transform) in orbitables.iter_mut() {
        orbitable.0 = transform.translation;
    }
}
