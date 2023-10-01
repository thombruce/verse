use bevy::{audio::PlaybackMode, prelude::*, render::view::NoFrustumCulling};
use bevy_rapier2d::prelude::*;
use bevy_tiling_background::{
    BackgroundImageBundle, BackgroundMaterial, SetImageRepeatingExt, TilingBackgroundPlugin,
};
use leafwing_input_manager::prelude::*;

mod camera;
mod credits;
mod effects;
mod hud;
mod menu;
mod pause;
mod ship;
mod state;

use crate::{
    camera::CameraPlugin,
    credits::CreditsPlugin,
    effects::EffectsPlugin,
    hud::HudPlugin,
    menu::{MenuAction, MenuPlugin},
    pause::{PauseAction, PausePlugin},
    ship::{ShipAction, ShipPlugin},
    state::{AppState, ForState, StatePlugin},
};

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
        InputManagerPlugin::<MenuAction>::default(),
        InputManagerPlugin::<ShipAction>::default(),
        TilingBackgroundPlugin::<BackgroundMaterial>::default(),
        CameraPlugin,
        StatePlugin,
        HudPlugin,
        ShipPlugin,
        MenuPlugin,
        CreditsPlugin,
        EffectsPlugin,
        PausePlugin,
    ));

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    // app.insert_resource(ClearColor(Color::rgb(0., 0., 0.)));

    app.add_systems(Startup, setup);

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

    commands.spawn((
        BackgroundImageBundle::from_image(image, materials.as_mut()).at_z_layer(-0.1),
        NoFrustumCulling,
    ));

    // TODO: Moved here from menu to prevent reloading every time the credits are toggled.
    //       In reality, we do want this to be respawned when the menu is re-entered,
    //       just not if the previous state was also a menu state (e.g. Credits).
    commands.spawn((
        AudioBundle {
            source: asset_server.load("sound/Beat Mekanik - Lightspeed.ogg"),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
        },
        ForState {
            states: AppState::IN_MENU_STATE.to_vec(),
        },
    ));
}
