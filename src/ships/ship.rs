use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum ShipAction {
    Forward,
    RotateLeft,
    RotateRight,
}

/// Ship component
#[derive(Component)]
pub struct Ship {
    /// linear speed in meters per second
    pub thrust: f32,
    /// rotation speed in radians per second
    pub rotation: f32,
}

/// Dampening
pub fn dampening(time: Res<Time>, velocity: &mut Velocity) {
    // TODO: Dampening should not affect gravitational attraction (dynamic_orbits.rs)
    // TODO: Use Rapier Damping: https://rapier.rs/docs/user_guides/bevy_plugin/rigid_bodies#damping
    let elapsed = time.delta_seconds();
    velocity.angvel *= 0.1f32.powf(elapsed);
    velocity.linvel *= 0.8f32.powf(elapsed);
}

pub fn ship_rotation(rotation_factor: f32, velocity: &mut Velocity, ship: &Ship) {
    // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    if rotation_factor != 0.0 {
        velocity.angvel += rotation_factor * ship.rotation / 60.0;
    }
}

pub fn ship_thrust(
    impulse: &mut ExternalImpulse,
    transform: &Transform,
    thrust_factor: f32,
    ship: &Ship,
) {
    impulse.impulse += (transform.rotation * (Vec3::Y * thrust_factor * ship.thrust)).truncate();
}
