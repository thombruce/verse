use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PauseAction {
    Pause,
}

pub fn pause_input_map() -> InputMap<PauseAction> {
    let mut input_map = InputMap::<PauseAction>::new([
        (KeyCode::Escape, PauseAction::Pause),
        (KeyCode::P, PauseAction::Pause),
    ]);
    input_map.insert(GamepadButtonType::Start, PauseAction::Pause);

    return input_map.build();
}
