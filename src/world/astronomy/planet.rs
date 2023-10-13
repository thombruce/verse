use bevy::prelude::*;

use crate::{
    core::effects::animate::{AnimationBundle, AnimationIndices, AnimationTimer},
    ui::hud::indicator::Indicated,
    world::spatial::KDNode,
};

use super::orbit::{Orbit, Orbitable};

const PLANET_ANIMATION_INDICES: AnimationIndices = AnimationIndices {
    first: 0,
    last: 124,
};

#[derive(Component, Clone, Debug)]
pub struct Planet;

#[derive(Bundle)]
pub struct PlanetBundle {
    pub name: Name,
    pub marker: Planet,
    pub indicated: Indicated,
    pub orbitable: Orbitable,
    pub orbit: Orbit,
    pub animation_bundle: AnimationBundle,
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub kd_node: KDNode,
}
impl Default for PlanetBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Planet"),
            marker: Planet,
            indicated: Indicated {
                color: Color::ORANGE_RED,
            },
            orbitable: Orbitable::default(),
            orbit: Orbit::default(),
            animation_bundle: AnimationBundle {
                indices: PLANET_ANIMATION_INDICES,
                timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
            },
            sprite_sheet_bundle: SpriteSheetBundle::default(),
            kd_node: KDNode,
        }
    }
}
