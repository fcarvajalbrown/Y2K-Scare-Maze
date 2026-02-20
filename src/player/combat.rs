//! Player combat — attack and hit detection.
//! Phase 1: stub only. Phase 2: wire up attacks and hit detection.

use bevy::prelude::*;

/// Marker component for a player melee attack hitbox.
#[derive(Component)]
pub struct PlayerAttack;

/// Tracks player combat state.
#[derive(Component, Debug, Default)]
pub struct PlayerCombatState {
    pub is_attacking: bool,
    pub attack_cooldown: f32,
}

/// Ticks down attack cooldown each frame.
pub fn tick_attack_cooldown(
    time: Res<Time>,
    mut query: Query<&mut PlayerCombatState>,
) {
    for mut state in query.iter_mut() {
        if state.attack_cooldown > 0.0 {
            state.attack_cooldown -= time.delta_secs();
        }
        if state.attack_cooldown <= 0.0 {
            state.is_attacking = false;
        }
    }
}