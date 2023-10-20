use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::core::resources::{assets::SpriteAssets, state::GameState};

use super::{
    bullet::BulletSpawnEvent,
    ship::{dampening, AttackSet, Health, MovementSet, Ship},
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
    for (_i, pos) in [1000.0 as f32, -1000.0 as f32].iter().enumerate() {
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
) {
    for (_ship, _transform, mut velocity, mut _impulse) in query.iter_mut() {
        dampening(&time, &mut velocity);

        // TODO: Enemy needs a brain!
    }
}

pub fn enemy_weapons_system(
    mut bullet_spawn_events: EventWriter<BulletSpawnEvent>,
    mut query: Query<(&mut Ship, &Transform, &mut Velocity), With<Enemy>>,
) {
    for (mut ship, transform, velocity) in query.iter_mut() {
        // TODO: Enemy needs a brain!
        if false && ship.bullet_timer.finished() {
            bullet_spawn_events.send(BulletSpawnEvent {
                transform: *transform,
                velocity: *velocity,
            });
            ship.bullet_timer.reset();
        }
    }
}
