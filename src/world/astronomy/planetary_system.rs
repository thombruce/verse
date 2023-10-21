use bevy::prelude::*;

use crate::{
    core::resources::{assets::SpriteAssets, state::GameState},
    ui::hud::indicator::Indicated,
};

use super::{
    orbit::{Mass, Orbit, OrbitPlugin},
    planet::PlanetBundle,
    star::{Star, StarBundle},
};

#[derive(Component)]
pub struct EarthLike;

pub struct PlanetarySystemPlugin;
impl Plugin for PlanetarySystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(OrbitPlugin);
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
    }
}

fn spawn_star(mut commands: Commands, sprites: Res<SpriteAssets>) {
    commands.spawn(StarBundle {
        name: Name::new("Sol"),
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: sprites.star.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        ..default()
    });
}

fn spawn_planets(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
    star_query: Query<Entity, With<Star>>,
) {
    commands.spawn((
        PlanetBundle {
            name: Name::new("Earth"),
            indicated: Indicated {
                color: Color::LIME_GREEN,
            },
            orbit: Orbit {
                parent: Some(star_query.single()),
                semi_major_axis: 20000.0,
                eccentricity: 0.0167086,
                argument_of_periapsis: f32::to_radians(288.1),
                initial_mean_anomaly: f32::to_radians(0.),
            },
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: sprites.earth.clone(),
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_scale(Vec3::splat(2.0)),
                ..default()
            },
            mass: Mass(5.9722e24),
            ..default()
        },
        EarthLike,
    ));

    commands.spawn(PlanetBundle {
        name: Name::new("Mercury"),
        indicated: Indicated { color: Color::GRAY },
        orbit: Orbit {
            parent: Some(star_query.single()),
            semi_major_axis: 4000.0,
            ..default()
        },
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: sprites.mercury.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        ..default()
    });
    commands.spawn(PlanetBundle {
        name: Name::new("Venus"),
        indicated: Indicated { color: Color::GRAY },
        orbit: Orbit {
            parent: Some(star_query.single()),
            semi_major_axis: 10000.0,
            ..default()
        },
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: sprites.venus.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(1.8)),
            ..default()
        },
        ..default()
    });
    commands.spawn(PlanetBundle {
        name: Name::new("Mars"),
        indicated: Indicated { color: Color::GRAY },
        orbit: Orbit {
            parent: Some(star_query.single()),
            semi_major_axis: 30000.0,
            ..default()
        },
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: sprites.mars.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(1.6)),
            ..default()
        },
        ..default()
    });
    commands.spawn(PlanetBundle {
        name: Name::new("Jupiter"),
        indicated: Indicated { color: Color::GRAY },
        orbit: Orbit {
            parent: Some(star_query.single()),
            semi_major_axis: 50000.0,
            ..default()
        },
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: sprites.jupiter.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(2.25)),
            ..default()
        },
        ..default()
    });
    commands.spawn(PlanetBundle {
        name: Name::new("Saturn"),
        indicated: Indicated { color: Color::GRAY },
        orbit: Orbit {
            parent: Some(star_query.single()),
            semi_major_axis: 60000.0,
            ..default()
        },
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: sprites.saturn.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(1.75)),
            ..default()
        },
        ..default()
    });
    commands.spawn(PlanetBundle {
        name: Name::new("Uranus"),
        indicated: Indicated { color: Color::GRAY },
        orbit: Orbit {
            parent: Some(star_query.single()),
            semi_major_axis: 70000.0,
            ..default()
        },
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: sprites.uranus.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        ..default()
    });
    commands.spawn(PlanetBundle {
        name: Name::new("Neptune"),
        indicated: Indicated { color: Color::GRAY },
        orbit: Orbit {
            parent: Some(star_query.single()),
            semi_major_axis: 80000.0,
            ..default()
        },
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: sprites.neptune.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        ..default()
    });
}

fn spawn_demo_orbital(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
    planet_query: Query<Entity, With<EarthLike>>,
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
            parent: planet_query.iter().min(),
            semi_major_axis: 250.0,
            ..default()
        },
        Name::new("Demo Orbital"),
    ));
}
