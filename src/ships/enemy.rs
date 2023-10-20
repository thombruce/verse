use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::core::resources::{assets::SpriteAssets, state::GameState};

use super::{
    bullet::BulletSpawnEvent,
    player::Player,
    ship::{dampening, ship_rotation, ship_thrust, AttackSet, Health, MovementSet, Ship},
};

/// Enemy component
#[derive(Component)]
pub struct Enemy;

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameCreate), setup);
        app.add_systems(
            Update,
            (
                enemy_flight_system.in_set(MovementSet),
                enemy_weapons_system.in_set(AttackSet),
            )
                .run_if(in_state(GameState::Active)),
        );
    }
}

/// The setup function
fn setup(mut commands: Commands, sprites: Res<SpriteAssets>) {
    // Spawns enemy ships
    for (_i, pos) in [250.0 as f32, -250.0 as f32].iter().enumerate() {
        commands.spawn((
            Enemy,
            Ship {
                thrust: 10000.0,                  // Ship thrust (TODO: What unit is this?)
                rotation: f32::to_radians(360.0), // Ship manoeuvrability (rad)
                bullet_timer: Timer::from_seconds(0.1, TimerMode::Once),
            },
            Health(1000.0),
            SpriteBundle {
                texture: sprites.enemy_ship.clone(),
                transform: Transform {
                    translation: Vec3::new(*pos, *pos, 100.0),
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
            Name::new("Enemy"),
        ));
    }
}

pub fn enemy_flight_system(
    time: Res<Time>,
    mut query: Query<(&Ship, &Transform, &mut Velocity, &mut ExternalImpulse), With<Enemy>>,
    player: Query<&Transform, With<Player>>,
) {
    for (ship, transform, mut velocity, mut impulse) in query.iter_mut() {
        dampening(&time, &mut velocity);

        let target = player.single();
        let desired_velocity = (target.translation - transform.translation)
            .xy()
            .normalize_or_zero();
        let steering = desired_velocity.angle_between(transform.local_y().truncate());

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

        ship_rotation(rotation_factor, &mut velocity, ship);

        ship_thrust(&mut impulse, transform, thrust_factor, ship);
    }
}

pub fn enemy_weapons_system(
    mut bullet_spawn_events: EventWriter<BulletSpawnEvent>,
    mut query: Query<(&mut Ship, &Transform, &mut Velocity), With<Enemy>>,
    player: Query<&Transform, With<Player>>,
) {
    for (mut ship, transform, velocity) in query.iter_mut() {
        // TODO: Don't Repeat Yourself!
        //       Perhaps steering intention can be stored within
        //       a component on the Enemy ship.
        //       Maybe a Targeting component with position and angle values.
        let target = player.single();
        let desired_velocity = (target.translation - transform.translation)
            .xy()
            .normalize_or_zero();
        let steering = desired_velocity.angle_between(transform.local_y().truncate());

        if steering.abs() < 0.1
            && transform.translation.distance(target.translation) < 400.0
            && ship.bullet_timer.finished()
        {
            bullet_spawn_events.send(BulletSpawnEvent {
                transform: *transform,
                velocity: *velocity,
            });
            ship.bullet_timer.reset();
        }
    }
}

// TODO: There is an issue with spawning bullets at extremely close proximity still.
// thread 'Compute Task Pool (1)' panicked at 'Attempting to create an EntityCommands for entity 32v3, which doesn't exist.', src\ships\ship.rs:85:18
