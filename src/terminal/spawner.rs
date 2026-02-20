//! Terminal spawner — places a glowing green box in the maze.
//! Player presses E within range to trigger AtTerminal state.

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::maze::renderer::CELL_SIZE;

/// Marker component for terminal entities.
#[derive(Component)]
pub struct Terminal;

/// How close the player must be to interact.
pub const INTERACT_RANGE: f32 = 2.0;

/// Spawns the terminal at a fixed position for now.
/// Future: randomize position across valid maze cells.
pub fn spawn_terminal(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Terminal,
        Transform::from_xyz(CELL_SIZE * 19.5, 0.5, CELL_SIZE * 19.5),
        Collider::cuboid(0.3, 0.5, 0.2),
        RigidBody::Fixed,
    ));
}

/// Checks if player is within range of a terminal and presses E.
pub fn interact_terminal(
    keys: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<crate::player::controller::Player>>,
    terminal_query: Query<&Transform, With<Terminal>>,
    mut next_state: ResMut<NextState<crate::states::GameState>>,
    mut tw: ResMut<crate::terminal::ui::TypewriterState>,
) {
    let Ok(player_transform) = player_query.get_single() else { return; };

    for terminal_transform in terminal_query.iter() {
        let distance = player_transform.translation
            .distance(terminal_transform.translation);

        if distance <= INTERACT_RANGE && keys.just_pressed(KeyCode::KeyE) {
            tw.reset();
            next_state.set(crate::states::GameState::AtTerminal);
        }
    }
}