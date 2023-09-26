use bevy::prelude::*;

use super::TIME;

/// Ship component
#[derive(Component)]
pub struct Ship {
    /// linear speed in meters per second
    pub speed: f32,
    /// rotation speed in radians per second
    pub rotation: f32,
}

pub fn ship_flight_system(keys: Res<Input<KeyCode>>, mut query: Query<(&Ship, &mut Transform)>) {
    let (ship, mut transform) = query.single_mut();

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        movement_factor += 1.0;
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        rotation_factor -= 1.0;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        rotation_factor += 1.0;
    }

    // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    transform.rotate_z(rotation_factor * ship.rotation * TIME);

    // get the ship's forward vector by applying the current rotation to the ships initial facing vector
    let movement_direction = transform.rotation * Vec3::Y;
    // get the distance the ship will move based on direction, the ship's movement speed and delta time
    let movement_distance = movement_factor * ship.speed * TIME;
    // create the change in translation using the new movement direction and distance
    let translation_delta = movement_direction * movement_distance;

    // update the ship translation with our new translation delta
    transform.translation += translation_delta;
}
