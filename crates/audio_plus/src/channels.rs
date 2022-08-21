use crate::{
    source::AudioPlusSource,
    voice::{AudioPlusVoiceHandle, AudioPlusVoiceState},
    AudioPlusSystems,
};
use bevy::ecs::system::Resource;
use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioChannel, AudioControl, AudioInstance};

macro_rules! channels {
    ( $( $x:ident ),* ) => {
        $(
            pub struct $x;
        )*
        pub fn add_audio_channels(app: &mut App) {
            $(
                app.add_audio_channel::<$x>();
                app.add_system(update_kira_channel::<$x>.after(AudioPlusSystems::UpdateAudioSources));
            )*
        }
    };
}

fn f32_sufficient_difference(to: f32, from: f32) -> bool {
    return (from - to).abs() > 0.02 || (to == 0. && from != 0.);
}

#[derive(Default)]
struct ChannelData {
    initialized: bool,
    voice_handle: Option<AudioPlusVoiceHandle>,
    instance_handle: Option<Handle<AudioInstance>>,
    last_volume: f32,
    last_panning: f32,
    last_playback_rate: f32,
}

fn update_kira_channel<T: Resource>(
    mut data: Local<ChannelData>,
    channel: Res<AudioChannel<T>>,
    mut query: Query<(Entity, &mut AudioPlusSource)>,
) {
    if !data.initialized {
        channel.set_volume(0.);
        data.initialized = true;
    }
    if let Some(voice_handle) = data.voice_handle {
        let mut unassign = true;
        if let Ok((_, mut source)) = query.get_mut(voice_handle.entity) {
            if let Some(voice) = source.voices.get_mut(voice_handle.index) {
                if voice.should_assign {
                    unassign = false;
                    if voice.state_dirty {
                        match voice.state {
                            AudioPlusVoiceState::Stopped => {
                                data.instance_handle = None;
                                channel.stop();
                            }
                            AudioPlusVoiceState::Playing => {
                                data.instance_handle = None;
                                channel.stop();
                                if let Some(audio_source) = &voice.audio_source {
                                    data.instance_handle =
                                        Some(channel.play(audio_source.clone()).handle());
                                }
                            }
                            AudioPlusVoiceState::Looping => {
                                data.instance_handle = None;
                                channel.stop();
                                if let Some(audio_source) = &voice.audio_source {
                                    data.instance_handle =
                                        Some(channel.play(audio_source.clone()).looped().handle());
                                }
                            }
                        }
                        voice.state_dirty = false;
                    }
                    let new_volume = voice.volume * voice.volume_multiplier * voice.volume_fade;
                    if f32_sufficient_difference(new_volume, data.last_volume) {
                        channel.set_volume(
                            (voice.volume * voice.volume_multiplier * voice.volume_fade) as f64,
                        );
                        data.last_volume = new_volume;
                    }
                    if f32_sufficient_difference(voice.panning, data.last_panning) {
                        channel.set_panning(voice.panning as f64);
                        data.last_panning = voice.panning;
                    }
                    if f32_sufficient_difference(voice.playback_rate, data.last_playback_rate) {
                        channel.set_playback_rate(voice.playback_rate as f64);
                        data.last_playback_rate = voice.playback_rate;
                    }
                    if let Some(instance_handle) = &data.instance_handle {
                        let has_position = channel.state(&instance_handle).position().is_some();
                        if voice.status.initialized {
                            voice.status.playing = has_position;
                        } else {
                            voice.status.initialized = has_position;
                            voice.status.playing = true;
                        }
                    }
                }
            }
        }
        if unassign {
            channel.stop();
            if f32_sufficient_difference(0., data.last_volume) {
                channel.set_volume(0.);
                data.last_volume = 0.;
            }
            data.voice_handle = None;
            data.instance_handle = None;
        }
    } else {
        let mut found = false;
        for (entity, mut source) in query.iter_mut() {
            for (index, voice) in source.voices.iter_mut().enumerate() {
                if voice.should_assign && !voice.assigned {
                    data.voice_handle = Some(AudioPlusVoiceHandle { entity, index });
                    voice.assigned = true;
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
    }
}

channels!(
    Channel1, Channel2, Channel3, Channel4, Channel5, Channel6, Channel7, Channel8, Channel9,
    Channel10
);
