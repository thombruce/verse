use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::core::resources::state::GameState;

use super::bullet::BulletShipContactEvent;

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub struct MovementSet;

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub struct AttackSet;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum ShipAction {
    Forward,
    RotateLeft,
    RotateRight,
    Fire,
}

/// Ship component
#[derive(Component)]
pub struct Ship {
    /// linear speed in meters per second
    pub thrust: f32,
    /// rotation speed in radians per second
    pub rotation: f32,
    /// staggers firing of bullets
    pub bullet_timer: Timer,
}

#[derive(Component)]
pub struct Health(pub f32);

pub struct ShipPlugin;
impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (bullet_timers_system, (ship_damage).after(AttackSet))
                .run_if(in_state(GameState::Active)),
        );
    }
}

/// Dampening
pub fn dampening(time: &Res<Time>, velocity: &mut Velocity) {
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

fn bullet_timers_system(time: Res<Time>, mut ship: Query<&mut Ship>) {
    for mut ship in ship.iter_mut() {
        ship.bullet_timer.tick(time.delta());
    }
}

fn ship_damage(
    mut commands: Commands,
    mut bullet_ship_contact_events: EventReader<BulletShipContactEvent>,
    mut ship_health: Query<&mut Health, With<Ship>>,
) {
    for event in bullet_ship_contact_events.iter() {
        commands.entity(event.bullet).despawn();

        if let Ok(mut health) = ship_health.get_mut(event.ship) {
            health.0 -= 100.0;
            if health.0 <= 0. {
                commands.entity(event.ship).despawn();
                // TODO: Handle Player ship despawning
                //       Right now, it crashes the game
            }
        }
    }
}
