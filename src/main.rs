use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod ship;
use ship::ship_flight_system;
use ship::Ship;

pub const TIME: f32 = 1.0 / 60.0;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0));

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.insert_resource(FixedTime::new_from_secs(TIME));
    app.insert_resource(ClearColor(Color::rgb(0., 0., 0.)));

    app.add_systems(Startup, setup);
    app.add_systems(FixedUpdate, ship_flight_system);

    app.run();
}

/// The setup function
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
) {
    rapier_configuration.gravity = Vec2::ZERO;

    // Spawns game camera
    commands.spawn(Camera2dBundle::default());

    // Spawns player ship
    commands.spawn((
        Ship {
            thrust: 10000.0,                  // Ship thrust (TODO: What unit is this?)
            rotation: f32::to_radians(360.0), // Ship manoeuvrability (rad)
        },
        SpriteBundle {
            texture: asset_server.load("space/ships/playerShip2_blue.png"),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(50.0),                 // 50.0 meters
        ColliderMassProperties::Density(0.5), // (Pi * 50.0^2) * 0.5 = 3926.99 kilograms
        Velocity::linear(Vec2::ZERO),
        ExternalImpulse::default(),
    ));
}
