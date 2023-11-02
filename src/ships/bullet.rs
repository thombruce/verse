use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{
    core::resources::{
        assets::{AudioAssets, SpriteAssets},
        despawn_timer::DespawnTimer,
    },
    systems::events::BulletSpawnEvent,
};

use crate::systems::states::{ForState, GameState};

#[allow(unused_imports)]
use super::dynamic_orbit::Gravitable;

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct SpawnedBy(pub Entity);

pub(crate) fn spawn_bullet(
    mut commands: Commands,
    mut bullet_spawn_events: EventReader<BulletSpawnEvent>,
    handles: Res<SpriteAssets>,
    audios: Res<AudioAssets>,
) {
    for spawn_event in bullet_spawn_events.iter() {
        // Change this random factor to alter accuracy (larger is less accurate).
        const SPREAD: f32 = 0.05;
        let random_factor: f32 = rand::thread_rng().gen_range(-SPREAD..SPREAD);

        let transform = spawn_event.transform;
        let rand_rotation = transform
            .rotation
            .mul_quat(Quat::from_rotation_z(random_factor));

        let velocity = Velocity::linear(
            (spawn_event.velocity.linvel * Vec2::Y) + (rand_rotation * Vec3::Y * 2000.0).truncate(),
        );

        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(transform.translation.x, transform.translation.y, 99.0)
                        + transform.rotation * (Vec3::Y * 1.0),
                    rotation: rand_rotation,
                    ..default()
                },
                texture: handles.bullet.clone(),
                ..default()
            },
            Bullet,
            SpawnedBy(spawn_event.spawner),
            // TODO: Should bullets be affected by gravity?
            //       If so, set their initial velocity higher and consider
            //       despawning them either based on distance or a longer
            //       timer. Systems as they are, it's far too ridiculous and
            //       highly unpredictable.
            //       A bullet speed of around 5000.0 is a reasonable demo of the effect.
            //       But think again about despawning based on distance - this would mean
            //       the player could spawn millions of bullets inside of a gravitational
            //       well and never have them despawn. There has to be a limit to either
            //       the entity lifetime or the entity count.
            //       Also consider an alternative to adjusting bullet's speed:
            //       - introduce a gravitational scaling factor that may be unique per entity
            //       This might be unrealistic, but it might make for better gameplay.
            // Gravitable,
            // ExternalImpulse::default(),
            DespawnTimer(Timer::from_seconds(2.0, TimerMode::Once)),
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
                settings: PlaybackSettings {
                    mode: PlaybackMode::Remove,
                    // TODO: This should be relative to an SFX Volume setting
                    //       which should in turn be relative to the master volume.
                    //       Right now, this is just being set relative to the
                    //       GlobalVolume (which is configurable as master_volume).
                    volume: Volume::new_relative(1.0),
                    ..default()
                },
            },
        ));
    }
}
