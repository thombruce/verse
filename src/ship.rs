use bevy::prelude::*;
use bevy_rapier2d::prelude::{ExternalImpulse, Velocity};

/// Ship component
#[derive(Component)]
pub struct Ship {
    /// linear speed in meters per second
    pub thrust: f32,
    /// rotation speed in radians per second
    pub rotation: f32,
}

pub fn ship_flight_system(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&Ship, &Transform, &mut Velocity, &mut ExternalImpulse)>,
) {
    let (ship, transform, mut velocity, mut impulse) = query.single_mut();

    // Dampening
    let elapsed = time.delta_seconds();
    velocity.angvel *= 0.1f32.powf(elapsed);
    velocity.linvel *= 0.8f32.powf(elapsed);

    // Controls
    let mut rotation_factor = 0.0;
    let mut thrust_factor = 0.0;

    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        thrust_factor += 1.0;
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        rotation_factor -= 1.0;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        rotation_factor += 1.0;
    }

    // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    if rotation_factor != 0.0 {
        velocity.angvel += rotation_factor * ship.rotation / 60.0;
    }

    impulse.impulse = (transform.rotation * (Vec3::Y * thrust_factor * ship.thrust)).truncate();
}
