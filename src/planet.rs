use bevy::prelude::*;

use crate::state::AppState;

pub struct PlanetPlugin;
impl Plugin for PlanetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameCreate), setup);
        app.add_systems(
            Update,
            (animate_sprite, orbital_positioning_system).run_if(in_state(AppState::Active)),
        );
    }
}

// TODO: Used regularly - move
#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Clone, Debug)]
struct Orbit {
    pub semi_major_axis: f32,
    // pub eccentricity: f32,
    // pub argument_of_periapsis: f32,
    // pub initial_mean_anomaly: f32,
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
    let texture_handle = asset_server.load("space/celestials/planet-pixelplanet.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(125.0, 125.0), 25, 5, None, None);
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
        Orbit {
            semi_major_axis: 500.0,
        },
        Name::new("Planet"),
    ));
}

/// Really basic circular motion around (0., 0.)
fn orbital_positioning_system(time: Res<Time>, mut orbits: Query<(&Orbit, &mut Transform)>) {
    for (orbit, mut transform) in orbits.iter_mut() {
        transform.translation.x = time.elapsed_seconds().cos() * orbit.semi_major_axis;
        transform.translation.y = time.elapsed_seconds().sin() * orbit.semi_major_axis;
    }
}
