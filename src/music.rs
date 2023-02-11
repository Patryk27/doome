use bevy::audio::AudioSink;
use bevy::prelude::*;
use doome_bevy::prelude::*;

const LERP_SPEED: f32 = 1.0;
const MUSIC_VOLUME: f32 = 0.1;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MusicTrack {
    Doome,
    Chillout,
}

pub struct SwitchTrack(pub MusicTrack);

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<SwitchTrack>();

        app.add_startup_system(setup_music);
        app.add_system(update_music_state);
        app.add_system(handle_switch_track);
        app.add_system(update_music);
    }
}

#[derive(Resource)]
struct MusicState {
    now_playing: MusicTrack,
    current_music_volume: f32,
    other_music_volume: f32,
}

fn handle_switch_track(
    mut state: ResMut<MusicState>,
    mut events: EventReader<SwitchTrack>,
) {
    let last = events.iter().last();

    if let Some(event) = last {
        if state.now_playing != event.0 {
            let tmp = state.current_music_volume;
            state.current_music_volume = state.other_music_volume;
            state.other_music_volume = tmp;

            state.now_playing = event.0;
        }
    }
}

fn update_music_state(time: Res<Time>, mut music_state: ResMut<MusicState>) {
    let delta = time.delta_seconds();

    music_state.current_music_volume =
        lerp(music_state.current_music_volume, 1.0, delta * LERP_SPEED)
            .clamp(0.0, 1.0);

    music_state.other_music_volume =
        lerp(music_state.other_music_volume, 0.0, delta * LERP_SPEED)
            .clamp(0.0, 1.0);
}

fn lerp(v: f32, t: f32, f: f32) -> f32 {
    v + (t - v) * f
}

fn update_music(
    music_state: Res<MusicState>,
    players: Query<(&MusicTrack, &Handle<AudioSink>)>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    for (track, player) in players.iter() {
        if let Some(player) = audio_sinks.get(player) {
            if *track == music_state.now_playing {
                player.set_volume(
                    music_state.current_music_volume * MUSIC_VOLUME,
                );
            } else {
                player
                    .set_volume(music_state.other_music_volume * MUSIC_VOLUME);
            }
        }
    }
}

fn setup_music(
    mut commands: Commands,
    assets: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let doome_music = assets.load("audio/audiorezout_time_hurries.ogg");
    let doome_music = audio.play_with_settings(
        doome_music,
        PlaybackSettings::LOOP.with_volume(MUSIC_VOLUME),
    );
    let doome_music = audio_sinks.get_handle(doome_music);

    let chillout_music = assets.load(
        "audio/monplaisir_internet_the_day_when_all_humans_will_disappear.ogg",
    );
    let chillout_music = audio.play_with_settings(
        chillout_music,
        PlaybackSettings::LOOP.with_volume(MUSIC_VOLUME),
    );
    let chillout_music = audio_sinks.get_handle(chillout_music);

    commands.spawn((MusicTrack::Doome, doome_music));
    commands.spawn((MusicTrack::Chillout, chillout_music));

    commands.insert_resource(MusicState {
        now_playing: MusicTrack::Doome,
        current_music_volume: 1.0,
        other_music_volume: 0.0,
    });
}
