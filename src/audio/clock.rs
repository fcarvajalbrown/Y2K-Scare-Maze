//! Clock audio — loads and loops an OGG tick sound.
//! Stops on Win or GameOver state.

use bevy::prelude::*;

/// Marker component for the clock audio entity.
#[derive(Component)]
pub struct ClockAudio;

/// Startup system — loads and loops the clock sound.
pub fn start_clock_audio(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        ClockAudio,
        AudioPlayer::new(asset_server.load("sounds/background_timer.ogg")),
        PlaybackSettings::LOOP,
    ));
}

/// Stops clock audio when win or game over state is reached.
pub fn stop_clock_audio(
    mut commands: Commands,
    query: Query<Entity, With<ClockAudio>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}