use bevy::{
    prelude::*,
    render::{
        texture::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
        view::NoFrustumCulling,
    },
};

#[derive(Component)]
pub struct Parallax(pub f32);

/// The setup function
pub(crate) fn spawn_starfield(mut commands: Commands, assets: Res<AssetServer>) {
    let sampler_desc = ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..Default::default()
    };

    let settings = move |s: &mut ImageLoaderSettings| {
        s.sampler = ImageSampler::Descriptor(sampler_desc.clone());
    };

    let image = assets.load_with_settings("space/backgrounds/custom.png", settings);

    // TODO: The Starfield no longer moves parallax to the foreground.
    //       Find a way to reimplement this behaviour.
    commands.spawn((
        SpriteBundle {
            texture: image,
            sprite: Sprite {
                // TODO: Scale for a galaxy (change image, make procedural, etc.)
                rect: Some(Rect::new(-1_000_000., -1_000_000., 1_000_000., 1_000_000.)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., -899.),
                ..default()
            },
            ..default()
        },
        NoFrustumCulling,
        Name::new("Background"),
        Parallax(0.5),
    ));
}

pub(crate) fn parallax_effect(
    mut parallaxes: Query<(&mut Transform, &Parallax)>,
    camera: Query<&Transform, (With<Camera>, Without<Parallax>)>,
) {
    let camera_transform = camera.single();
    let camera_translation = camera_transform.translation.truncate();

    for (mut transform, parallax) in parallaxes.iter_mut() {
        let z = transform.translation.z;
        transform.translation = (camera_translation * parallax.0).extend(z);
    }
}
