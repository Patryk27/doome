use std::io::Cursor;

use bevy::prelude::*;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

use crate::assets::{DoomeAssetHandle, DoomeAssets};

pub struct DoomeSound {
    content: Vec<u8>,
}

impl DoomeSound {
    pub fn new(content: Vec<u8>) -> Self {
        Self { content }
    }
}

#[derive(Resource)]
pub struct DoomeAudio {
    queue: Vec<DoomeAssetHandle<DoomeSound>>,
}

impl DoomeAudio {
    pub fn play(&mut self, sound: DoomeAssetHandle<DoomeSound>) {
        self.queue.push(sound);
    }
}

pub struct AudioPlugin;

pub struct AudioOutput {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sinks: Vec<Sink>,
}

#[derive(Component)]
pub struct AudioPlayer {
    asset: DoomeAssetHandle<DoomeSound>,
    sink: Sink,
}

impl AudioPlayer {
    pub fn new(asset: DoomeAssetHandle<DoomeSound>, outout: &AudioOutput) -> Self {
        let sink = Sink::try_new(&outout.stream_handle).unwrap();

        Self { asset, sink }
    }

    pub fn play(&self) {
        self.sink.play();
    }

    pub fn pause(&self) {
        self.sink.pause();
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    pub fn set_speed(&self, speed: f32) {
        self.sink.set_speed(speed);
    }
}

const NUM_SINKS: usize = 32;

impl FromWorld for AudioOutput {
    fn from_world(_world: &mut World) -> Self {
        let (stream, stream_handle) = OutputStream::try_default()
            .expect("Failed to initialize audio state");

        let sinks = (0..NUM_SINKS)
            .map(|_| Sink::try_new(&stream_handle).unwrap())
            .collect();

        Self {
            _stream: stream,
            stream_handle,
            sinks,
        }
    }
}

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_non_send_resource::<AudioOutput>();
        app.insert_resource(DoomeAudio { queue: Vec::new() });

        app.add_system_to_stage(CoreStage::PostUpdate, play_queued_audio);
        app.add_system_to_stage(CoreStage::PostUpdate, play_audio_players);
    }
}

fn play_audio_players(assets: Res<DoomeAssets>, audio_players: Query<&AudioPlayer>) {
    for player in audio_players.iter() {
        if player.sink.len() < 2 {
            let sound = assets.sound(player.asset);

            let source = Decoder::new(Cursor::new(sound.content.clone()))
                .expect("Failed to decode audio");

            player.sink.append(source);
        }
    }
}

fn play_queued_audio(
    assets: Res<DoomeAssets>,
    mut audio: ResMut<DoomeAudio>,
    state: NonSend<AudioOutput>,
) {
    for handle in audio.queue.drain(..) {
        let sound = assets.sound(handle);

        let source = Decoder::new(Cursor::new(sound.content.clone()))
            .expect("Failed to decode audio");

        for sink in state.sinks.iter() {
            if sink.empty() {
                sink.append(source);
                break;
            }
        }
    }
}
