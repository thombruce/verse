use bevy::prelude::*;

#[derive(Debug, Resource)]
pub struct SpriteAssets {
    pub player_ship: Handle<Image>,
    pub background: Handle<Image>,
}
#[derive(Debug, Resource)]
pub struct AudioAssets {
    pub title_music: Handle<AudioSource>,
    pub ambience: Handle<AudioSource>,
}

#[derive(Debug, Resource)]
pub struct UiAssets {
    pub font: Handle<Font>,
    pub title: UiImage,
}

pub struct AssetsPlugin;
impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
    }
}

// TODO: This is probably loading all of the assets at startup.
//       Prefer to insert_resource as and when needed, and unload
//       them when they aren't being used.
//       See: asset_server.free_unused_assets()
//       We can then remove the use of PreStartup above, which
//       doesn't feel idiomatic.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SpriteAssets {
        player_ship: asset_server.load("space/ships/playerShip2_blue.png"),
        background: asset_server.load("space/backgrounds/custom.png"),
    });
    commands.insert_resource(AudioAssets {
        title_music: asset_server.load("sound/Beat Mekanik - Lightspeed.ogg"),
        ambience: asset_server.load("sound/Kirk Osamayo - Space Dust.ogg"),
    });
    commands.insert_resource(UiAssets {
        font: asset_server.load("fonts/kenvector_future.ttf"),
        title: asset_server.load("verse.png").into(),
    });
}
