use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::core::resources::{assets::SpriteAssets, state::GameState};

use super::{
    bullet::BulletSpawnEvent,
    ship::{dampening, Ship},
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
            (enemy_flight_system, enemy_weapons_system).run_if(in_state(GameState::Active)),
        );
    }
}

/// The setup function
fn setup(mut commands: Commands, sprites: Res<SpriteAssets>) {
    // Spawns enemy ship
    commands.spawn((
        Enemy,
        Ship {
            thrust: 10000.0,                  // Ship thrust (TODO: What unit is this?)
            rotation: f32::to_radians(360.0), // Ship manoeuvrability (rad)
            bullet_timer: Timer::from_seconds(0.1, TimerMode::Once),
        },
        SpriteBundle {
            texture: sprites.enemy_ship.clone(),
            transform: Transform {
                translation: Vec3::new(1000., 1000., 100.0),
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

pub fn enemy_flight_system(
    time: Res<Time>,
    mut query: Query<(&Ship, &Transform, &mut Velocity, &mut ExternalImpulse), With<Enemy>>,
) {
    let (_ship, _transform, mut velocity, mut _impulse) = query.single_mut();

    dampening(time, &mut velocity);

    // TODO: Enemy needs a brain!
}

pub fn enemy_weapons_system(
    mut bullet_spawn_events: EventWriter<BulletSpawnEvent>,
    mut query: Query<(&mut Ship, &Transform, &mut Velocity), With<Enemy>>,
) {
    let (mut ship, transform, velocity) = query.single_mut();

    // TODO: Enemy needs a brain!
    if false && ship.bullet_timer.finished() {
        bullet_spawn_events.send(BulletSpawnEvent {
            transform: *transform,
            velocity: *velocity,
        });
        ship.bullet_timer.reset();
    }
}
