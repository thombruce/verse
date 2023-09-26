use bevy::prelude::*;

mod ship;
use ship::ship_flight_system;
use ship::Ship;

pub const TIME: f32 = 1.0 / 60.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(FixedTime::new_from_secs(TIME))
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, ship_flight_system)
        .run();
}

/// The setup function
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawns game camera
    commands.spawn(Camera2dBundle::default());

    // Spawns player ship
    commands.spawn((
        Ship {
            speed: 750.0,                     // Ship speed
            rotation: f32::to_radians(360.0), // Ship manoeuvrability
        },
        SpriteBundle {
            texture: asset_server.load("space/ships/playerShip2_blue.png"),
            ..default()
        },
    ));
}
