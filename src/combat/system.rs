//! Combat system — hit resolution and damage application.
//! Phase 1: stubs only. Phase 2: full implementation.

use bevy::prelude::*;
use super::events::{AttackEvent, DamageEvent, DeathEvent};
use crate::player::stats::PlayerStats;
use crate::enemies::types::Enemy;

/// Resolves incoming damage events and applies them to the target.
/// Phase 2: add hit reactions, sound, and visual feedback.
pub fn resolve_damage(
    mut damage_events: EventReader<DamageEvent>,
    mut death_events: EventWriter<DeathEvent>,
    mut player_query: Query<&mut PlayerStats>,
    mut enemy_query: Query<&mut Enemy>,
) {
    for event in damage_events.read() {
        // Try applying damage to player
        if let Ok(mut stats) = player_query.get_mut(event.target) {
            stats.health -= event.amount;
            info!("Player took {} damage, health: {}", event.amount, stats.health);
            if stats.health <= 0.0 {
                death_events.send(DeathEvent { entity: event.target });
            }
        }

        // Try applying damage to enemy
        if let Ok(mut enemy) = enemy_query.get_mut(event.target) {
            enemy.health -= event.amount;
            info!("Enemy took {} damage, health: {}", event.amount, enemy.health);
            if enemy.health <= 0.0 {
                death_events.send(DeathEvent { entity: event.target });
            }
        }
    }
}

/// Handles death events — despawns enemies, triggers game over for player.
/// Phase 2: add death animations, loot drops.
pub fn handle_death(
    mut commands: Commands,
    mut death_events: EventReader<DeathEvent>,
    player_query: Query<Entity, With<crate::player::controller::Player>>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
) {
    for event in death_events.read() {
        if let Ok(_) = player_query.get(event.entity) {
            next_state.set(crate::states::GameState::GameOver);
        } else {
            commands.entity(event.entity).despawn_recursive();
        }
    }
}

/// Listens for attack events and converts them into damage events.
/// Phase 2: add raycasting for hit detection.
pub fn resolve_attack(
    mut attack_events: EventReader<AttackEvent>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for _event in attack_events.read() {
        // Phase 2: raycast from player, find hit enemy, send DamageEvent
        info!("Attack event received — phase 2 hit detection pending.");
    }
}