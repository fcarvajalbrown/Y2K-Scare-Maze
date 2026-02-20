//! Terminal puzzle logic.
//! Player must fix the Y2K bug by correcting a pseudocode snippet.

use bevy::prelude::*;

/// All possible puzzle states.
#[derive(Debug, Clone, PartialEq)]
pub enum PuzzleState {
    Unsolved,
    Solved,
    Failed,
}

/// The terminal puzzle resource.
#[derive(Resource, Debug)]
pub struct TerminalPuzzle {
    pub state: PuzzleState,
    pub current_input: String,
    pub attempts: u32,
    pub max_attempts: u32,
}

impl Default for TerminalPuzzle {
    /// Creates a new unsolved puzzle with 3 attempts.
    fn default() -> Self {
        TerminalPuzzle {
            state: PuzzleState::Unsolved,
            current_input: String::new(),
            attempts: 0,
            max_attempts: 3,
        }
    }
}

/// The pseudocode prompt shown to the player on the terminal.
pub const PUZZLE_PROMPT: &str = r#"
PATCH FILE: y2k_fix.pseudo
-----------------------------------------
FUNCTION get_full_year(year):
    stored_yeer = year % 100
    if stored_year > 99:
        RETRUN stored_year
    RETURN stored_year + 190
-----------------------------------------
FIX ALL ERRORS. TYPE CORRECTED LINE (1/2/3):
"#;

/// Checks the player's input against the correct answer.
pub fn check_answer(input: &str) -> PuzzleState {
    match input.trim().to_uppercase().as_str() {
        "B" => PuzzleState::Solved,
        _   => PuzzleState::Failed,
    }
}