use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::ui::menus::credits::Credits;

use super::config::GameConfig;

#[derive(AssetCollection, Resource)]
pub struct DataAssets {
    #[asset(path = "verse.config.ron")]
    pub config: Handle<GameConfig>,

    #[asset(path = "data/verse.credits.ron")]
    pub credits: Handle<Credits>,
}

#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {
    #[asset(path = "space/ships/player.png")]
    pub player_ship: Handle<Image>,

    #[asset(path = "space/ships/enemy.png")]
    pub enemy_ship: Handle<Image>,

    #[asset(path = "space/ships/bullet.png")]
    pub bullet: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 500., tile_size_y = 500., columns = 25, rows = 5))]
    #[asset(path = "space/celestials/star-yellow.png")]
    pub star: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 125., tile_size_y = 125., columns = 25, rows = 5))]
    #[asset(path = "space/celestials/earth.png")]
    pub earth: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 75., tile_size_y = 75., columns = 25, rows = 5))]
    #[asset(path = "space/celestials/mercury.png")]
    pub mercury: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 125., tile_size_y = 125., columns = 25, rows = 5))]
    #[asset(path = "space/celestials/venus.png")]
    pub venus: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 125., tile_size_y = 125., columns = 25, rows = 5))]
    #[asset(path = "space/celestials/mars.png")]
    pub mars: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 200., tile_size_y = 200., columns = 25, rows = 5))]
    #[asset(path = "space/celestials/jupiter.png")]
    pub jupiter: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 600., tile_size_y = 600., columns = 25, rows = 5))]
    #[asset(path = "space/celestials/saturn.png")]
    pub saturn: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 175., tile_size_y = 175., columns = 25, rows = 5))]
    #[asset(path = "space/celestials/uranus.png")]
    pub uranus: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 175., tile_size_y = 175., columns = 25, rows = 5))]
    #[asset(path = "space/celestials/neptune.png")]
    pub neptune: Handle<TextureAtlas>,

    #[asset(path = "space/backgrounds/custom.png")]
    pub background: Handle<Image>,

    #[asset(path = "space/meteors/meteorGrey_med1.png")]
    pub meteor: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "sound/music/Beat Mekanik - Lightspeed.ogg")]
    pub title_music: Handle<AudioSource>,

    #[asset(path = "sound/music/Kirk Osamayo - Space Dust.ogg")]
    pub ambience: Handle<AudioSource>,

    #[asset(path = "sound/sfx/impactPlank_medium_000.ogg")]
    pub gun: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "fonts/Xolonium/Xolonium-Bold.ttf")]
    pub font: Handle<Font>,

    #[asset(path = "images/verse.png")]
    pub title: Handle<Image>,
}
