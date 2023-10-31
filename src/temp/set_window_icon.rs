use bevy::{prelude::*, window::PrimaryWindow, winit::WinitWindows};
use winit::window::Icon;

pub struct SetWindowIconPlugin;
impl Plugin for SetWindowIconPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_window_icon);
    }
}

// Documented:
// - https://bevy-cheatbook.github.io/window/icon.html
// - https://stackoverflow.com/a/76729516/2225649
// Open issue: https://github.com/bevyengine/bevy/issues/1031
// TODO: Change to official approach when issue is closed and released
pub fn set_window_icon(
    main_window: Query<Entity, With<PrimaryWindow>>,
    // we have to use `NonSend` here
    windows: NonSend<WinitWindows>,
) {
    let primary = windows.get_window(main_window.single()).unwrap();

    // here we use the `image` crate to load our icon data from a png file
    // this is not a very bevy-native solution, but it will do
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open("assets/images/VerseSquircle-256.png")
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };

    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    primary.set_window_icon(Some(icon));
}
