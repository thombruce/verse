use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum MenuAction {
    Select,
    Start,
    Settings,
    Credits,
    Quit,
}

pub fn menu_input_map() -> InputMap<MenuAction> {
    let mut input_map = InputMap::<MenuAction>::new([
        // Action Keys
        (KeyCode::Return, MenuAction::Select),
        // Hotkeys
        (KeyCode::N, MenuAction::Start),
        (KeyCode::S, MenuAction::Settings),
        (KeyCode::C, MenuAction::Credits),
        (KeyCode::Q, MenuAction::Quit),
    ]);
    // Action Buttons
    input_map.insert(GamepadButtonType::South, MenuAction::Select);
    // Hotkey Buttons
    input_map.insert(GamepadButtonType::Start, MenuAction::Start);
    input_map.insert(GamepadButtonType::North, MenuAction::Credits);

    return input_map.build();
}
