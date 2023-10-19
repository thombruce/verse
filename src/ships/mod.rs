use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod bullet;
pub mod contact;
pub mod dynamic_orbit;
pub mod enemy;
pub mod player;
pub mod ship;

use self::{
    bullet::BulletPlugin, contact::ContactPlugin, dynamic_orbit::DynamicOrbitPlugin,
    enemy::EnemyPlugin, player::PlayerPlugin, ship::ShipPlugin,
};

pub struct ShipsPlugin;
impl Plugin for ShipsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0),
            ContactPlugin,
            ShipPlugin,
            PlayerPlugin,
            EnemyPlugin,
            DynamicOrbitPlugin,
            BulletPlugin,
        ));

        app.add_systems(Startup, setup);
    }
}

/// The setup function
fn setup(mut rapier_configuration: ResMut<RapierConfiguration>) {
    rapier_configuration.gravity = Vec2::ZERO;
}
