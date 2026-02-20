//! Maze renderer — builds 3D box meshes from maze cell data.
//! Walls are Cuboid meshes with a solid color material.
//! Wireframe overlay is stubbed for future use.

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::maze::generator::{Maze, MAZE_WIDTH, MAZE_HEIGHT, NORTH, EAST};

/// Wall dimensions in world units.
pub const WALL_HEIGHT: f32 = 2.0;
pub const WALL_THICKNESS: f32 = 0.2;
pub const CELL_SIZE: f32 = 2.0;

/// Marker component for maze wall entities.
#[derive(Component)]
pub struct MazeWall;

/// Marker component for maze floor entity.
#[derive(Component)]
pub struct MazeFloor;

/// Spawns all maze walls and floor as 3D box meshes.
pub fn spawn_maze(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    maze: Res<Maze>,
) {
    let wall_color = Color::srgb(0.0, 0.6, 0.8); // cyan-blue tone
    let floor_color = Color::srgb(0.05, 0.05, 0.05); // near black

    let wall_material = materials.add(StandardMaterial {
        base_color: wall_color,
        emissive: LinearRgba::new(0.0, 0.3, 0.4, 1.0),
        ..default()
    });

    let floor_material = materials.add(StandardMaterial {
        base_color: floor_color,
        ..default()
    });

    // Spawn floor
    commands.spawn((
        MazeFloor,
        Mesh3d(meshes.add(Cuboid::new(
            MAZE_WIDTH as f32 * CELL_SIZE,
            0.1,
            MAZE_HEIGHT as f32 * CELL_SIZE,
        ))),
        MeshMaterial3d(floor_material),
        Transform::from_xyz(
            MAZE_WIDTH as f32 * CELL_SIZE / 2.0,
            -0.05,
            MAZE_HEIGHT as f32 * CELL_SIZE / 2.0,
        ),
        Collider::cuboid(
            MAZE_WIDTH as f32 * CELL_SIZE / 2.0,
            0.05,
            MAZE_HEIGHT as f32 * CELL_SIZE / 2.0,
        ),
        RigidBody::Fixed,
    ));

    // Spawn walls
    for y in 0..MAZE_HEIGHT {
        for x in 0..MAZE_WIDTH {
            let cell = maze.cells[y][x];
            let wx = x as f32 * CELL_SIZE;
            let wz = y as f32 * CELL_SIZE;

            // North wall
            if !cell.is_open(NORTH) {
                commands.spawn((
                    MazeWall,
                    Mesh3d(meshes.add(Cuboid::new(CELL_SIZE, WALL_HEIGHT, WALL_THICKNESS))),
                    MeshMaterial3d(wall_material.clone()),
                    Transform::from_xyz(wx + CELL_SIZE / 2.0, WALL_HEIGHT / 2.0, wz),
                    Collider::cuboid(CELL_SIZE / 2.0, WALL_HEIGHT / 2.0, WALL_THICKNESS / 2.0),
                    RigidBody::Fixed,
                    ));
            }

            // East wall
            if !cell.is_open(EAST) {
                commands.spawn((
                    MazeWall,
                    Mesh3d(meshes.add(Cuboid::new(WALL_THICKNESS, WALL_HEIGHT, CELL_SIZE))),
                    MeshMaterial3d(wall_material.clone()),
                    Transform::from_xyz(wx + CELL_SIZE, WALL_HEIGHT / 2.0, wz + CELL_SIZE / 2.0),
                    Collider::cuboid(WALL_THICKNESS / 2.0, WALL_HEIGHT / 2.0, CELL_SIZE / 2.0),
                    RigidBody::Fixed,
                ));
            }
        }
    }

    // Spawn ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
        ..default()
    });
}