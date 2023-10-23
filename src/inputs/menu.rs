use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum MenuAction {
    Start,
    Credits,
}

pub fn menu_input_map() -> InputMap<MenuAction> {
    let mut input_map = InputMap::<MenuAction>::new([
        (KeyCode::Return, MenuAction::Start),
        (KeyCode::C, MenuAction::Credits),
    ]);
    input_map.insert(GamepadButtonType::Start, MenuAction::Start);
    input_map.insert(GamepadButtonType::North, MenuAction::Credits);

    return input_map.build();
}
