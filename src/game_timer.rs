//! Game timer — 60 second countdown.
//! Hitting zero triggers GameOver.
//! Phase 2: enemies subtract time on hit.

use bevy::prelude::*;
use crate::states::GameState;

/// The main countdown timer resource.
#[derive(Resource, Debug)]
pub struct GameTimer {
    pub remaining: f32,
    pub total: f32,
}

impl Default for GameTimer {
    fn default() -> Self {
        GameTimer {
            remaining: 60.0,
            total: 60.0,
        }
    }
}

/// Ticks the timer down and triggers GameOver at zero.
pub fn tick_timer(
    time: Res<Time>,
    mut timer: ResMut<GameTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    timer.remaining -= time.delta_secs();
    if timer.remaining <= 0.0 {
        timer.remaining = 0.0;
        next_state.set(GameState::GameOver);
    }
}