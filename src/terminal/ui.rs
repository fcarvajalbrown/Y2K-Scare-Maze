//! Terminal UI overlay using bevy_egui.
//! Renders a fullscreen black panel with green typewriter text.
//! CRT shader support is stubbed via TerminalStyle for future use.

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::terminal::puzzle::{TerminalPuzzle, PuzzleState, PUZZLE_PROMPT, check_answer};

/// Controls terminal visual style.
/// crt_enabled is a stub for future CRT shader integration.
#[derive(Resource)]
pub struct TerminalStyle {
    pub crt_enabled: bool,
    pub text_color: egui::Color32,
    pub background_color: egui::Color32,
}

impl Default for TerminalStyle {
    fn default() -> Self {
        TerminalStyle {
            crt_enabled: false,
            text_color: egui::Color32::from_rgb(0, 255, 70),
            background_color: egui::Color32::BLACK,
        }
    }
}

/// Tracks typewriter reveal progress.
#[derive(Resource, Default)]
pub struct TypewriterState {
    pub chars_revealed: usize,
    pub timer: f32,
    pub chars_per_second: f32,
    pub phase: TypewriterPhase,
}

/// Which text block the typewriter is currently revealing.
#[derive(Default, Debug, PartialEq)]
pub enum TypewriterPhase {
    #[default]
    Warning,
    Puzzle,
    Done,
}

impl TypewriterState {
    /// Resets typewriter to beginning of warning phase.
    pub fn reset(&mut self) {
        self.chars_revealed = 0;
        self.timer = 0.0;
        self.chars_per_second = 30.0;
        self.phase = TypewriterPhase::Warning;
    }
}

/// Warning text shown before the puzzle.
pub const WARNING_TEXT: &str = "\
> INITIALIZING Y2K PATCH CONSOLE...\n\
> WARNING: CRITICAL SYSTEM FAULT DETECTED\n\
> DATE OVERFLOW IMMINENT: 00/00/00\n\
> LOCATE AND FIX ALL SYNTAX ERRORS IN THE PATCH CODE\n\
> THE MACHINE CANNOT DO IT. YOU MUST.\n\
> [PRESS ENTER TO BEGIN]\n";

/// Advances the typewriter timer and reveals characters over time.
pub fn tick_typewriter(
    time: Res<Time>,
    mut tw: ResMut<TypewriterState>,
    puzzle: Res<TerminalPuzzle>,
) {
    if tw.phase == TypewriterPhase::Done { return; }
    if puzzle.state != PuzzleState::Unsolved { return; }

    tw.timer += time.delta_secs();
    let chars_to_show = (tw.timer * tw.chars_per_second) as usize;

    let current_text = match tw.phase {
        TypewriterPhase::Warning => WARNING_TEXT,
        TypewriterPhase::Puzzle  => PUZZLE_PROMPT,
        TypewriterPhase::Done    => return,
    };

    tw.chars_revealed = chars_to_show.min(current_text.len());

    if tw.chars_revealed >= current_text.len() {
        if tw.phase == TypewriterPhase::Warning {
            // Wait for Enter before moving to puzzle phase
        } else {
            tw.phase = TypewriterPhase::Done;
        }
    }
}

/// Renders the fullscreen terminal overlay.
pub fn render_terminal(
    mut contexts: EguiContexts,
    mut puzzle: ResMut<TerminalPuzzle>,
    style: Res<TerminalStyle>,
    mut tw: ResMut<TypewriterState>,
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default()
        .frame(egui::Frame::none().fill(style.background_color))
        .show(ctx, |ui| {
            ui.visuals_mut().override_text_color = Some(style.text_color);

            match tw.phase {
                TypewriterPhase::Warning => {
                    let visible = &WARNING_TEXT[..tw.chars_revealed];
                    ui.label(egui::RichText::new(visible).monospace().size(16.0));

                    if tw.chars_revealed >= WARNING_TEXT.len()
                        && keys.just_pressed(KeyCode::Enter)
                    {
                        tw.chars_revealed = 0;
                        tw.timer = 0.0;
                        tw.phase = TypewriterPhase::Puzzle;
                    }
                }

                TypewriterPhase::Puzzle | TypewriterPhase::Done => {
                    let visible_len = tw.chars_revealed.min(PUZZLE_PROMPT.len());
                    let visible = &PUZZLE_PROMPT[..visible_len];
                    ui.label(egui::RichText::new(visible).monospace().size(16.0));

                    if tw.phase == TypewriterPhase::Done {
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("> ").monospace().size(16.0));
                            ui.text_edit_singleline(&mut puzzle.current_input);
                        });

                        if keys.just_pressed(KeyCode::Enter) {
                            let result = check_answer(&puzzle.current_input);
                            puzzle.attempts += 1;
                            match result {
                                PuzzleState::Solved => {
                                    puzzle.state = PuzzleState::Solved;
                                    next_state.set(crate::states::GameState::Win);
                                }
                                _ => {
                                    if puzzle.attempts >= puzzle.max_attempts {
                                        puzzle.state = PuzzleState::Failed;
                                    } else {
                                        puzzle.current_input.clear();
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Escape exits terminal
            if keys.just_pressed(KeyCode::Escape) {
                next_state.set(crate::states::GameState::Exploring);
            }
        });
}