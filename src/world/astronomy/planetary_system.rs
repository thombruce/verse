use bevy::prelude::*;

use crate::{
    core::resources::{assets::SpriteAssets, state::GameState},
    ui::hud::indicator::Indicated,
};

use super::{
    orbit::{Orbit, OrbitPlugin},
    planet::{Planet, PlanetBundle},
    star::{Star, StarBundle},
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
    }
}

fn spawn_star(mut commands: Commands, sprites: Res<SpriteAssets>) {
    commands.spawn(StarBundle::from_sprites(sprites));
}

fn spawn_planets(
    mut commands: Commands,
    sprites: Res<SpriteAssets>,
    star_query: Query<Entity, With<Star>>,
) {
    commands.spawn(PlanetBundle {
        indicated: Indicated {
            color: Color::LIME_GREEN,
        },
        orbit: Orbit {
            parent: Some(star_query.single()),
            semi_major_axis: 5000.0,
        },
        sprite_sheet_bundle: SpriteSheetBundle {
            texture_atlas: sprites.planet.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_scale(Vec3::splat(2.0)), // Divide by parent scale?
            ..default()
        },
        ..default()
    });

    for (i, radius) in [10000.0, 50000.0].iter().enumerate() {
        commands.spawn(PlanetBundle {
            name: Name::new(format!("Rocky Planet {}", i + 1)),
            indicated: Indicated { color: Color::GRAY },
            orbit: Orbit {
                parent: Some(star_query.single()),
                semi_major_axis: *radius,
            },
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: sprites.noatmos.clone(),
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_scale(Vec3::splat(2.0)), // Divide by parent scale?
                ..default()
            },
            ..default()
        });
    }
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
            parent: planet_query.iter().min(),
            semi_major_axis: 250.0,
        },
        Name::new("Demo Orbital"),
    ));
}
