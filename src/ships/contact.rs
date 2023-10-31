use bevy::ecs::query::Has;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::systems::events::BulletShipContactEvent;

use super::bullet::Bullet;
use super::ship::Ship;

pub(crate) fn contact_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut bullet_ship_contact_events: EventWriter<BulletShipContactEvent>,
    query: Query<(Has<Ship>, Has<Bullet>)>,
) {
    for event in collision_events.iter() {
        if let CollisionEvent::Started(e1, e2, _flags) = event {
            let (e1_is_ship, e1_is_bullet) = query.get(*e1).unwrap();
            let (e2_is_ship, e2_is_bullet) = query.get(*e2).unwrap();
            if e1_is_ship && e2_is_bullet {
                bullet_ship_contact_events.send(BulletShipContactEvent {
                    ship: *e1,
                    bullet: *e2,
                });
            }
            if e2_is_ship && e1_is_bullet {
                bullet_ship_contact_events.send(BulletShipContactEvent {
                    ship: *e2,
                    bullet: *e1,
                });
            }
        }
    }
}
