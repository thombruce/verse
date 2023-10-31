use bevy::prelude::*;

pub mod camera;
pub mod damage;
pub mod hud;
pub mod menus;
pub mod resources;

use self::{
    camera::CameraPlugin,
    menus::{credits::CreditsPlugin, pause::PausePlugin, start_menu::MenuPlugin},
};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MenuPlugin, CreditsPlugin, PausePlugin, CameraPlugin));
    }
}
