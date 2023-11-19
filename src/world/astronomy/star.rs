use bevy::prelude::*;

use crate::{
    core::effects::animate::{AnimationBundle, AnimationIndices, AnimationTimer},
    core::resources::assets::SpriteAssets,
    ui::hud::indicator::Indicated,
    world::spatial::KDNode,
};

use super::orbit::{Mass, Orbitable};

const STAR_ANIMATION_INDICES: AnimationIndices = AnimationIndices {
    first: 0,
    last: 124,
};

#[derive(Component, Clone, Debug)]
pub struct Star;

#[derive(Bundle)]
pub struct StarBundle {
    pub name: Name,
    pub marker: Star,
    pub indicated: Indicated,
    pub orbitable: Orbitable,
    pub animation_bundle: AnimationBundle,
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub kd_node: KDNode,
    pub mass: Mass,
}
impl Default for StarBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Star"),
            marker: Star,
            indicated: Indicated {
                color: Color::ANTIQUE_WHITE,
                ..default()
            },
            orbitable: Orbitable::ZERO,
            animation_bundle: AnimationBundle {
                indices: STAR_ANIMATION_INDICES,
                timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            },
            sprite_sheet_bundle: SpriteSheetBundle::default(),
            kd_node: KDNode,
            mass: Mass(1.989e30), // 1 Sol
        }
    }
}
impl StarBundle {
    #[allow(dead_code)]
    pub fn from_sprites(sprites: Res<SpriteAssets>) -> Self {
        Self {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: sprites.star.clone(),
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_scale(Vec3::splat(2.0)),
                ..default()
            },
            ..default()
        }
    }
}
