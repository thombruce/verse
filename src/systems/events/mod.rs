use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

pub struct EventsPlugin;
impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BulletSpawnEvent>()
            .add_event::<BulletShipContactEvent>();
    }
}

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
