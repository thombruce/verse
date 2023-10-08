use bevy::prelude::*;

use crate::{
    assets::SpriteAssets,
    hud::indicator::Indicated,
    orbit::{Orbit, OrbitPlugin, Orbitable},
    planet::Planet,
    star::Star,
    state::GameState,
};

pub struct PlanetarySystemPlugin;
impl Plugin for PlanetarySystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OrbitPlugin);
        // TODO: Having some real trouble with ordering systems
        app.add_systems(
            OnEnter(GameState::GameCreate),
            (
                spawn_star,
                apply_deferred,
                spawn_planets,
                apply_deferred,
                spawn_demo_orbital,
            )
                .chain(),
        );
        app.add_systems(Update, animate_sprite.run_if(in_state(GameState::Active)));
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

fn spawn_star(mut commands: Commands, sprites: Res<SpriteAssets>) {
    // Star
    let star_animation_indices = AnimationIndices {
        first: 0,
        last: 124,
    };

    commands.spawn((
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
        Indicated {
            color: Color::ANTIQUE_WHITE,
        },
        Orbitable::ZERO,
        Name::new("Star"),
    ));
}

fn spawn_planets(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
    star_query: Query<Entity, With<Star>>,
) {
    // Planet
    let planet_animation_indices = AnimationIndices {
        first: 0,
        last: 124,
    };

    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: sprites.planet.clone(),
            sprite: TextureAtlasSprite::new(planet_animation_indices.first),
            transform: Transform::from_scale(Vec3::splat(2.0)), // Divide by parent scale?
            ..default()
        },
        planet_animation_indices,
        // TODO: .1 is too fast, .2 is too choppy; needs more animation frames.
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Planet {},
        Indicated {
            color: Color::LIME_GREEN,
        },
        Orbitable::default(),
        Orbit {
            parent: Some(star_query.single()),
            semi_major_axis: 5000.0,
        },
        Name::new("Planet"),
    ));
}

fn spawn_demo_orbital(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
    planet_query: Query<Entity, With<Planet>>,
) {
    commands.spawn((
        SpriteBundle {
            texture: sprites.meteor.clone(),
            transform: Transform {
                scale: Vec3::splat(0.5),
                ..default()
            },
            ..default()
        },
        Orbit {
            parent: Some(planet_query.single()),
            semi_major_axis: 250.0,
        },
        Name::new("Demo Orbital"),
    ));
}
