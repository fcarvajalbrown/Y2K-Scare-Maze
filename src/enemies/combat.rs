//! Enemy combat — attack patterns and damage.
//! Phase 1: stub only. Phase 2: full implementation.

use bevy::prelude::*;
use super::types::Enemy;
use super::ai::EnemyAiState;

/// Tracks enemy attack cooldown.
#[derive(Component, Debug, Default)]
pub struct EnemyCombatState {
    pub attack_cooldown: f32,
}

/// Ticks down enemy attack cooldown each frame.
pub fn tick_enemy_cooldown(
    time: Res<Time>,
    mut query: Query<&mut EnemyCombatState>,
) {
    for mut state in query.iter_mut() {
        if state.attack_cooldown > 0.0 {
            state.attack_cooldown -= time.delta_secs();
        }
    }
}

/// Triggers an attack if the enemy is in range and cooldown is ready.
/// Phase 2: fire DamageEvent here.
pub fn enemy_attack(
    mut enemy_query: Query<(&Enemy, &EnemyAiState, &mut EnemyCombatState)>,
) {
    for (enemy, ai_state, mut combat_state) in enemy_query.iter_mut() {
        if *ai_state == EnemyAiState::Attacking && combat_state.attack_cooldown <= 0.0 {
            // Phase 2: send DamageEvent to player
            info!("Enemy {:?} attacks for {} damage!", enemy.enemy_type, enemy.damage);
            combat_state.attack_cooldown = 1.5;
        }
    }
}