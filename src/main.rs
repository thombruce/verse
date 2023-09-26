use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_player))
        .run();
}

/// The setup function
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

/// Spawns player sprite
fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        texture: asset_server.load("space/ships/playerShip2_blue.png"),
        ..default()
    });
}
