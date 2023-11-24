use bevy::prelude::*;

#[derive(Component)]
pub struct Parallax(pub f32);

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
