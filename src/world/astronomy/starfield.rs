use bevy::{prelude::*, render::view::NoFrustumCulling};
use bevy_tiling_background::{
    BackgroundImageBundle, BackgroundMaterial, SetImageRepeatingExt, TilingBackgroundPlugin,
};

use crate::core::resources::assets::SpriteAssets;

pub struct StarfieldPlugin;
impl Plugin for StarfieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TilingBackgroundPlugin::<BackgroundMaterial>::default());
    }
}

/// The setup function
pub(crate) fn spawn_starfield(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    let image = sprites.background.clone();
    // Queue a command to set the image to be repeating once the image is loaded.
    commands.set_image_repeating(image.clone());

    commands.spawn((
        BackgroundImageBundle::from_image(image, materials.as_mut())
            .at_z_layer(-0.1)
            .with_movement_scale(0.1),
        NoFrustumCulling,
        Name::new("Background"),
    ));
}
