use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[cfg(debug_assertions)]
use {bevy_inspector_egui::quick::WorldInspectorPlugin, bevy_rapier2d::prelude::*};

mod core;
mod inputs;
mod shaders;
mod ships;
mod systems;
mod temp;
mod ui;
mod world;

use crate::{
    core::resources::{
        assets::{AudioAssets, DataAssets, I18nAssets, SpriteAssets, UiAssets},
        config::ConfigPlugin,
        i18n::I18nPlugin,
    },
    core::CorePlugin,
    ships::ShipsPlugin,
    systems::{events::EventsPlugin, states::GameState, SystemsPlugin},
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
        ConfigPlugin,
        CorePlugin,
        ShipsPlugin,
        WorldPlugin,
        UiPlugin,
        SystemsPlugin,
        EventsPlugin,
        I18nPlugin,
    ));

    #[cfg(debug_assertions)]
    app.add_plugins((
        RapierDebugRenderPlugin::default(),
        WorldInspectorPlugin::new(),
    ));

    // TODO: Assets should be separated according to where and when they are needed.
    //       Right now, we're loading ALL assets at startup. This is unnecessary.
    app.add_loading_state(
        LoadingState::new(GameState::Loading).continue_to_state(GameState::StartMenu),
    )
    .add_collection_to_loading_state::<_, SpriteAssets>(GameState::Loading)
    .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
    .add_collection_to_loading_state::<_, UiAssets>(GameState::Loading)
    .add_collection_to_loading_state::<_, DataAssets>(GameState::Loading)
    .add_collection_to_loading_state::<_, I18nAssets>(GameState::Loading);

    app.insert_resource(ClearColor(Color::rgb(0., 0., 0.)));

    app.run();
}
