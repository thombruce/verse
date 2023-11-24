// Adapted for Verse from https://github.com/BorisBoutillier/Kataster/blob/main/src/background.rs

use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2dPlugin, Mesh2dHandle};
use bevy::{
    reflect::TypeUuid,
    sprite::{Material2d, MaterialMesh2dBundle},
};

use crate::systems::states::GameState;

use super::starfield::Parallax;

pub const ARENA_WIDTH: f32 = 1500.0; // TODO: Change me
pub const ARENA_HEIGHT: f32 = 750.0; // TODO: Change me

// Plugin that will insert a background at Z = -10.0, use the custom 'Star Nest' shader
pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<BackgroundMaterial>::default())
            .add_systems(Startup, spawn_background)
            .add_systems(Update, update_background_time);
    }
}

// Spawn a simple stretched quad that will use of backgound shader
fn spawn_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                translation: Vec3::new(0.0, -6000.0, 0.0), // TODO: Probably follow player/camera??
                scale: Vec3::new(ARENA_WIDTH, ARENA_HEIGHT, 1.0),
                ..default()
            },
            material: materials.add(BackgroundMaterial { x: 0.0 }),
            ..default()
        },
        Parallax(1.0),
    ));
}

#[derive(Asset, AsBindGroup, Debug, Clone, TypeUuid, TypePath)]
#[uuid = "d1776d38-712a-11ec-90d6-0242ac120003"]
struct BackgroundMaterial {
    #[uniform(0)]
    x: f32,
}

impl Material2d for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/background.wgsl".into()
    }
}

fn update_background_time(
    time: Res<Time>,
    state: Res<State<GameState>>,
    mut backgrounds: ResMut<Assets<BackgroundMaterial>>,
    mesh: Query<&Transform, (With<Mesh2dHandle>, With<Parallax>)>,
) {
    if state.get() != &GameState::Paused {
        for (_, background) in backgrounds.iter_mut() {
            // background.time += time.delta_seconds();

            let mesh = mesh.single();
            background.x = mesh.translation.x;
        }
    }
}
