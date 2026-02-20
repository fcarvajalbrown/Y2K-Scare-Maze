//! Enemy AI — pathfinding and aggro.
//! Phase 1: stub only. Phase 2: A* on maze graph.

use bevy::prelude::*;
use super::types::Enemy;

/// Tracks the enemy's current AI state.
#[derive(Component, Debug, Default, PartialEq)]
pub enum EnemyAiState {
    #[default]
    Idle,
    Chasing,
    Attacking,
}

/// Stores the enemy's current path through the maze.
#[derive(Component, Debug, Default)]
pub struct EnemyPath {
    pub waypoints: Vec<Vec3>,
    pub current_waypoint: usize,
}

/// Checks distance to player and sets aggro state.
/// Phase 2: replace with A* pathfinding.
pub fn update_aggro(
    player_query: Query<&Transform, With<crate::player::controller::Player>>,
    mut enemy_query: Query<(&Transform, &Enemy, &mut EnemyAiState)>,
) {
    let Ok(player_transform) = player_query.get_single() else { return; };

    for (enemy_transform, enemy, mut ai_state) in enemy_query.iter_mut() {
        let distance = enemy_transform.translation
            .distance(player_transform.translation);

        *ai_state = if distance <= enemy.aggro_range {
            EnemyAiState::Chasing
        } else {
            EnemyAiState::Idle
        };
    }
}