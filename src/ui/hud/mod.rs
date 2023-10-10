use bevy::prelude::*;

use crate::core::{resources::assets::UiAssets, resources::state::GameState};

pub mod indicator;
pub mod nav;
pub mod speedometer;

use indicator::IndicatorPlugin;
use nav::current_location;
use speedometer::hud_speedometer;

use self::{nav::NavBundle, speedometer::SpeedometerBundle};

pub struct HudPlugin;
impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(IndicatorPlugin);
        app.add_systems(OnEnter(GameState::GameCreate), setup);
        app.add_systems(
            Update,
            (hud_speedometer, current_location).run_if(in_state(GameState::Active)),
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
}
