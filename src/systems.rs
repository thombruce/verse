use bevy::prelude::*;

use crate::{
    core::{
        effects::{animate, blink},
        resources::{
            config, despawn_timer, game_time,
            state::{self, GameState},
        },
    },
    temp,
};

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // PreStartup
        // app.add_systems(PreStartup, _);

        // Startup
        app.add_systems(Startup, temp::set_window_icon::set_window_icon);

        // PostStartup
        // app.add_systems(PostStartup, _);

        // First
        // app.add_systems(First, _);

        // PreUpdate
        // app.add_systems(PreUpdate, _);

        // OnEnter
        app.add_systems(OnEnter(GameState::GameCreate), state::game_setup);
        for state in GameState::variants() {
            app.add_systems(OnEnter(state), state::state_enter_despawn::<GameState>);
        }

        // OnTransition
        app.add_systems(
            OnTransition {
                from: GameState::Loading,
                to: GameState::StartMenu,
            },
            config::apply_config,
        );

        // OnExit
        app.add_systems(OnExit(GameState::Loading), config::load_config);

        // FixedUpdate
        // app.add_systems(FixedUpdate, _);

        // Update
        app.add_systems(Update, blink::menu_blink_system);

        app.add_systems(
            Update,
            (
                animate::animate_sprite,
                despawn_timer::despawn_system,
                game_time::tick_game_time,
            )
                .run_if(in_state(GameState::Active)),
        );

        // PostUpdate
        // app.add_systems(PostUpdate, _);

        // Last
        // app.add_systems(Last, _);
    }
}
