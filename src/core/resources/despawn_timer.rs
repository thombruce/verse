use bevy::prelude::*;

use crate::core::resources::state::GameState;

#[derive(Component)]
pub struct DespawnTimer(pub Timer);

pub struct DespawnTimerPlugin;
impl Plugin for DespawnTimerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_system.run_if(in_state(GameState::Active)));
    }
}

fn despawn_system(
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
