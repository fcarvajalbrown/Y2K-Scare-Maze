//! Enemy type definitions and components.
//! Phase 1: stubs only. Phase 2: full implementation.

use bevy::prelude::*;

/// All possible enemy variants.
#[derive(Component, Debug, Clone, PartialEq)]
pub enum EnemyType {
    /// Slow, high damage melee enemy.
    GlitchBot,
    /// Ranged enemy that fires error packets.
    CorruptProcess,
    /// Fast and fragile, rushes the player.
    VirusSprite,
}

/// Core enemy component storing health and type.
#[derive(Component, Debug)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub health: f32,
    pub max_health: f32,
    pub damage: f32,
    pub speed: f32,
    pub aggro_range: f32,
}

impl Enemy {
    /// Creates a new enemy with stats based on its type.
    pub fn new(enemy_type: EnemyType) -> Self {
        match enemy_type {
            EnemyType::GlitchBot => Enemy {
                enemy_type: EnemyType::GlitchBot,
                health: 80.0,
                max_health: 80.0,
                damage: 20.0,
                speed: 2.0,
                aggro_range: 5.0,
            },
            EnemyType::CorruptProcess => Enemy {
                enemy_type: EnemyType::CorruptProcess,
                health: 50.0,
                max_health: 50.0,
                damage: 15.0,
                speed: 1.5,
                aggro_range: 8.0,
            },
            EnemyType::VirusSprite => Enemy {
                enemy_type: EnemyType::VirusSprite,
                health: 30.0,
                max_health: 30.0,
                damage: 10.0,
                speed: 5.0,
                aggro_range: 6.0,
            },
        }
    }
}