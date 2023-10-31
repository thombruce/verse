use bevy::prelude::*;

#[derive(Component)]
pub struct DespawnTimer(pub Timer);

pub(crate) fn despawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DespawnTimer)>,
) {
    for (entity, mut bullet) in query.iter_mut() {
        bullet.0.tick(time.delta());
        if bullet.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}
