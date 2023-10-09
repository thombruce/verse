use bevy::prelude::*;

use crate::resources::state::GameState;

pub struct AnimatePlugin;
impl Plugin for AnimatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_sprite.run_if(in_state(GameState::Active)));
    }
}

#[derive(Component, Clone, Copy)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Bundle)]
pub struct AnimationBundle {
    pub indices: AnimationIndices,
    pub timer: AnimationTimer,
}

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}
