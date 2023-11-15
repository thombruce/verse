use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    core::resources::assets::SpriteAssets,
    inputs::ship::{ship_input_map, ShipAction},
    systems::events::BulletSpawnEvent,
};

use super::{
    dynamic_orbit::Gravitable,
    ship::{ship_rotation, ship_thrust, Health, Ship},
};

/// Player component
#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<ShipAction>::default());
    }
}

/// The setup function
pub(crate) fn spawn_player(mut commands: Commands, sprites: Res<SpriteAssets>) {
    // Spawns player ship
    commands.spawn((
        Player,
        Ship {
            thrust: 1_800_000.0,              // Ship thrust (TODO: What unit is this?)
            rotation: f32::to_radians(720.0), // Ship manoeuvrability (rad)
            bullet_timer: Timer::from_seconds(0.1, TimerMode::Once),
        },
        Gravitable,
        Health(10000.0), // TODO: Game balancing
        SpriteBundle {
            texture: sprites.player_ship.clone(),
            transform: Transform {
                translation: Vec3::new(0., -6000., 100.0),
                scale: Vec3::splat(0.5),
                ..default()
            },
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(50.0), // 50.0 meters
        // Setting the mass to 3926.99 is the same as setting density to 0.5
        // Pi * r^2 / density = (Pi * 50.0^2) * 0.5 = 3926.99 kilograms
        ColliderMassProperties::Mass(3926.99), // 3926.99 kilograms
        // In the future, we might attempt to configure the center of mass as well
        // but this will require access to its position for the camera to follow:
        // ColliderMassProperties::MassProperties(MassProperties {
        //     local_center_of_mass: Vec2::new(0.0, -25.0),
        //     mass: 3926.99,
        //     ..default()
        // }),
        Velocity::linear(Vec2::ZERO),
        ExternalImpulse::default(),
        Damping {
            linear_damping: 0.3,
            angular_damping: 2.0,
        },
        InputManagerBundle::<ShipAction> {
            action_state: ActionState::default(),
            input_map: ship_input_map(),
        },
        Name::new("Player"),
    ));
}

pub fn player_flight_system(
    time: Res<Time>,
    mut query: Query<
        (
            &Ship,
            &Transform,
            &mut Velocity,
            &mut ExternalImpulse,
            &ActionState<ShipAction>,
        ),
        With<Player>,
    >,
) {
    if let Ok((ship, transform, mut velocity, mut impulse, action_state)) = query.get_single_mut() {
        // Controls
        let mut rotation_factor = 0.0;
        let mut thrust_factor = 0.0;

        if action_state.pressed(ShipAction::Forward) {
            thrust_factor += 1.0;
        }
        if action_state.pressed(ShipAction::RotateRight) {
            rotation_factor -= 1.0;
        }
        if action_state.pressed(ShipAction::RotateLeft) {
            rotation_factor += 1.0;
        }

        ship_rotation(&time, rotation_factor, &mut velocity, ship);

        ship_thrust(&time, &mut impulse, transform, thrust_factor, ship);
    }
}

pub fn player_weapons_system(
    mut bullet_spawn_events: EventWriter<BulletSpawnEvent>,
    mut query: Query<
        (
            &mut Ship,
            &Transform,
            &mut Velocity,
            &ActionState<ShipAction>,
            Entity,
        ),
        With<Player>,
    >,
) {
    if let Ok((mut ship, transform, velocity, action_state, entity)) = query.get_single_mut() {
        if action_state.pressed(ShipAction::Fire) && ship.bullet_timer.finished() {
            bullet_spawn_events.send(BulletSpawnEvent {
                spawner: entity,
                transform: *transform,
                velocity: *velocity,
            });
            ship.bullet_timer.reset();
        }
    }
}
