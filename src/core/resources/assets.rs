use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct SpriteAssets {
    #[asset(path = "space/ships/player.png")]
    pub player_ship: Handle<Image>,

    #[asset(path = "space/ships/enemy.png")]
    pub enemy_ship: Handle<Image>,

    #[asset(path = "space/ships/bullet.png")]
    pub bullet: Handle<Image>,

    #[asset(texture_atlas(tile_size_x = 500., tile_size_y = 500., columns = 25, rows = 5))]
    #[asset(path = "space/celestials/star-pixelplanet.png")]
    pub star: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 125., tile_size_y = 125., columns = 25, rows = 5))]
    #[asset(path = "space/celestials/planet-pixelplanet.png")]
    pub planet: Handle<TextureAtlas>,

    #[asset(texture_atlas(tile_size_x = 75., tile_size_y = 75., columns = 25, rows = 5))]
    #[asset(path = "space/celestials/noatmos-pixelplanet.png")]
    pub noatmos: Handle<TextureAtlas>,

    #[asset(path = "space/backgrounds/custom.png")]
    pub background: Handle<Image>,

    #[asset(path = "space/meteors/meteorGrey_med1.png")]
    pub meteor: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "sound/Beat Mekanik - Lightspeed.ogg")]
    pub title_music: Handle<AudioSource>,

    #[asset(path = "sound/Kirk Osamayo - Space Dust.ogg")]
    pub ambience: Handle<AudioSource>,

    #[asset(path = "sound/sfx/impactPlank_medium_000.ogg")]
    pub gun: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "fonts/Xolonium/Xolonium-Bold.ttf")]
    pub font: Handle<Font>,

    #[asset(path = "verse.png")]
    pub title: Handle<Image>,
}
