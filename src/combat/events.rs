//! Combat events — damage, death, and attack signals.
//! Phase 1: stubs only. Phase 2: full implementation.

use bevy::prelude::*;

/// Fired when an entity deals damage to another.
#[derive(Event, Debug)]
pub struct DamageEvent {
    /// The entity dealing damage.
    pub source: Entity,
    /// The entity receiving damage.
    pub target: Entity,
    /// Amount of damage dealt.
    pub amount: f32,
}

/// Fired when an entity dies.
#[derive(Event, Debug)]
pub struct DeathEvent {
    /// The entity that died.
    pub entity: Entity,
}

/// Fired when the player initiates an attack.
#[derive(Event, Debug)]
pub struct AttackEvent {
    /// The attacking entity.
    pub attacker: Entity,
}