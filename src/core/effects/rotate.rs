use bevy::prelude::*;

#[derive(Component)]
pub struct Rotation(pub f32);

pub(crate) fn rotate_system(time: Res<Time>, mut query: Query<(&mut Transform, &Rotation)>) {
    for (mut transform, rotation) in query.iter_mut() {
        transform.rotate_z(rotation.0 * time.delta_seconds());
    }
}
