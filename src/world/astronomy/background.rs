// Adapted for Verse from https://github.com/BorisBoutillier/Kataster/blob/main/src/background.rs

use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::render::view::NoFrustumCulling;
use bevy::sprite::Material2dPlugin;
use bevy::{
    reflect::TypeUuid,
    sprite::{Material2d, MaterialMesh2dBundle},
};

use crate::systems::states::GameState;

use super::starfield::Parallax;

// Plugin that will insert a background at Z = -10.0, use the custom 'Star Nest' shader
pub struct BackgroundPlugin;
impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<BackgroundMaterial>::default());
    }
}

#[derive(Component, Clone, Debug)]
pub struct Starfield;

// Spawn a simple stretched quad that will use of backgound shader
pub(crate) fn spawn_background(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<BackgroundMaterial>>,
    camera: Query<&Transform, (With<Camera>, Without<Parallax>)>,
) {
    let camera_xy = camera.single().translation.truncate();

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform {
                translation: camera_xy.extend(-898.0),
                scale: Vec3::new(5000.0, 5000.0, 1.0),
                ..default()
            },
            material: materials.add(BackgroundMaterial {
                position: Vec2::new(0.0, 0.0),
            }),
            ..default()
        },
        NoFrustumCulling,
        Name::new("Background"),
        Parallax(1.0),
        Starfield,
    ));
}

#[derive(Asset, AsBindGroup, Debug, Clone, TypeUuid, TypePath)]
#[uuid = "d1776d38-712a-11ec-90d6-0242ac120003"]
pub(crate) struct BackgroundMaterial {
    #[uniform(0)]
    position: Vec2,
}

impl Material2d for BackgroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/background.wgsl".into()
    }
}

pub(crate) fn update_background_position(
    state: Res<State<GameState>>,
    mut backgrounds: ResMut<Assets<BackgroundMaterial>>,
    mesh: Query<&Transform, With<Starfield>>,
) {
    if state.get() != &GameState::Paused {
        for (_, background) in backgrounds.iter_mut() {
            let mesh = mesh.single();
            background.position = mesh.translation.truncate();
        }
    }
}
