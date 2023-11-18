use bevy::prelude::*;

use crate::core::resources::assets::UiAssets;

pub mod health;
pub mod indicator;
pub mod nav;
pub mod score;
pub mod speedometer;
pub mod time;

use self::{
    health::HealthBundle, nav::NavBundle, score::ScoreBundle, speedometer::SpeedometerBundle,
};

pub(crate) fn spawn_hud(mut commands: Commands, ui: Res<UiAssets>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::SpaceBetween,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            Name::new("HUD"),
        ))
        .with_children(|parent| {
            parent.spawn(NavBundle::use_font(ui.font.clone()));
            parent.spawn(SpeedometerBundle::use_font(ui.font.clone()));
        });
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::FlexStart,
                    justify_content: JustifyContent::SpaceBetween,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            },
            Name::new("HUD"),
        ))
        .with_children(|parent| {
            parent.spawn(ScoreBundle::use_font(ui.font.clone()));
            parent.spawn(HealthBundle::use_font(ui.font.clone()));
        });
}
