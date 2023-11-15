use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::systems::{events::BulletShipContactEvent, states::GameState};

use super::player::Player;

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

pub fn ship_rotation(time: &Res<Time>, rotation_factor: f32, velocity: &mut Velocity, ship: &Ship) {
    // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    if rotation_factor != 0.0 {
        velocity.angvel += rotation_factor * ship.rotation * time.delta_seconds();
    }
}

pub fn ship_thrust(
    time: &Res<Time>,
    impulse: &mut ExternalImpulse,
    transform: &Transform,
    thrust_factor: f32,
    ship: &Ship,
) {
    impulse.impulse +=
        (transform.rotation * (Vec3::Y * thrust_factor * ship.thrust) * time.delta_seconds())
            .truncate();
}

pub(crate) fn bullet_timers_system(time: Res<Time>, mut ship: Query<&mut Ship>) {
    for mut ship in ship.iter_mut() {
        ship.bullet_timer.tick(time.delta());
    }
}

pub(crate) fn ship_damage(
    mut commands: Commands,
    mut bullet_ship_contact_events: EventReader<BulletShipContactEvent>,
    mut ship_health: Query<&mut Health, With<Ship>>,
) {
    for event in bullet_ship_contact_events.read() {
        commands.entity(event.bullet).despawn();

        if let Ok(mut health) = ship_health.get_mut(event.ship) {
            health.0 -= 100.0;
            if health.0 <= 0. {
                commands.entity(event.ship).despawn();
            }
        }
    }
}

pub(crate) fn game_over(
    player: Query<Entity, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if player.is_empty() {
        next_state.set(GameState::GameOver);
    }
}
