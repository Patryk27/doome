use std::io::Cursor;

use bevy::prelude::*;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

use crate::assets::{AssetHandle, Assets};

pub struct Sound {
    content: Vec<u8>,
}

impl Sound {
    pub fn new(content: Vec<u8>) -> Self {
        Self { content }
    }
}

#[derive(Resource)]
pub struct Audio {
    queue: Vec<AssetHandle<Sound>>,
}

impl Audio {
    pub fn play(&mut self, sound: AssetHandle<Sound>) {
        self.queue.push(sound);
    }
}

pub struct AudioPlugin;

struct AudioOutput {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Sink,
}

impl FromWorld for AudioOutput {
    fn from_world(_world: &mut World) -> Self {
        let (stream, stream_handle) = OutputStream::try_default()
            .expect("Failed to initialize audio state");

        let sink = Sink::try_new(&stream_handle).unwrap();

        Self {
            stream,
            stream_handle,
            sink,
        }
    }
}

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_non_send_resource::<AudioOutput>();
        app.insert_resource(Audio { queue: Vec::new() });

        app.add_system_to_stage(CoreStage::PostUpdate, play_audio);
    }
}

fn play_audio(
    assets: Res<Assets>,
    mut audio: ResMut<Audio>,
    state: NonSend<AudioOutput>,
) {
    for handle in audio.queue.drain(..) {
        let sound = assets.sound(handle);

        let source = Decoder::new(Cursor::new(sound.content.clone()))
            .expect("Failed to decode audio");

        state.sink.append(source);
    }
}
