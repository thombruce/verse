use bevy::prelude::*;

use crate::{
    effects::{animate::AnimatePlugin, blink::EffectsPlugin},
    resources::{game_time::GameTimePlugin, state::StatePlugin},
};

pub struct CorePlugin;
impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GameTimePlugin, StatePlugin, EffectsPlugin, AnimatePlugin));
    }
}
