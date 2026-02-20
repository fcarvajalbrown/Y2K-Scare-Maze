//! Entry point for Y2K Scare Maze.
//! Registers all plugins, resources, systems, and state transitions.

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

mod states;
mod maze;
mod player;
mod enemies;
mod terminal;
mod combat;
mod ui;

use states::GameState;
use player::controller::{spawn_player, lock_cursor, player_look, player_move};
use player::stats::regenerate_stamina;
use player::combat::tick_attack_cooldown;
use terminal::ui::{render_terminal, tick_typewriter, TerminalStyle, TypewriterState};
use terminal::puzzle::TerminalPuzzle;
use combat::events::{AttackEvent, DamageEvent, DeathEvent};
use combat::system::{resolve_damage, resolve_attack, handle_death};
use ui::hud::render_hud;
use ui::game_over::render_game_over;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Y2K Scare Maze".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .init_state::<GameState>()
        // Resources
        .init_resource::<player::controller::MouseSensitivity>()
        .init_resource::<TerminalPuzzle>()
        .init_resource::<TerminalStyle>()
        .init_resource::<TypewriterState>()
        // Events
        .add_event::<AttackEvent>()
        .add_event::<DamageEvent>()
        .add_event::<DeathEvent>()
        // Startup systems
        .add_systems(Startup, (spawn_player, lock_cursor))
        // Exploring state systems
        .add_systems(Update, (
            player_look,
            player_move,
            regenerate_stamina,
            tick_attack_cooldown,
            render_hud,
        ).run_if(in_state(GameState::Exploring)))
        // AtTerminal state systems
        .add_systems(Update, (
            tick_typewriter,
            render_terminal,
        ).run_if(in_state(GameState::AtTerminal)))
        // Combat state systems (Phase 2)
        .add_systems(Update, (
            resolve_damage,
            resolve_attack,
            handle_death,
        ).run_if(in_state(GameState::Combat)))
        // GameOver state systems
        .add_systems(Update,
            render_game_over.run_if(in_state(GameState::GameOver))
        )
        .run();
}