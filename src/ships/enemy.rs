use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::core::resources::assets::SpriteAssets;
use crate::systems::events::BulletSpawnEvent;
use crate::ui::hud::indicator::Indicated;

use super::dynamic_orbit::Gravitable;
use super::{
    player::Player,
    ship::{ship_rotation, ship_thrust, Health, Ship},
};

/// Enemy component
#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Targeting {
    pub pos: Vec3,
    pub angle: f32,
}

pub struct DamagedBy {
    pub attacker: Entity,
    pub damage: u32,
}
#[derive(Component)]
pub struct Adversaries(pub Vec<DamagedBy>);

// NOTE: We might add a way for the enemy to be healed (healer class drone?)
//       Since the intention is to use the Adversaries damage to determine
//       what percentage of the damage done was by the player... we would
//       want to rethink this to avoid a single enemy being bled for inf%
//       XP more than they ought to be.
//
//       Since we also want the enemy to remember who their greatest adversary
//       has been however, we don't want to delete damage done as it is healed...
//
//       So we could simply cap the damage-based XP reward at 100%. This seems fair.
//       If you, the player, have done 10_000 damage, for example... you should be
//       rewarded for that just the same regardless of whether the enemy has healed
//       and some other adversary has done 30_000... no?
//
//       An alternative approach would be to base the XP on the relative proportionality
//       of damages done. That is, a percentage of the total damage delt rather than of
//       total health (which could differ). But this could result in very little reward
//       for the same amount of work... the other approach seems fairer.
//
//       Imagine if it were a multiplayer scenario. You could technically have both
//       players earn maximum HP by allowing the enemy to heal before finishing it.
//       Does that seem like the right way to go about it? I think so.
//
//       Another scenario: The player has delt some damage, the enemy has gone away and
//       been fully healed and returns as if it was never damaged in the first place...
//       Should this be treated as a new encounter? Or should the previous damage remain?
//       What if it has been X amount of time since you last encountered this enemy? When
//       the game is an RPG with named characters, this could be weeks, months, years...
//       The enemy _should_ remember that you, the player, were responsible for damage
//       done... but the XP potential, perhaps that should be forgotten.
//
//       Propose we also have a concept of "Current Conflict". While in this mode, it
//       all counts! When the conflict is over - due to fleeing or whatever - the XP
//       potential can be forgotten. But the memory that you both fought should remain
//       (particularly when the game is a more complete RPG).
//
//       It's sort of like a short-term memory and long-term memory. Write the short-term
//       current conflict values first, then record these in the long-term memory for
//       retrieval/use at a later time.

/// The setup function
pub(crate) fn spawn_enemies(mut commands: Commands, sprites: Res<SpriteAssets>) {
    // Spawns enemy ships
    for (_i, pos) in [
        (250.0 as f32, 250.0 as f32),
        (-250.0 as f32, -250.0 as f32),
        (-25000.0 as f32, 0.0 as f32),
        (25000.0 as f32, 0.0 as f32),
        (0.0 as f32, 25000.0 as f32),
        (0.0 as f32, -25000.0 as f32),
    ]
    .iter()
    .enumerate()
    {
        commands.spawn((
            Enemy,
            Ship {
                thrust: 1_800_000.0,              // Ship thrust (TODO: What unit is this?)
                rotation: f32::to_radians(720.0), // Ship manoeuvrability (rad)
                bullet_timer: Timer::from_seconds(0.1, TimerMode::Once),
            },
            Gravitable,
            Health(1000.0),
            Indicated { color: Color::RED },
            SpriteBundle {
                texture: sprites.enemy_ship.clone(),
                transform: Transform {
                    translation: Vec3::new(pos.0, pos.1, 100.0),
                    scale: Vec3::splat(0.5),
                    ..default()
                },
                ..default()
            },
            RigidBody::Dynamic,
            Collider::ball(50.0),                  // 50.0 meters
            ColliderMassProperties::Mass(3926.99), // 3926.99 kilograms
            Velocity::linear(Vec2::ZERO),
            ExternalImpulse::default(),
            Damping {
                linear_damping: 0.3,
                angular_damping: 2.0,
            },
            Name::new("Enemy"),
            Adversaries(Vec::new()),
        ));
    }
}

pub fn enemy_targeting_system(
    mut commands: Commands,
    mut query: Query<(&Transform, Entity), With<Enemy>>,
    player: Query<&Transform, With<Player>>,
) {
    for (transform, entity) in query.iter_mut() {
        if let Ok(target) = player.get_single() {
            let desired_velocity = (target.translation - transform.translation)
                .truncate()
                .normalize_or_zero();
            let steering = desired_velocity.angle_between(transform.local_y().truncate());

            commands.entity(entity).insert(Targeting {
                pos: target.translation,
                angle: steering,
            });
        }
    }
}

pub fn enemy_flight_system(
    time: Res<Time>,
    mut query: Query<
        (
            &Ship,
            &Transform,
            &mut Velocity,
            &mut ExternalImpulse,
            &Targeting,
        ),
        With<Enemy>,
    >,
) {
    for (ship, transform, mut velocity, mut impulse, targeting) in query.iter_mut() {
        let desired_velocity = (targeting.pos - transform.translation)
            .truncate()
            .normalize_or_zero();
        let steering = targeting.angle;

        // Controls
        let mut rotation_factor = 0.0;
        let mut thrust_factor = 0.0;

        if desired_velocity != Vec2::ZERO && steering.abs() < 0.5 {
            thrust_factor += 1.0;
        }
        if steering > -0.1 {
            rotation_factor -= 1.0;
        }
        if steering < 0.1 {
            rotation_factor += 1.0;
        }

        ship_rotation(&time, rotation_factor, &mut velocity, ship);

        ship_thrust(&time, &mut impulse, transform, thrust_factor, ship);
    }
}

pub fn enemy_weapons_system(
    mut bullet_spawn_events: EventWriter<BulletSpawnEvent>,
    mut query: Query<(&mut Ship, &Transform, &mut Velocity, &Targeting, Entity), With<Enemy>>,
) {
    for (mut ship, transform, velocity, targeting, entity) in query.iter_mut() {
        let steering = targeting.angle;

        if steering.abs() < 0.1
            && transform.translation.distance(targeting.pos) < 1_000.0 // TODO: This can probably scale with difficulty
            && ship.bullet_timer.finished()
        {
            bullet_spawn_events.send(BulletSpawnEvent {
                spawner: entity,
                transform: *transform,
                velocity: *velocity,
            });
            ship.bullet_timer.reset();
        }
    }
}
