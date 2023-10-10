use bevy::prelude::*;

use crate::{
    hud::HudPlugin,
    menus::{credits::CreditsPlugin, pause::PausePlugin, start_menu::MenuPlugin},
    resources::camera::CameraPlugin,
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
        ));
    }
}
