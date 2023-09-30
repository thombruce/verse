use bevy::prelude::*;
use bevy_rapier2d::prelude::RapierConfiguration;
use leafwing_input_manager::{
    prelude::{ActionState, InputMap},
    Actionlike,
};

#[derive(States, Debug, Copy, Clone, Hash, Eq, PartialEq, Default)]
pub enum AppState {
    #[default]
    Active,
    Paused,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PauseAction {
    Pause,
}

#[derive(Component, Debug)]
pub struct PauseState {}

#[derive(Component)]
pub struct DrawBlinkTimer(pub Timer);

pub struct StatePlugin;
impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, (pause_system, menu_blink_system))
            .add_systems(OnEnter(AppState::Paused), pause_screen)
            .add_systems(OnExit(AppState::Paused), despawn);
    }
}

fn setup(mut commands: Commands) {
    let input_map = InputMap::<PauseAction>::new([
        (KeyCode::Escape, PauseAction::Pause),
        (KeyCode::P, PauseAction::Pause),
    ]);

    commands.insert_resource(input_map);
    commands.insert_resource(ActionState::<PauseAction>::default());
}

fn pause_system(
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    pause_action_state: Res<ActionState<PauseAction>>,
    mut rapier_configuration: ResMut<RapierConfiguration>,
) {
    if pause_action_state.just_pressed(PauseAction::Pause) {
        match state.get() {
            AppState::Active => {
                next_state.set(AppState::Paused);
                rapier_configuration.physics_pipeline_active = false;
            }
            AppState::Paused => {
                next_state.set(AppState::Active);
                rapier_configuration.physics_pipeline_active = true;
            }
        }
    }
}

fn pause_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0., 0., 0., 0.65)),
                ..default()
            },
            PauseState {},
        ))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    style: Style { ..default() },
                    text: Text::from_section(
                        "Pause",
                        TextStyle {
                            font: asset_server.load("fonts/kenvector_future.ttf"),
                            font_size: 50.0,
                            color: Color::rgb_u8(0x88, 0x00, 0x00),
                        },
                    ),
                    ..default()
                },
                DrawBlinkTimer(Timer::from_seconds(0.65, TimerMode::Repeating)),
            ));
        });
}

fn menu_blink_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DrawBlinkTimer, &ComputedVisibility)>,
) {
    for (entity, mut timer, visibility) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            let new_visibility = if visibility.is_visible() {
                Visibility::Hidden
            } else {
                Visibility::Visible
            };
            commands.entity(entity).insert(new_visibility);
        }
    }
}

fn despawn(mut commands: Commands, query: Query<Entity, With<PauseState>>) {
    for entity in &mut query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
