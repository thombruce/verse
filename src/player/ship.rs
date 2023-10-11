use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::core::resources::{assets::SpriteAssets, state::GameState};

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum ShipAction {
    Forward,
    RotateLeft,
    RotateRight,
}

/// Ship component
#[derive(Component)]
pub struct Ship {
    /// linear speed in meters per second
    pub thrust: f32,
    /// rotation speed in radians per second
    pub rotation: f32,
}

pub struct ShipPlugin;
impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<ShipAction>::default());

        app.add_systems(OnEnter(GameState::GameCreate), setup);
        app.add_systems(
            Update,
            ship_flight_system.run_if(in_state(GameState::Active)),
        );
    }
}

/// The setup function
fn setup(mut commands: Commands, sprites: Res<SpriteAssets>) {
    let mut input_map = InputMap::new([
        // Cursor keys
        (KeyCode::Up, ShipAction::Forward),
        (KeyCode::Left, ShipAction::RotateLeft),
        (KeyCode::Right, ShipAction::RotateRight),
        // WASD
        (KeyCode::W, ShipAction::Forward),
        (KeyCode::A, ShipAction::RotateLeft),
        (KeyCode::D, ShipAction::RotateRight),
    ]);
    // Gamepad
    input_map.insert(GamepadButtonType::RightTrigger2, ShipAction::Forward);
    input_map.insert(
        SingleAxis::positive_only(GamepadAxisType::LeftStickX, 0.4),
        ShipAction::RotateRight,
    );
    input_map.insert(
        SingleAxis::negative_only(GamepadAxisType::LeftStickX, -0.4),
        ShipAction::RotateLeft,
    );

    // Spawns player ship
    commands.spawn((
        Ship {
            thrust: 10000.0,                  // Ship thrust (TODO: What unit is this?)
            rotation: f32::to_radians(360.0), // Ship manoeuvrability (rad)
        },
        SpriteBundle {
            texture: sprites.player_ship.clone(),
            transform: Transform {
                translation: Vec3::new(200., 0., 100.0),
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
        InputManagerBundle::<ShipAction> {
            action_state: ActionState::default(),
            input_map: input_map.build(),
        },
        Name::new("Ship"),
    ));
}

pub fn ship_flight_system(
    time: Res<Time>,
    mut query: Query<(
        &Ship,
        &Transform,
        &mut Velocity,
        &mut ExternalImpulse,
        &ActionState<ShipAction>,
    )>,
) {
    let (ship, transform, mut velocity, mut impulse, action_state) = query.single_mut();

    // Dampening
    let elapsed = time.delta_seconds();
    velocity.angvel *= 0.1f32.powf(elapsed);
    velocity.linvel *= 0.8f32.powf(elapsed);

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

    // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    if rotation_factor != 0.0 {
        velocity.angvel += rotation_factor * ship.rotation / 60.0;
    }

    impulse.impulse += (transform.rotation * (Vec3::Y * thrust_factor * ship.thrust)).truncate();
}
