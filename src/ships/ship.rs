use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    core::resources::score::Score,
    systems::{
        events::{BulletShipContactEvent, DeathEvent},
        states::GameState,
    },
};

use super::{
    enemy::{Adversaries, DamagedBy},
    player::Player,
};

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
    mut ship: Query<&mut Health, With<Ship>>,
    mut score: ResMut<Score>,
    player: Query<Entity, (With<Player>, With<Ship>)>,
    mut death_events: EventWriter<DeathEvent>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };

    for event in bullet_ship_contact_events.read() {
        commands.entity(event.bullet).despawn();

        if let Ok(mut health) = ship.get_mut(event.ship) {
            if event.bullet_spawner == player {
                score.0 += 100;
            }

            health.0 -= 100.0;
            if health.0 <= 0. {
                death_events.send(DeathEvent { entity: event.ship });
            }
        }
    }
}

pub(crate) fn ship_death_handling(
    mut commands: Commands,
    mut death_events: EventReader<DeathEvent>,
    player: Query<Entity, (With<Player>, With<Ship>)>,
    mut score: ResMut<Score>,
    adversaries: Query<&Adversaries, With<Ship>>,
) {
    let Ok(player) = player.get_single() else {
        return;
    };

    for event in death_events.read() {
        if event.entity == player {
            commands.entity(event.entity).despawn();
            continue;
        }

        let Ok(adversaries) = adversaries.get(event.entity) else {
            return;
        };

        let player_dealt_damage: u32 = adversaries
            .0
            .iter()
            .filter_map(|a| {
                if a.attacker == player {
                    Some(a.damage)
                } else {
                    None
                }
            })
            .sum();

        // If the destroyed ship is not the player, award XP to the player equal to
        // 10x the damage the player has dealt to the ship.
        score.0 += 10 * player_dealt_damage; // TODO: Make proportionate to player-dealt damage relative to MaxHealth

        commands.entity(event.entity).despawn();
    }
}

pub(crate) fn adversary_system(
    mut bullet_ship_contact_events: EventReader<BulletShipContactEvent>,
    mut ship: Query<&mut Adversaries, With<Ship>>,
) {
    for event in bullet_ship_contact_events.read() {
        if let Ok(mut adversaries) = ship.get_mut(event.ship) {
            adversaries.0.push(DamagedBy {
                attacker: event.bullet_spawner,
                damage: 100,
            });
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
