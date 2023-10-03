use bevy::prelude::*;

use crate::state::AppState;

pub struct StarPlugin;
impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameCreate), setup);
        app.add_systems(Update, animate_sprite.run_if(in_state(AppState::Active)));
    }
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("space/celestials/star-pixelplanet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(500.0, 500.0), 25, 5, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let animation_indices = AnimationIndices {
        first: 0,
        last: 124,
    };
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        animation_indices,
        // TODO: .1 is too fast, .2 is too choppy; needs more animation frames.
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
