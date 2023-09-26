use bevy::prelude::*;

// TODO: Better time handling
const TIME: f32 = 120.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_systems(Startup, setup)
        .add_systems(Update, player_flight_system)
        .run();
}

/// The setup function
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawns game camera
    commands.spawn(Camera2dBundle::default());

    // Spawns player ship
    commands.spawn((
        Player {
            speed: 750.0,                     // Player speed
            rotation: f32::to_radians(360.0), // Player manoeuvrability
        },
        SpriteBundle {
            texture: asset_server.load("space/ships/playerShip2_blue.png"),
            ..default()
        },
    ));
}

/// player component
#[derive(Component)]
struct Player {
    /// linear speed in meters per second
    speed: f32,
    /// rotation speed in radians per second
    rotation: f32,
}

fn player_flight_system(keys: Res<Input<KeyCode>>, mut query: Query<(&Player, &mut Transform)>) {
    let (ship, mut transform) = query.single_mut();

    let mut rotation_factor = 0.0;
    let mut movement_factor = 0.0;

    if keys.any_pressed([KeyCode::Up, KeyCode::W]) {
        movement_factor += 1.0;
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        rotation_factor -= 1.0;
    }
    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        rotation_factor += 1.0;
    }

    // update the ship rotation around the Z axis (perpendicular to the 2D plane of the screen)
    transform.rotate_z(rotation_factor * ship.rotation / TIME);

    // get the ship's forward vector by applying the current rotation to the ships initial facing vector
    let movement_direction = transform.rotation * Vec3::Y;
    // get the distance the ship will move based on direction, the ship's movement speed and delta time
    let movement_distance = movement_factor * ship.speed / TIME;
    // create the change in translation using the new movement direction and distance
    let translation_delta = movement_direction * movement_distance;

    // update the ship translation with our new translation delta
    transform.translation += translation_delta;
}
