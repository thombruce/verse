use bevy::prelude::*;

use crate::core::{resources::assets::UiAssets, resources::state::GameState};

pub mod health;
pub mod indicator;
pub mod nav;
pub mod speedometer;
pub mod time;

use health::hud_health;
use indicator::IndicatorPlugin;
use nav::current_location;
use speedometer::hud_speedometer;

use self::{
    health::HealthBundle,
    nav::NavBundle,
    speedometer::SpeedometerBundle,
    time::{current_time, TimeBundle},
};

pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(IndicatorPlugin);
        app.add_systems(OnEnter(GameState::GameCreate), setup);
        app.add_systems(
            Update,
            (hud_speedometer, hud_health, current_location, current_time)
                .run_if(in_state(GameState::Active)),
        );
    }
}

fn setup(mut commands: Commands, ui: Res<UiAssets>) {
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
            parent.spawn(TimeBundle::use_font(ui.font.clone()));
            parent.spawn(HealthBundle::use_font(ui.font.clone()));
        });
}
