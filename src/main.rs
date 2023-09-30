use bevy::{prelude::*, render::view::NoFrustumCulling};
use bevy_rapier2d::prelude::*;
use bevy_tiling_background::{
    BackgroundImageBundle, BackgroundMaterial, SetImageRepeatingExt, TilingBackgroundPlugin,
};
use leafwing_input_manager::prelude::*;

mod ship;
use ship::{Ship, ShipAction, ShipPlugin};

mod camera;
use camera::follow_player;

mod hud;
use hud::HudPlugin;

mod state;
use state::{AppState, PauseAction, StatePlugin};

fn main() {
    let mut app = App::new();

    app.add_state::<AppState>();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Verse"),
                ..default()
            }),
            ..default()
        }),
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0),
        InputManagerPlugin::<PauseAction>::default(),
        InputManagerPlugin::<ShipAction>::default(),
        TilingBackgroundPlugin::<BackgroundMaterial>::default(),
        HudPlugin,
        ShipPlugin,
        StatePlugin,
    ));

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    // app.insert_resource(ClearColor(Color::rgb(0., 0., 0.)));

    app.add_systems(Startup, setup);
    app.add_systems(Update, follow_player.run_if(in_state(AppState::Active)));

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
}
