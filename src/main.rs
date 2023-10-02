use bevy::{audio::PlaybackMode, prelude::*, render::view::NoFrustumCulling};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use bevy_tiling_background::{
    BackgroundImageBundle, BackgroundMaterial, SetImageRepeatingExt, TilingBackgroundPlugin,
};
use leafwing_input_manager::prelude::*;

mod assets;
mod camera;
mod credits;
mod effects;
mod hud;
mod menu;
mod pause;
mod ship;
mod state;

use crate::{
    assets::{AssetsPlugin, AudioAssets, SpriteAssets},
    camera::CameraPlugin,
    credits::CreditsPlugin,
    effects::EffectsPlugin,
    hud::HudPlugin,
    menu::{MenuAction, MenuPlugin},
    pause::PausePlugin,
    ship::ShipPlugin,
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
        AssetsPlugin,
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0),
        InputManagerPlugin::<MenuAction>::default(),
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
    app.add_plugins((
        RapierDebugRenderPlugin::default(),
        WorldInspectorPlugin::new(),
    ));

    // app.insert_resource(ClearColor(Color::rgb(0., 0., 0.)));

    app.add_systems(Startup, setup);

    app.run();
}

/// The setup function
fn setup(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
    audios: Res<AudioAssets>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    rapier_configuration.gravity = Vec2::ZERO;

    let image = sprites.background.clone();
    // Queue a command to set the image to be repeating once the image is loaded.
    commands.set_image_repeating(image.clone());

    commands.spawn((
        BackgroundImageBundle::from_image(image, materials.as_mut()).at_z_layer(-0.1),
        NoFrustumCulling,
        Name::new("Background"),
    ));

    // TODO: Moved here from menu to prevent reloading every time the credits are toggled.
    //       In reality, we do want this to be respawned when the menu is re-entered,
    //       just not if the previous state was also a menu state (e.g. Credits).
    commands.spawn((
        AudioBundle {
            source: audios.title_music.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
        },
        ForState {
            states: AppState::IN_MENU_STATE.to_vec(),
        },
        Name::new("Menu Music"),
    ));
}
