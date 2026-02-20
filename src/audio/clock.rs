//! Procedurally generated ticking clock ambient sound.
//! Synthesizes a mechanical tick using raw PCM samples via rodio.

use bevy::prelude::*;
use rodio::{OutputStream, Sink, Source};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Resource holding the audio sink so it stays alive.
//#[derive(Resource)]
//pub struct ClockAudio {
//    pub sink: Sink,
//}

/// Generates a single mechanical tick as raw PCM samples.
fn generate_tick() -> Vec<i16> {
    let sample_rate = 44100u32;
    let tick_duration_ms = 12;
    let samples = (sample_rate as usize * tick_duration_ms) / 1000;
    let mut buffer = Vec::with_capacity(samples);

    for i in 0..samples {
        let t = i as f32 / sample_rate as f32;
        // Sharp transient + fast decay
        let envelope = (-t * 300.0).exp();
        // Low thud frequency
        let wave = (t * 180.0 * std::f32::consts::TAU).sin();
        let sample = (wave * envelope * i16::MAX as f32) as i16;
        buffer.push(sample);
    }
    buffer
}

/// Synthesizes a full tick-tock cycle as a rodio Source.
#[derive(Clone)]
pub struct TickSource {
    samples: Arc<Vec<i16>>,
    index: usize,
    sample_rate: u32,
}

impl TickSource {
    pub fn new() -> Self {
        TickSource {
            samples: Arc::new(generate_tick()),
            index: 0,
            sample_rate: 44100,
        }
    }
}

impl Iterator for TickSource {
    type Item = i16;
    fn next(&mut self) -> Option<i16> {
        if self.index < self.samples.len() {
            let s = self.samples[self.index];
            self.index += 1;
            Some(s)
        } else {
            None
        }
    }
}

impl Source for TickSource {
    fn current_frame_len(&self) -> Option<usize> { None }
    fn channels(&self) -> u16 { 1 }
    fn sample_rate(&self) -> u32 { self.sample_rate }
    fn total_duration(&self) -> Option<Duration> { None }
}

/// Startup system — initializes the clock audio and starts ticking.
use std::cell::RefCell;

thread_local! {
    static AUDIO_STREAM: RefCell<Option<OutputStream>> = RefCell::new(None);
}

/// Resource holding only the Sink — OutputStream lives in thread_local.
#[derive(Resource)]
pub struct ClockAudio {
    pub sink: Arc<Mutex<Sink>>,
}

/// Startup system — initializes the clock audio and starts ticking.
pub fn start_clock_audio(mut commands: Commands) {
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    loop_tick(&sink);

    AUDIO_STREAM.with(|s| {
        *s.borrow_mut() = Some(stream);
    });

    commands.insert_resource(ClockAudio { sink: Arc::new(Mutex::new(sink))
    });
}

/// Queues a repeating tick pattern into the sink.
fn loop_tick(sink: &Sink) {
    let sample_rate = 44100u32;
    let gap_samples = sample_rate as usize; // 1 second gap between ticks

    for _ in 0..120 { // pre-queue 2 minutes of ticks
        sink.append(TickSource::new());
        // Silence between ticks
        sink.append(rodio::source::Zero::<i16>::new(1, sample_rate).take_duration(
            Duration::from_millis(900),
        ));
    }
}