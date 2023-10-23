use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

/* Ship Actions */

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum ShipAction {
    Forward,
    RotateLeft,
    RotateRight,
    Fire,
}

pub fn ship_input_map() -> InputMap<ShipAction> {
    let mut input_map = InputMap::new([
        // Cursor keys
        (KeyCode::Up, ShipAction::Forward),
        (KeyCode::Left, ShipAction::RotateLeft),
        (KeyCode::Right, ShipAction::RotateRight),
        // WASD
        (KeyCode::W, ShipAction::Forward),
        (KeyCode::A, ShipAction::RotateLeft),
        (KeyCode::D, ShipAction::RotateRight),
        // Actions
        (KeyCode::Space, ShipAction::Fire),
    ]);
    // Gamepad
    input_map.insert(GamepadButtonType::RightTrigger2, ShipAction::Forward);
    input_map.insert(
        SingleAxis::positive_only(GamepadAxisType::LeftStickX, 0.4),
        ShipAction::RotateRight,
    );
    input_map.insert(
        SingleAxis::negative_only(GamepadAxisType::LeftStickX, -0.4),
        ShipAction::RotateLeft,
    );
    input_map.insert(GamepadButtonType::South, ShipAction::Fire);

    return input_map.build();
}
