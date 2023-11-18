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
            .add_event::<BulletShipContactEvent>()
            .add_event::<DeathEvent>();
    }
}

#[derive(Event)]
pub struct DeathEvent {
    // The entity
    pub entity: Entity,
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
    pub bullet_spawner: Entity,
    // The thinking is that if the BulletShipContactEvent
    // can help identify the spawner as being the player,
    // we can use this to award points when the player
    // deals damage to an enemy...
    //
    // But what about enemy kills?
    //
    // The way I'd like to see kills handled in terms of
    // XP rewards is... the kill should be worth a certain
    // amount, but it should be proportionately awarded
    // based on the percentage of the entity's total health
    // that was taken away by the player. Which means...
    //
    // We need to record WHO did WHAT damage. This will
    // also be handy later for advanced AI stuff.
    //
    // This would be best done, how? As a component on the
    // Enemy which records Damage and Aggressor;
    // e.g. { aggressor: Player, damage: 700 }
    //
    // So some kind of Attackers component that is a Vec
    // of such objects?
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
                    bullet_spawner: spawner,
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
                    bullet_spawner: spawner,
                });
            }
        }
    }
}
