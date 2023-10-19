use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::core::resources::{
    assets::{AudioAssets, SpriteAssets},
    state::{ForState, GameState},
};

#[derive(Component)]
pub struct Bullet {
    pub despawn_timer: Timer,
}

#[derive(Event)]
pub struct BulletSpawnEvent {
    // The full position (translation+rotation) of the bullet to spawn
    pub transform: Transform,
    // The velocity of the entity emitting the bullet
    pub velocity: Velocity,
}

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletSpawnEvent>().add_systems(
            Update,
            (spawn_bullet, bullet_despawn_system).run_if(in_state(GameState::Active)),
        );
    }
}

fn spawn_bullet(
    mut commands: Commands,
    mut bullet_spawn_events: EventReader<BulletSpawnEvent>,
    handles: Res<SpriteAssets>,
    audios: Res<AudioAssets>,
) {
    for spawn_event in bullet_spawn_events.iter() {
        let transform = spawn_event.transform;
        let velocity = Velocity::linear(
            (spawn_event.velocity.linvel * Vec2::Y)
                + (transform.rotation * Vec3::Y * 2000.0).truncate(),
        );
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(transform.translation.x, transform.translation.y, 99.0),
                    rotation: transform.rotation,
                    ..default()
                },
                texture: handles.bullet.clone(),
                ..default()
            },
            Bullet {
                despawn_timer: Timer::from_seconds(2.0, TimerMode::Once),
            },
            ForState {
                states: GameState::IN_GAME_STATE.to_vec(),
            },
            RigidBody::Dynamic,
            Collider::cuboid(2.5, 8.0),
            velocity,
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            AudioBundle {
                source: audios.gun.clone(),
                ..default()
            },
        ));
    }
}

fn bullet_despawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Bullet)>,
) {
    for (entity, mut bullet) in query.iter_mut() {
        bullet.despawn_timer.tick(time.delta());
        if bullet.despawn_timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
