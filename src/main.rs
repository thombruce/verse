#[allow(unused_imports)]
use bevy::{
    prelude::*,
    window::{Cursor, WindowMode},
};
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod astronomy;
mod effects;
mod hud;
mod menus;
mod resources;
mod shaders;
mod ship;

use crate::{
    astronomy::{planetary_system::PlanetarySystemPlugin, starfield::StarfieldPlugin},
    effects::{animate::AnimatePlugin, blink::EffectsPlugin},
    hud::HudPlugin,
    menus::{credits::CreditsPlugin, pause::PausePlugin, start_menu::MenuPlugin},
    resources::{
        assets::{AudioAssets, SpriteAssets, UiAssets},
        camera::CameraPlugin,
        game_time::GameTimePlugin,
        spatial::SpatialPlugin,
        state::{GameState, StatePlugin},
    },
    ship::ShipPlugin,
};

fn main() {
    let mut app = App::new();

    app.add_state::<GameState>();

    // Defaults
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Verse"),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    cursor: Cursor {
                        visible: false,
                        ..default()
                    },
                    // mode: WindowMode::Fullscreen,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );

    // Core
    app.add_plugins((GameTimePlugin, StatePlugin, EffectsPlugin, AnimatePlugin));

    // World
    app.add_plugins((
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0),
        StarfieldPlugin,
        ShipPlugin,
        PlanetarySystemPlugin,
        SpatialPlugin,
    ));

    // UI
    app.add_plugins((
        HudPlugin,
        MenuPlugin,
        CreditsPlugin,
        PausePlugin,
        CameraPlugin,
    ));

    #[cfg(debug_assertions)]
    app.add_plugins((
        RapierDebugRenderPlugin::default(),
        WorldInspectorPlugin::new(),
    ));

    app.add_loading_state(
        LoadingState::new(GameState::Loading).continue_to_state(GameState::StartMenu),
    )
    .add_collection_to_loading_state::<_, SpriteAssets>(GameState::Loading)
    .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
    .add_collection_to_loading_state::<_, UiAssets>(GameState::Loading);

    app.insert_resource(ClearColor(Color::rgb(0., 0., 0.)));

    app.add_systems(
        OnTransition {
            from: GameState::Loading,
            to: GameState::StartMenu,
        },
        setup,
    );

    app.run();
}

/// The setup function
fn setup(mut rapier_configuration: ResMut<RapierConfiguration>) {
    rapier_configuration.gravity = Vec2::ZERO;
}
