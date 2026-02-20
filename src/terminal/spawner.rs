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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Terminal,
        Mesh3d(meshes.add(Cuboid::new(0.6, 1.0, 0.4))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.8, 0.2),
            emissive: LinearRgba::new(0.0, 2.0, 0.5, 1.0),
            ..default()
        })),
        Transform::from_xyz(CELL_SIZE * 18.0, 0.5, CELL_SIZE * 18.0),
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
        let distance = player_transform.translat