use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;

mod astronomy;
mod effects;
mod hud;
mod menus;
mod resources;
mod shaders;
mod ship;

use crate::{
    astronomy::planetary_system::PlanetarySystemPlugin,
    astronomy::starfield::BackgroundPlugin,
    effects::blink::EffectsPlugin,
    hud::HudPlugin,
    menus::credits::CreditsPlugin,
    menus::pause::PausePlugin,
    menus::start_menu::{MenuAction, MenuPlugin},
    resources::assets::{AudioAssets, SpriteAssets, UiAssets},
    resources::camera::CameraPlugin,
    resources::game_time::GameTimePlugin,
    resources::state::{GameState, StatePlugin},
    ship::ShipPlugin,
};

fn main() {
    let mut app = App::new();

    app.add_state::<GameState>();

    app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("Verse"),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
        GameTimePlugin,
        RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0),
        InputManagerPlugin::<MenuAction>::default(),
        BackgroundPlugin,
        CameraPlugin,
        StatePlugin,
        HudPlugin,
        ShipPlugin,
        MenuPlugin,
        CreditsPlugin,
        EffectsPlugin,
        PausePlugin,
        PlanetarySystemPlugin,
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
