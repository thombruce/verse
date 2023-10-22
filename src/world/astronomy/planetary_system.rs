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
        mass: Mass(1.989e30),
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
                initial_mean_anomaly: f32::to_radians(0.), // TODO: Set me
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
        indicated: Indicated {
            color: Color::ORANGE,
        },
        orbit: Orbit {
            parent: Some(star_query.single()),
            semi_major_axis: 4000.0,
            eccentricity: 0.205630,
            argument_of_periapsis: f32::to_radians(29.124),
            initial_mean_anomaly: f32::to_radians(0.), //TODO: Set me
        },
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: sprites.mercury.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        mass: Mass(3.3011e23),
        ..default()
    });
    commands.spawn(PlanetBundle {
        name: Name::new("Venus"),
        indicated: Indicated {
            color: Color::ORANGE,
        },
        orbit: Orbit {
            parent: Some(star_query.single()),
            semi_major_axis: 10000.0,
            eccentricity: 0.006772,
            argument_of_periapsis: f32::to_radians(54.884),
            initial_mean_anomaly: f32::to_radians(0.), //TODO: Set me
        },
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: sprites.venus.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(1.8)),
            ..default()
        },
        mass: Mass(4.8675e24),
        ..default()
    });
    commands.spawn(PlanetBundle {
        name: Name::new("Mars"),
        indicated: Indicated {
            color: Color::ORANGE,
        },
        orbit: Orbit {
            parent: Some(star_query.single()),
            semi_major_axis: 30000.0,
            eccentricity: 0.0934,
            argument_of_periapsis: f32::to_radians(286.5),
            initial_mean_anomaly: f32::to_radians(0.), //TODO: Set me
        },
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: sprites.mars.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(1.6)),
            ..default()
        },
        mass: Mass(6.4171e23),
        ..default()
    });
    commands.spawn(PlanetBundle {
        name: Name::new("Jupiter"),
        indicated: Indicated {
            color: Color::SALMON,
        },
        orbit: Orbit {
            parent: Some(star_query.single()),
            semi_major_axis: 50000.0,
            eccentricity: 0.0489,
            argument_of_periapsis: f32::to_radians(273.867),
            initial_mean_anomaly: f32::to_radians(0.), //TODO: Set me
        },
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: sprites.jupiter.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(2.25)),
            ..default()
        },
        mass: Mass(1.8982e27),
        ..default()
    });
    commands.spawn(PlanetBundle {
        name: Name::new("Saturn"),
        indicated: Indicated {
            color: Color::SALMON,
        },
        orbit: Orbit {
            parent: Some(star_query.single()),
            semi_major_axis: 60000.0,
            eccentricity: 0.0565,
            argument_of_periapsis: f32::to_radians(339.392),
            initial_mean_anomaly: f32::to_radians(0.), //TODO: Set me
        },
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: sprites.saturn.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(1.75)),
            ..default()
        },
        mass: Mass(5.6834e26),
        ..default()
    });
    commands.spawn(PlanetBundle {
        name: Name::new("Uranus"),
        indicated: Indicated {
            color: Color::SALMON,
        },
        orbit: Orbit {
            parent: Some(star_query.single()),
            semi_major_axis: 70000.0,
            eccentricity: 0.04717,
            argument_of_periapsis: f32::to_radians(96.998857),
            initial_mean_anomaly: f32::to_radians(0.), //TODO: Set me
        },
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: sprites.uranus.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        mass: Mass(8.6810e25),
        ..default()
    });
    commands.spawn(PlanetBundle {
        name: Name::new("Neptune"),
        indicated: Indicated {
            color: Color::SALMON,
        },
        orbit: Orbit {
            parent: Some(star_query.single()),
            semi_major_axis: 80000.0,
            eccentricity: 0.008678,
            argument_of_periapsis: f32::to_radians(273.187),
            initial_mean_anomaly: f32::to_radians(0.), //TODO: Set me
        },
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: sprites.neptune.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        },
        mass: Mass(1.02413e26),
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
            eccentricity: 0.0549,
            // argument_of_periapsis: f32::to_radians(0.), // Cannot be set - it rotates
            initial_mean_anomaly: f32::to_radians(0.), //TODO: Set me
            ..default()
        },
        Name::new("Demo Orbital"),
    ));
}
