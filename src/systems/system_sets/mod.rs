use bevy::prelude::*;

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub struct MovementSet;

#[derive(SystemSet, Clone, Hash, Debug, PartialEq, Eq)]
pub struct AttackSet;
