use crate::{
    effect::AudioPlusSoundEffect,
    listener::AudioPlusListener,
    mixer::{AudioPlusMixer, AudioPlusMixerChannel},
    voice::{AudioPlusVoice, AudioPlusVoiceState},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct AudioPlusSource {
    pub(crate) sound_effect: AudioPlusSoundEffect,
    pub(crate) voices: Vec<AudioPlusVoice>,
    pub(crate) next_voice: usize,
}

impl AudioPlusSource {
    pub fn new(sound_effect: AudioPlusSoundEffect) -> Self {
        Self {
            sound_effect,
            voices: vec![],
            next_voice: 0,
        }
    }

    pub fn as_playing(mut self) -> Self {
        self.play();
        self
    }

    pub fn as_looping(mut self) -> Self {
        self.play_looped();
        self
    }

    fn create_voices(&mut self) {
        if self.voices.len() != self.sound_effect.voices {
            self.voices = vec![];
            for _ in 0..self.sound_effect.voices {
                self.voices.push(AudioPlusVoice::new());
            }
        }
    }

    fn prepare_voice(&mut self) -> Option<usize> {
        self.create_voices();
        let should_play = self.sound_effect.chance > rand::random::<f32>();
        if !self.voices.is_empty() && !self.sound_effect.audio_sources.is_empty() && should_play {
            self.next_voice = self.next_voice % self.voices.len();
            let id = self.next_voice;
            self.next_voice = (self.next_voice + 1) % self.voices.len();
            let voice = &mut self.voices[id];
            let audio_source = &self.sound_effect.audio_sources
                [rand::random::<usize>() % self.sound_effect.audio_sources.len()];
            voice.reset();
            voice.audio_source = Some(audio_source.clone());
            voice.state_dirty = true;
            voice.volume = (self.sound_effect.volume - self.sound_effect.volume_variation * 0.5
                + rand::random::<f32>() * self.sound_effect.volume_variation)
                .clamp(0., 1.);
            voice.playback_rate = (self.sound_effect.pitch
                - self.sound_effect.pitch_variation * 0.5
                + rand::random::<f32>() * self.sound_effect.pitch_variation)
                .max(0.);
            Some(id)
        } else {
            None
        }
    }

    pub fn play(&mut self) {
        if let Some(index) = self.prepare_voice() {
            self.voices[index].state = AudioPlusVoiceState::Playing;
        }
    }

    pub fn play_looped(&mut self) {
        if let Some(index) = self.prepare_voice() {
            self.voices[index].state = AudioPlusVoiceState::Looping;
        }
    }

    pub fn stop(&mut self) {
        for voice in self.voices.iter_mut() {
            if voice.state != AudioPlusVoiceState::Stopped {
                if voice.status.playing {
                    voice.stopping = true;
                } else {
                    voice.reset();
                    voice.state_dirty = true;
                }
            }
        }
    }

    pub fn effect(&self) -> &AudioPlusSoundEffect {
        &self.sound_effect
    }

    pub fn effect_mut(&mut self) -> &mut AudioPlusSoundEffect {
        &mut self.sound_effect
    }
}

pub(crate) fn update_audio_sources(
    mut queries: ParamSet<(
        Query<(&mut AudioPlusSource, Option<&GlobalTransform>)>,
        Query<&GlobalTransform, With<AudioPlusListener>>,
    )>,
    time: Res<Time>,
    mixer: Res<AudioPlusMixer>,
) {
    let listener_transform = if let Ok(transform) = queries.p1().get_single() {
        Some(*transform)
    } else {
        None
    };
    for (mut source, transform) in queries.p0().iter_mut() {
        source.create_voices();
        let mut volume = 1.;
        let mut panning = 0.5;
        if source.sound_effect.positional && transform.is_some() && listener_transform.is_some() {
            let relative_position = transform.unwrap().translation().truncate()
                - listener_transform.unwrap().translation().truncate();
            let distance = relative_position.length();
            volume *= ((source.sound_effect.distance - distance) / source.sound_effect.distance)
                .clamp(0., 1.);
            panning =
                (0.5 + (relative_position.x / source.sound_effect.distance) * 1.2).clamp(0.2, 0.8);
        }
        if source.sound_effect.channel != AudioPlusMixerChannel::None {
            volume *= mixer.get_volume(source.sound_effect.channel);
        }
        let AudioPlusSource {
            voices,
            sound_effect,
            ..
        } = source.as_mut();
        for voice in voices.iter_mut() {
            if voice.status.initialized && !voice.status.playing {
                voice.reset();
                voice.state_dirty = true;
            } else {
                if !voice.stopping {
                    if sound_effect.fade_in > 0. {
                        voice.volume_fade += time.delta_seconds() / sound_effect.fade_in;
                    } else {
                        voice.volume_fade = 1.;
                    }
                } else {
                    if sound_effect.fade_out > 0. {
                        voice.volume_fade -= time.delta_seconds() / sound_effect.fade_out;
                    } else {
                        voice.volume_fade = 0.;
                    }
                }
                voice.volume_fade = voice.volume_fade.clamp(0., 1.);
                if voice.volume_fade == 0. {
                    voice.reset()
                } else {
                    voice.should_assign =
                        voice.state != AudioPlusVoiceState::Stopped && volume > 0.;
                    voice.volume_multiplier = volume;
                    voice.panning = panning;
                }
            }
        }
    }
}
