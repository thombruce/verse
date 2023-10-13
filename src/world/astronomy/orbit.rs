use std::f32::consts::{PI, TAU};

use bevy::prelude::*;

use crate::core::resources::{game_time::GameTime, state::GameState};

pub const GRAVITATIONAL_CONSTANT: f32 = 6.67430e-11; // https://en.wikipedia.org/wiki/Gravitational_constant#Modern_value
pub const ORBITAL_PERIOD_SCALING_FACTOR: f32 = 1.0e-15;

#[derive(Component, Clone, Debug)]
pub struct Orbit {
    pub parent: Option<Entity>,
    pub semi_major_axis: f32,       // Meters
    pub eccentricity: f32,          // Circular: 0., Elliptic: < 1., Parabolic: 1., Hyperbolic: > 1.
    pub argument_of_periapsis: f32, // Radians
    pub initial_mean_anomaly: f32,  // Radians
}
impl Default for Orbit {
    fn default() -> Self {
        Self {
            parent: None,
            semi_major_axis: 0.,
            eccentricity: 0.,
            argument_of_periapsis: 0.,
            initial_mean_anomaly: 0.,
        }
    }
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

        let pos = position_at_time(
            orbit.semi_major_axis,
            orbit.eccentricity,
            orbit.argument_of_periapsis,
            orbit.initial_mean_anomaly,
            // TODO: Set mass on parent Orbitable component
            1.989e30 * ORBITAL_PERIOD_SCALING_FACTOR, // parent_mass.mass,
            game_time.elapsed_secs(),
        );
        transform.translation = entity_translation + Vec3::from(pos);
    }
}

pub fn orbitable_update_system(mut orbitables: Query<(&mut Orbitable, &Transform)>) {
    for (mut orbitable, transform) in orbitables.iter_mut() {
        orbitable.0 = transform.translation;
    }
}

/* Math */

fn position_at_time(
    semi_major_axis: f32,
    eccentricity: f32,
    argument_of_periapsis: f32,
    initial_mean_anomaly: f32,
    parent_mass: f32,
    time: f32,
) -> (f32, f32, f32) {
    let period = period(semi_major_axis, parent_mass);
    let mean_motion = mean_motion(period);
    let mean_anomaly = mean_anomaly(mean_motion, initial_mean_anomaly, time);
    let eccentric_anomaly = eccentric_anomaly(eccentricity, mean_anomaly);
    let true_anomaly = true_anomaly(eccentricity, eccentric_anomaly);
    let heliocentric_distance = heliocentric_distance(semi_major_axis, eccentricity, true_anomaly);

    position(
        true_anomaly,
        heliocentric_distance,
        argument_of_periapsis,
        mean_anomaly,
    )
}

fn period(semi_major_axis: f32, parent_mass: f32) -> f32 {
    TAU * (semi_major_axis.powi(3) / (GRAVITATIONAL_CONSTANT * parent_mass)).sqrt()
}

fn mean_motion(period: f32) -> f32 {
    TAU / period
}

fn mean_anomaly(mean_motion: f32, initial_mean_anomaly: f32, time: f32) -> f32 {
    (initial_mean_anomaly + mean_motion * time).rem_euclid(TAU)
}

fn eccentric_anomaly(eccentricity: f32, mean_anomaly: f32) -> f32 {
    let e = eccentricity;
    let ma = mean_anomaly;
    let mut ea = ma;
    // Uses Newton's method to estimate the eccentric anomaly
    for _i in 0..5 {
        ea = ea - (ea - e * ea.sin() - ma) / (1.0 - e * ea.cos());
    }
    ea
}

fn true_anomaly(eccentricity: f32, eccentric_anomaly: f32) -> f32 {
    let e = eccentricity;
    let e_a = eccentric_anomaly;
    2.0 * (((1.0 + e) / (1.0 - e) * ((e_a / 2.0).tan()).powi(2)).sqrt()).atan()
}

fn heliocentric_distance(semi_major_axis: f32, eccentricity: f32, true_anomaly: f32) -> f32 {
    let semilatus_rectum = semi_major_axis * (1.0 - eccentricity.powi(2));
    semilatus_rectum / (1.0 + eccentricity * true_anomaly.cos())
}

fn position(
    true_anomaly: f32,
    heliocentric_distance: f32,
    argument_of_periapsis: f32,
    mean_anomaly: f32,
) -> (f32, f32, f32) {
    let ymod = if (mean_anomaly % TAU) < PI { -1.0 } else { 1.0 };

    let x = heliocentric_distance * true_anomaly.cos();
    let y = heliocentric_distance * true_anomaly.sin() * ymod;

    let rotated_x = x * argument_of_periapsis.cos() - y * argument_of_periapsis.sin();
    let rotated_y = x * argument_of_periapsis.sin() + y * argument_of_periapsis.cos();

    (rotated_x, rotated_y, 0.)
}
