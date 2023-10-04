use bevy::prelude::*;

use crate::{
    assets::SpriteAssets,
    orbit::{orbital_positioning_system, Orbit},
    planet::Planet,
    star::Star,
    state::AppState,
};

pub struct PlanetarySystemPlugin;
impl Plugin for PlanetarySystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameCreate), setup);
        app.add_systems(
            Update,
            (animate_sprite, orbital_positioning_system).run_if(in_state(AppState::Active)),
        );
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

fn setup(mut commands: Commands, sprites: Res<SpriteAssets>) {
    // Star
    let star_animation_indices = AnimationIndices {
        first: 0,
        last: 124,
    };

    // Planet
    let planet_animation_indices = AnimationIndices {
        first: 0,
        last: 124,
    };

    commands
        .spawn((
            SpriteSheetBundle {
                texture_atlas: sprites.star.clone(),
                sprite: TextureAtlasSprite::new(star_animation_indices.first),
                transform: Transform::from_scale(Vec3::splat(2.0)),
                ..default()
            },
            star_animation_indices,
            // TODO: .1 is too fast, .2 is too choppy; needs more animation frames.
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            Star {},
            Name::new("Star"),
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteSheetBundle {
                    texture_atlas: sprites.planet.clone(),
                    sprite: TextureAtlasSprite::new(planet_animation_indices.first),
                    transform: Transform::from_scale(Vec3::splat(2.0 / 2.0)), // Divide by parent scale?
                    ..default()
                },
                planet_animation_indices,
                // TODO: .1 is too fast, .2 is too choppy; needs more animation frames.
                AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                Planet {},
                Orbit {
                    semi_major_axis: 500.0 / 2.0, // Divide by parent scale?
                },
                Name::new("Planet"),
            ));
        });
}
