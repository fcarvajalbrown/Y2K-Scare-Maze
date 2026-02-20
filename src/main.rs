//! Entry point for Y2K Scare Maze.
//! Registers all plugins, resources, systems, and state transitions.

use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_rapier3d::prelude::*;
use maze::renderer::spawn_maze;
use maze::generator::Maze;
use bevy_egui::egui;

mod states;
mod maze;
mod player;
mod enemies;
mod terminal;
mod combat;
mod ui;
mod audio;
mod game_timer;

// Imports from modules
use states::GameState;
use player::controller::{spawn_player, lock_cursor, player_look, player_move};
use player::stats::regenerate_stamina;
use player::combat::tick_attack_cooldown;
use terminal::ui::{render_terminal, tick_typewriter, TerminalStyle, TypewriterState};
use terminal::puzzle::TerminalPuzzle;
use combat::events::{AttackEvent, DamageEvent, DeathEvent};
use combat::system::{resolve_damage, resolve_attack, handle_death};
use ui::hud::render_hud;
use terminal::spawner::{spawn_terminal, interact_terminal};
use ui::game_over::{render_game_over, render_win};
use audio::clock::{start_clock_audio, stop_clock_audio};
use terminal::monitor::spawn_monitor;
use game_timer::{GameTimer, tick_timer, reset_timer};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Y2K Scare Maze".into(),
                mode: bevy::window::WindowMode::BorderlessFullscreen(
                    bevy::window::MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .init_state::<GameState>()
        // Resources
        .init_resource::<player::controller::MouseSensitivity>()
        .init_resource::<TerminalPuzzle>()
        .init_resource::<TerminalStyle>()
        .init_resource::<TypewriterState>()
        .init_resource::<GameTimer>()
        // Events
        .add_event::<AttackEvent>()
        .add_event::<DamageEvent>()
        .add_event::<DeathEvent>()
        // Startup systems
        .insert_resource(Maze::generate(12345))
        .add_systems(Update, (
            player_look,
            player_move,
            regenerate_stamina,
            tick_attack_cooldown,
            render_hud,
            interact_terminal,
            tick_timer,
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
        .add_systems(Startup, (
            spawn_player, 
            lock_cursor, 
            spawn_maze, 
            spawn_terminal, 
            spawn_monitor, 
            start_clock_audio,
            load_fonts))
        // GameOver state systems
        .add_systems(Update,
            render_game_over.run_if(in_state(GameState::GameOver))
        )
        // Win Render Screen
        .add_systems(Update,
            render_win.run_if(in_state(GameState::Win))
        )
        .add_systems(OnEnter(GameState::Win), 
            stop_clock_audio
        )
        .add_systems(OnEnter(GameState::GameOver), 
            stop_clock_audio
        )
        .add_systems(OnEnter(GameState::Exploring), 
        reset_timer
    )
        .run();
}

// Resets the game state to the initial conditions for a new playthrough.
fn reset_game(
    mut player_query: Query<&mut Transform, With<player::controller::Player>>,
    mut puzzle: ResMut<terminal::puzzle::TerminalPuzzle>,
    mut tw: ResMut<terminal::ui::TypewriterState>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        transform.translation = Vec3::new(1.0, 0.5, 1.0);
    }
    *puzzle = terminal::puzzle::TerminalPuzzle::default();
    tw.reset();
}

fn load_fonts(mut contexts: bevy_egui::EguiContexts) {
    let ctx = contexts.ctx_mut();
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "ds_digital".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/fonts/ds_digital.ttf")),
    );
    fonts.families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "ds_digital".to_owned());
    ctx.set_fonts(fonts);
}