use bevy::prelude::*;

pub mod camera;
pub mod damage;
pub mod hud;
pub mod menus;

use self::{
    camera::CameraPlugin,
    damage::UiDamagePlugin,
    hud::HudPlugin,
    menus::{credits::CreditsPlugin, pause::PausePlugin, start_menu::MenuPlugin},
};

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            HudPlugin,
            MenuPlugin,
            CreditsPlugin,
            PausePlugin,
            CameraPlugin,
            UiDamagePlugin,
        ));
    }
}
