use bevy::prelude::*;

/// Global game state machine.
///
/// - `Exploring`: player navigates the maze
/// - `AtTerminal`: player reached the fix terminal, puzzle UI shown
/// - `Combat`: enemy engagement (phase 2)
/// - `Win`: puzzle solved
/// - `GameOver`: player died (phase 2)
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Exploring,
    AtTerminal,
    Combat,
    Win,
    GameOver,
}