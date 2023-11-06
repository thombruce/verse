#[allow(unused_imports)]
use bevy::{prelude::*, window::Cursor};
use bevy_asset_loader::prelude::*;

#[cfg(debug_assertions)]
use {bevy_inspector_egui::quick::WorldInspectorPlugin, bevy_rapier2d::prelude::*};

mod core;
mod i18n;
mod inputs;
mod shaders;
mod ships;
mod systems;
mod temp;
mod ui;
mod world;

use crate::{
    core::resources::{
        assets::{AudioAssets, DataAssets, SpriteAssets, UiAssets},
        config::ConfigPlugin,
    },
    core::CorePlugin,
    i18n::I18nPlugin,
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
        LoadingState::new(GameState::Loading).continue_to_state(GameState::LoadingTranslations),
    )
    .add_collection_to_loading_state::<_, SpriteAssets>(GameState::Loading)
    .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
    .add_collection_to_loading_state::<_, UiAssets>(GameState::Loading)
    .add_collection_to_loading_state::<_, DataAssets>(GameState::Loading);

    app.insert_resource(ClearColor(Color::rgb(0., 0., 0.)));

    app.run();
}
