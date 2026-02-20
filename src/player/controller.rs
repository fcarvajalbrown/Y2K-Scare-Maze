//! First-person player controller.
//! Handles WASD movement, mouse look, and cursor locking.

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

/// Marker component for the player entity.
#[derive(Component)]
pub struct Player;

/// Stores player look angles to avoid gimbal lock.
#[derive(Component)]
pub struct PlayerLook {
    pub yaw: f32,
    pub pitch: f32,
}

/// Mouse sensitivity setting.
#[derive(Resource)]
pub struct MouseSensitivity(pub f32);

impl Default for MouseSensitivity {
    fn default() -> Self {
        MouseSensitivity(0.002)
    }
}

/// Spawns the player camera at maze start position.
pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        PlayerLook { yaw: 0.0, pitch: 0.0 },
        Camera3d::default(),
        Transform::from_xyz(1.0, 0.5, 1.0),
    ));
}

/// Locks the cursor on startup.
pub fn lock_cursor(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
    window.cursor_options.visible = false;
}

/// Handles mouse motion to rotate the camera.
pub fn player_look(
    mut motion: EventReader<MouseMotion>,
    sensitivity: Res<MouseSensitivity>,
    mut query: Query<(&mut Transform, &mut PlayerLook), With<Player>>,
) {
    let (mut transform, mut look) = query.single_mut();
    for ev in motion.read() {
        look.yaw   -= ev.delta.x * sensitivity.0;
        look.pitch -= ev.delta.y * sensitivity.0;
        look.pitch  = look.pitch.clamp(-1.4, 1.4);
    }
    transform.rotation = Quat::from_euler(EulerRot::YXZ, look.yaw, look.pitch, 0.0);
}

/// Handles WASD movement relative to camera facing direction.
pub fn player_move(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &PlayerLook), With<Player>>,
) {
    let (mut transform, look) = query.single_mut();
    let speed = 4.0;
    let forward = Vec3::new(look.yaw.sin(), 0.0, look.yaw.cos());
    let right   = Vec3::new(look.yaw.cos(), 0.0, -look.yaw.sin());
    let mut velocity = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) { velocity -= forward; }
    if keys.pressed(KeyCode::KeyS) { velocity += forward; }
    if keys.pressed(KeyCode::KeyA) { velocity -= right; }
    if keys.pressed(KeyCode::KeyD) { velocity += right; }

    if velocity != Vec3::ZERO {
        velocity = velocity.normalize() * speed * time.delta_secs();
        transform.translation += velocity;
    }
}