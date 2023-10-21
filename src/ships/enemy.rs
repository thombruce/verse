use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::core::resources::{assets::SpriteAssets, state::GameState};
use crate::ui::hud::indicator::Indicated;

use super::{
    bullet::BulletSpawnEvent,
    player::Player,
    ship::{dampening, ship_rotation, ship_thrust, AttackSet, Health, MovementSet, Ship},
};

/// Enemy component
#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Targeting {
    pub pos: Vec3,
    pub angle: f32,
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameCreate), setup);
        app.add_systems(
            Update,
            (
                enemy_targeting_system.before(MovementSet),
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
                thrust: 10000.0,                  // Ship thrust (TODO: What unit is this?)
                rotation: f32::to_radians(360.0), // Ship manoeuvrability (rad)
                bullet_timer: Timer::from_seconds(0.1, TimerMode::Once),
            },
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
            Name::new("Enemy"),
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
        dampening(&time, &mut velocity);

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

        ship_rotation(rotation_factor, &mut velocity, ship);

        ship_thrust(&mut impulse, transform, thrust_factor, ship);
    }
}

pub fn enemy_weapons_system(
    mut bullet_spawn_events: EventWriter<BulletSpawnEvent>,
    mut query: Query<(&mut Ship, &Transform, &mut Velocity, &Targeting), With<Enemy>>,
) {
    for (mut ship, transform, velocity, targeting) in query.iter_mut() {
        let steering = targeting.angle;

        if steering.abs() < 0.1
            && transform.translation.distance(targeting.pos) < 400.0
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
