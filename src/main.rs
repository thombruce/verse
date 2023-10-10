#[allow(unused_imports)]
use bevy::{
    prelude::*,
    window::{Cursor, WindowMode},
};
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

mod core;
mod player;
mod shaders;
mod ui;
mod world;

use crate::{
    core::resources::{
        assets::{AudioAssets, SpriteAssets, UiAssets},
        state::GameState,
    },
    core::CorePlugin,
    player::PlayerPlugin,
    ui::UiPlugin,
    world::WorldPlugin,
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

    app.add_plugins((CorePlugin, PlayerPlugin, WorldPlugin, UiPlugin));

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
fn setup() {
    // Good place to put window setup configs, like whether or not
    // the player has suggested the game be played fullscreen.
}
