#[allow(unused_imports)]
use bevy::{
    prelude::*,
    window::{Cursor, PrimaryWindow, WindowMode},
    winit::WinitWindows,
};
use bevy_asset_loader::prelude::*;
use winit::window::Icon;

#[cfg(debug_assertions)]
use {bevy_inspector_egui::quick::WorldInspectorPlugin, bevy_rapier2d::prelude::*};

mod common_assets_demo;
mod core;
mod inputs;
mod shaders;
mod ships;
mod ui;
mod world;

use crate::{
    common_assets_demo::CommonAssetsDemoPlugin,
    core::resources::{
        assets::{AudioAssets, SpriteAssets, UiAssets},
        state::GameState,
    },
    core::CorePlugin,
    ships::ShipsPlugin,
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
                    #[cfg(not(debug_assertions))]
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

    app.add_plugins((
        CommonAssetsDemoPlugin,
        CorePlugin,
        ShipsPlugin,
        WorldPlugin,
        UiPlugin,
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

    app.add_systems(Startup, set_window_icon);

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

// Documented:
// - https://bevy-cheatbook.github.io/window/icon.html
// - https://stackoverflow.com/a/76729516/2225649
// Open issue: https://github.com/bevyengine/bevy/issues/1031
// TODO: Change to official approach when issue is closed and released
fn set_window_icon(
    main_window: Query<Entity, With<PrimaryWindow>>,
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    let primary = windows.get_window(main_window.single()).unwrap();

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/VerseSquircle-256.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary.set_window_icon(Some(icon));
}
