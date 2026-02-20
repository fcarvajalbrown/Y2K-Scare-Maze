//! Player stats — health and stamina.

use bevy::prelude::*;

/// Player health and stamina values.
#[derive(Component, Debug)]
pub struct PlayerStats {
    pub health: f32,
    pub max_health: f32,
    pub stamina: f32,
    pub max_stamina: f32,
}

impl Default for PlayerStats {
    /// Returns a player with full health and stamina.
    fn default() -> Self {
        PlayerStats {
            health: 100.0,
            max_health: 100.0,
            stamina: 100.0,
            max_stamina: 100.0,
        }
    }
}

/// Regenerates stamina over time when not sprinting.
pub fn regenerate_stamina(
    time: Res<Time>,
    mut query: Query<&mut PlayerStats>,
) {
    for mut stats in query.iter_mut() {
        if stats.stamina < stats.max_stamina {
            stats.stamina = (stats.stamina + 10.0 * time.delta_secs())
                .min(stats.max_stamina);
        }
    }
}