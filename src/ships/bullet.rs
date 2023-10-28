use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::core::resources::{
    assets::{AudioAssets, SpriteAssets},
    despawn_timer::DespawnTimer,
    state::{ForState, GameState},
};

#[allow(unused_imports)]
use super::{dynamic_orbit::Gravitable, ship::MovementSet};

#[derive(Component)]
pub struct Bullet;

#[derive(Event)]
pub struct BulletSpawnEvent {
    // The full position (translation+rotation) of the bullet to spawn
    pub transform: Transform,
    // The velocity of the entity emitting the bullet
    pub velocity: Velocity,
}

#[derive(Event)]
pub struct BulletShipContactEvent {
    pub bullet: Entity,
    pub ship: Entity,
}

pub struct BulletPlugin;
impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletSpawnEvent>()
            .add_event::<BulletShipContactEvent>()
            .add_systems(
                Update,
                (spawn_bullet.after(MovementSet)).run_if(in_state(GameState::Active)),
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
                    translation: Vec3::new(transform.translation.x, transform.translation.y, 99.0)
                        + transform.rotation * (Vec3::Y * 35.0), // Ships radius * scaling factor + 5px padding
                    rotation: transform.rotation,
                    ..default()
                },
                texture: handles.bullet.clone(),
                ..default()
            },
            Bullet,
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
                ..default()
            },
        ));
    }
}
