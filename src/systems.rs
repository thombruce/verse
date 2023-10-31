use bevy::prelude::*;

use crate::{
    core::{
        effects::{animate, blink},
        resources::{
            config, despawn_timer, game_time,
            state::{self, is_in_game_state, is_in_menu_state, GameState},
        },
    },
    ships::{player, ship},
    temp,
    ui::{
        camera, damage, hud,
        menus::{credits, pause, start_menu},
    },
    world::astronomy::{orbit, planetary_system, starfield},
};

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // PreStartup
        // app.add_systems(PreStartup, _);

        // Startup
        app.add_systems(Startup, temp::set_window_icon::set_window_icon);
        app.add_systems(Startup, camera::spawn_camera);

        // PostStartup
        // app.add_systems(PostStartup, _);

        // First
        // app.add_systems(First, _);

        // PreUpdate
        // app.add_systems(PreUpdate, _);

        // OnEnter
        // - Any
        for state in GameState::variants() {
            app.add_systems(OnEnter(state), state::state_enter_despawn::<GameState>);
        }

        // - StartMenu
        app.add_systems(OnEnter(GameState::StartMenu), starfield::spawn_starfield);
        app.add_systems(OnEnter(GameState::StartMenu), start_menu::spawn_start_menu);

        // - Credits
        app.add_systems(OnEnter(GameState::Credits), credits::spawn_credits);

        // - GameCreate
        app.add_systems(OnEnter(GameState::GameCreate), state::game_setup);
        app.add_systems(OnEnter(GameState::GameCreate), pause::setup_pause_systems);
        app.add_systems(
            OnEnter(GameState::GameCreate),
            (
                planetary_system::spawn_star,
                apply_deferred,
                planetary_system::spawn_planets,
                apply_deferred,
                planetary_system::spawn_demo_orbital,
            )
                .chain(),
        );
        app.add_systems(OnEnter(GameState::GameCreate), hud::spawn_hud);

        // - Paused
        app.add_systems(OnEnter(GameState::Paused), pause::pause_screen);

        // OnTransition
        app.add_systems(
            OnTransition {
                from: GameState::Loading,
                to: GameState::StartMenu,
            },
            config::apply_config,
        );

        app.add_systems(
            OnTransition {
                from: GameState::Loading,
                to: GameState::StartMenu,
            },
            start_menu::init_start_menu,
        );

        // OnExit
        app.add_systems(OnExit(GameState::Loading), config::load_config);
        app.add_systems(
            OnExit(GameState::GameCreate),
            hud::indicator::spawn_indicators,
        );

        // FixedUpdate
        // app.add_systems(FixedUpdate, _);

        // Update
        app.add_systems(Update, blink::menu_blink_system);

        app.add_systems(
            Update,
            credits::credits_system.run_if(in_state(GameState::Credits)),
        );

        app.add_systems(
            Update,
            (
                animate::animate_sprite,
                despawn_timer::despawn_system,
                game_time::tick_game_time,
                orbit::orbitable_update_system,
                orbit::orbital_positioning_system,
            )
                .run_if(in_state(GameState::Active)),
        );

        app.add_systems(
            Update,
            camera::follow_player
                // TODO: player_flight_system won't always be the only player control system
                //       Consider creating a SystemSet for the player control step (whatever
                //       it may be for the given GameState) and executing the follow_player
                //       system after that SystemSet.
                .after(player::player_flight_system)
                .run_if(in_state(GameState::Active)),
        );

        app.add_systems(
            Update,
            (damage::ui_spawn_damage, damage::ui_text_fade_out)
                .after(ship::AttackSet)
                .run_if(in_state(GameState::Active)),
        );

        app.add_systems(Update, pause::pause_system.run_if(is_in_game_state));

        app.add_systems(
            Update,
            start_menu::menu_input_system.run_if(is_in_menu_state),
        );

        app.add_systems(
            Update,
            (
                hud::speedometer::hud_speedometer,
                hud::health::hud_health,
                hud::nav::current_location,
                hud::time::current_time,
            )
                .run_if(in_state(GameState::Active)),
        );

        app.add_systems(
            Update,
            hud::indicator::indicators_system.run_if(in_state(GameState::Active)),
        );

        // PostUpdate
        app.add_systems(
            PostUpdate,
            hud::indicator::despawn_indicators_system.run_if(in_state(GameState::Active)),
        );

        // Last
        // app.add_systems(Last, _);
    }
}
