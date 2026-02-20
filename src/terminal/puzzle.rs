//! Terminal puzzle logic.
//! Player fixes 3 typos in pseudocode one line at a time.

use bevy::prelude::*;

/// A single broken line the player must fix.
#[derive(Debug, Clone)]
pub struct PuzzleLine {
    /// The broken line shown to the player.
    pub broken: &'static str,
    /// The correct answer the player must type.
    pub correct: &'static str,
    /// Hint shown after a failed attempt.
    pub hint: &'static str,
}

/// All three buggy lines in order.
pub const PUZZLE_LINES: [PuzzleLine; 3] = [
    PuzzleLine {
        broken:  "    stored_yeer = year % 100",
        correct: "stored_year = year % 100",
        hint:    "HINT: CHECK THE VARIABLE NAME",
    },
    PuzzleLine {
        broken:  "        RETRUN stored_year",
        correct: "RETURN stored_year",
        hint:    "HINT: CHECK THE KEYWORD SPELLING",
    },
    PuzzleLine {
        broken:  "    RETURN stored_year + 190",
        correct: "RETURN stored_year + 1900",
        hint:    "HINT: CHECK THE CONSTANT VALUE",
    },
];

/// All possible puzzle states.
#[derive(Debug, Clone, PartialEq)]
pub enum PuzzleState {
    Unsolved,
    Solved,
    Failed,
}

/// The terminal puzzle resource tracking progress across all lines.
#[derive(Resource, Debug)]
pub struct TerminalPuzzle {
    pub state: PuzzleState,
    pub current_line: usize,
    pub current_input: String,
    pub attempts_on_line: u32,
    pub max_attempts: u32,
    pub show_hint: bool,
}

impl Default for TerminalPuzzle {
    /// Creates a new unsolved puzzle starting at line 1.
    fn default() -> Self {
        TerminalPuzzle {
            state: PuzzleState::Unsolved,
            current_line: 0,
            current_input: String::new(),
            attempts_on_line: 0,
            max_attempts: 3,
            show_hint: false,
        }
    }
}

/// The full pseudocode block shown as context.
pub const PUZZLE_PROMPT: &str = "\
PATCH FILE: y2k_fix.pseudo\n\
-----------------------------------------\n\
FUNCTION get_full_year(year):\n\
    stored_yeer = year % 100\n\
        RETRUN stored_year\n\
    RETURN stored_year + 190\n\
-----------------------------------------\n";

/// Checks the player input against the current line's correct answer.
/// Returns true if correct.
pub fn check_answer(input: &str, line_index: usize) -> bool {
    let normalized_input = input.trim().to_lowercase();
    let normalized_correct = PUZZLE_LINES[line_index].correct.trim().to_lowercase();
    normalized_input == normalized_correct
}