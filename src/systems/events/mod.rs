use bevy::ecs::query::Has;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::ships::{
    bullet::{Bullet, SpawnedBy},
    ship::Ship,
};

pub struct EventsPlugin;
impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletSpawnEvent>()
            .add_event::<BulletShipContactEvent>();
    }
}

#[derive(Event)]
pub struct BulletSpawnEvent {
    // The entity requesting a bullet spawn
    pub spawner: Entity,
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

pub(crate) fn contact_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut bullet_ship_contact_events: EventWriter<BulletShipContactEvent>,
    query: Query<(Has<Ship>, Has<Bullet>)>,
    bullet_spawner: Query<&SpawnedBy, With<Bullet>>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _flags) = event {
            let (e1_is_ship, e1_is_bullet) = query.get(*e1).unwrap();
            let (e2_is_ship, e2_is_bullet) = query.get(*e2).unwrap();
            if e1_is_ship && e2_is_bullet {
                let spawner = bullet_spawner.get(*e2).unwrap().0;

                if spawner == *e1 {
                    continue;
                }

                bullet_ship_contact_events.send(BulletShipContactEvent {
                    ship: *e1,
                    bullet: *e2,
                });
            }
            if e2_is_ship && e1_is_bullet {
                let spawner = bullet_spawner.get(*e1).unwrap().0;

                if spawner == *e2 {
                    continue;
                }

                bullet_ship_contact_events.send(BulletShipContactEvent {
                    ship: *e2,
                    bullet: *e1,
                });
            }
        }
    }
}
