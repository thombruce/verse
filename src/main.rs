use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy_rapier2d::prelude::*;
use bevy_tiling_background::{
    BackgroundImageBundle, BackgroundMaterial, SetImageRepeatingExt, TilingBackgroundPlugin,
};

mod ship;
use ship::ship_flight_system;
use ship::Ship;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0));
    app.add_plugins(TilingBackgroundPlugin::<BackgroundMaterial>::default());

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    // app.insert_resource(ClearColor(Color::rgb(0., 0., 0.)));

    app.add_systems(Startup, setup);
    app.add_systems(Update, (ship_flight_system, follow_player));

    app.run();
}

/// The setup function
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    rapier_configuration.gravity = Vec2::ZERO;

    let image = asset_server.load("space/backgrounds/custom.png");
    // Queue a command to set the image to be repeating once the image is loaded.
    commands.set_image_repeating(image.clone());

    // Spawns game camera
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        BackgroundImageBundle::from_image(image, materials.as_mut()).at_z_layer(-0.1),
        NoFrustumCulling,
    ));

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

fn follow_player(
    mut camera: Query<&mut Transform, (With<Camera>, Without<Ship>)>,
    player: Query<&Transform, (With<Ship>, Without<Camera>)>,
) {
    let mut camera_transform = camera.single_mut();
    let player_transform = player.single();

    let pos = player_transform.translation;

    camera_transform.translation.x = pos.x;
    camera_transform.translation.y = pos.y;
}
