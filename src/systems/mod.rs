use bevy::prelude::*;
use bevy_ui_navigation::prelude::NavRequestSystem;

pub mod events;
pub mod states;
pub mod system_sets;

use crate::{
    core::{
        effects,
        resources::{self, config},
    },
    i18n,
    ships::{self, bullet, dynamic_orbit, enemy, player, ship},
    temp,
    ui::{
        camera, damage, hud,
        menus::{credits, pause, start_menu},
    },
    world::astronomy::{self, planetary_system, starfield},
};

use self::{
    states::{is_in_game_state, is_in_menu_state, GameState},
    system_sets::{AttackSet, MovementSet},
};

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Configure system set ordering:
        // app.configure_sets(Update, (MovementSet, AttackSet).chain());
        // app.configure_sets(Update, MovementSet.before(AttackSet));
        // app.configure_sets(Update, AttackSet.after(MovementSet));

        // PreStartup
        app.add_systems(PreStartup, temp::set_window_icon::set_window_icon);

        // Startup
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
            app.add_systems(
                OnEnter(state),
                states::transitions::state_enter_despawn::<GameState>,
            );
        }

        // - StartMenu
        app.add_systems(
            OnEnter(GameState::StartMenu),
            (starfield::spawn_starfield, start_menu::spawn_start_menu),
        );

        // - Credits
        app.add_systems(OnEnter(GameState::Credits), credits::spawn_credits);

        // - GameCreate
        app.add_systems(
            OnEnter(GameState::GameCreate),
            (
                states::transitions::game_setup,
                pause::setup_pause_systems,
                ships::configure_physics_engine,
                hud::spawn_hud,
                enemy::spawn_enemies,
                player::spawn_player,
                // Planetary system spawning, chained
                (
                    planetary_system::spawn_star,
                    apply_deferred,
                    planetary_system::spawn_planets,
                    apply_deferred,
                    planetary_system::spawn_demo_orbital,
                )
                    .chain(),
            ),
        );

        // - Paused
        app.add_systems(OnEnter(GameState::Paused), pause::pause_screen);

        // OnTransition
        app.add_systems(
            OnTransition {
                from: GameState::Loading,
                to: GameState::LoadingTranslations,
            },
            config::apply_config,
        );
        app.add_systems(
            OnTransition {
                from: GameState::LoadingTranslations,
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
        // app.insert_resource(FixedTime::new_from_secs(1.0 / 60.0));
        // app.add_systems(FixedUpdate, _);

        // NOTE: Alternative approach for running arbitrary systems on a fixed timer:
        // app.add_systems(
        //     Update,
        //     _.run_if(on_timer(Duration::from_secs(1.0 / 60.0))),
        // );

        // Update
        app.add_systems(
            Update,
            i18n::load_translations.run_if(in_state(GameState::LoadingTranslations)),
        );

        app.add_systems(
            Update,
            start_menu::menu_input_system.run_if(is_in_menu_state),
        );
        app.add_systems(
            Update,
            start_menu::menu_focus_system
                .after(NavRequestSystem)
                .run_if(is_in_menu_state),
        );

        app.add_systems(
            Update,
            credits::credits_system.run_if(in_state(GameState::Credits)),
        );

        app.add_systems(Update, pause::pause_system.run_if(is_in_game_state));

        app.add_systems(Update, effects::blink::menu_blink_system);

        app.add_systems(
            Update,
            (
                // NOTE: Maximum of 20 entries
                resources::game_time::tick_game_time,
                resources::despawn_timer::despawn_system,
                effects::animate::animate_sprite,
                astronomy::orbit::orbitable_update_system,
                astronomy::orbit::orbital_positioning_system,
                ship::bullet_timers_system,
                dynamic_orbit::dynamic_orbital_positioning_system,
                hud::indicator::indicators_system,
                hud::speedometer::hud_speedometer,
                hud::health::hud_health,
                hud::nav::current_location,
                hud::time::current_time,
                // TODO: This clumsily resolves a crash to desktop where enemy_targeting_system
                //       was happening after ship_damage some of the time.
                //       The SystemSets aren't aware of one another, they need to be configured.
                //       Reassess and order systems appropriately.
                enemy::enemy_targeting_system.before(ship::ship_damage),
                (player::player_flight_system, enemy::enemy_flight_system).in_set(MovementSet),
                (bullet::spawn_bullet, camera::follow_player).after(MovementSet),
                (
                    enemy::enemy_weapons_system,
                    player::player_weapons_system,
                    events::contact_system,
                )
                    .in_set(AttackSet),
                (
                    ship::ship_damage,
                    damage::ui_spawn_damage,
                    damage::ui_text_fade_out,
                )
                    .after(AttackSet),
            )
                .run_if(in_state(GameState::Active)),
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
