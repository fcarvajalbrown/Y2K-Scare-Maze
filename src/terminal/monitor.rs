//! Old CRT monitor built from primitive box meshes.
//! No imported models — pure Cuboid geometry.

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::terminal::spawner::Terminal;
use crate::maze::renderer::CELL_SIZE;

/// Spawns a CRT monitor made of cuboids at the terminal position.
pub fn spawn_monitor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let base_x = CELL_SIZE * 18.0;
    let base_z = CELL_SIZE * 18.0;

    let casing_color = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.15, 0.12), // dirty beige/grey
        ..default()
    });

    let screen_color = materials.add(StandardMaterial {
        base_color: Color::srgb(0.0, 0.05, 0.0),
        emissive: LinearRgba::new(0.0, 1.5, 0.4, 1.0), // green glow
        ..default()
    });

    let bezel_color = materials.add(StandardMaterial {
        base_color: Color::srgb(0.1, 0.1, 0.08),
        ..default()
    });

    // Desk/base unit (the big box computer body)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.8, 0.15, 0.7))),
        MeshMaterial3d(casing_color.clone()),
        Transform::from_xyz(base_x, 0.75, base_z),
        RigidBody::Fixed,
        Collider::cuboid(0.4, 0.075, 0.35),
    ));

    // Monitor back casing (thick box behind screen)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.7, 0.55, 0.45))),
        MeshMaterial3d(casing_color.clone()),
        Transform::from_xyz(base_x, 1.45, base_z + 0.1),
        RigidBody::Fixed,
        Collider::cuboid(0.35, 0.275, 0.225),
    ));

    // Monitor bezel (front face, slightly proud of casing)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.65, 0.5, 0.05))),
        MeshMaterial3d(bezel_color),
        Transform::from_xyz(base_x, 1.45, base_z - 0.125),
    ));

    // Screen (recessed slightly into bezel)
    commands.spawn((
        Terminal,
        Mesh3d(meshes.add(Cuboid::new(0.52, 0.38, 0.02))),
        MeshMaterial3d(screen_color),
        Transform::from_xyz(base_x, 1.47, base_z - 0.14),
    ));

    // Keyboard (flat box in front)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.6, 0.04, 0.22))),
        MeshMaterial3d(casing_color.clone()),
        Transform::from_xyz(base_x, 0.84, base_z - 0.55),
        RigidBody::Fixed,
        Collider::cuboid(0.3, 0.02, 0.11),
    ));

    // Small indicator light on bezel
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.03, 0.03, 0.02))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 1.0, 0.0),
            emissive: LinearRgba::new(0.0, 3.0, 0.0, 1.0),
            ..default()
        })),
        Transform::from_xyz(base_x + 0.28, 1.22, base_z - 0.15),
    ));
}