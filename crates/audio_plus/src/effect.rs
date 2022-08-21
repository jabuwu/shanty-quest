use bevy::prelude::*;

use crate::mixer::AudioPlusMixerChannel;

#[derive(Clone)]
pub struct AudioPlusSoundEffect {
    pub(crate) audio_sources: Vec<Handle<bevy_kira_audio::AudioSource>>,
    pub(crate) voices: usize,
    pub(crate) positional: bool,
    pub(crate) volume: f32,
    pub(crate) volume_variation: f32,
    pub(crate) pitch: f32,
    pub(crate) pitch_variation: f32,
    pub(crate) distance: f32,
    pub(crate) chance: f32,
    pub(crate) fade_in: f32,
    pub(crate) fade_out: f32,
    pub(crate) channel: AudioPlusMixerChannel,
}

impl Default for AudioPlusSoundEffect {
    fn default() -> Self {
        Self {
            audio_sources: vec![],
            voices: 1,
            positional: false,
            volume: 1.,
            volume_variation: 0.,
            pitch: 1.,
            pitch_variation: 0.,
            distance: 1000.,
            chance: 1.,
            fade_in: 0.,
            fade_out: 0.,
            channel: AudioPlusMixerChannel::None,
        }
    }
}

impl AudioPlusSoundEffect {
    pub fn single(audio_source: Handle<bevy_kira_audio::AudioSource>) -> Self {
        Self::multiple(vec![audio_source])
    }

    pub fn multiple(audio_sources: Vec<Handle<bevy_kira_audio::AudioSource>>) -> Self {
        Self {
            audio_sources,
            ..Default::default()
        }
    }

    pub fn with_voices(self, voices: usize) -> Self {
        Self { voices, ..self }
    }

    pub fn set_voices(&mut self, voices: usize) {
        self.voices = voices;
    }

    pub fn voices(&self) -> usize {
        self.voices
    }

    pub fn with_positional(self, positional: bool) -> Self {
        Self { positional, ..self }
    }

    pub fn set_positional(&mut self, positional: bool) {
        self.positional = positional;
    }

    pub fn positional(&self) -> bool {
        self.positional
    }

    pub fn with_volume(self, volume: f32, volume_variation: f32) -> Self {
        Self {
            volume,
            volume_variation,
            ..self
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }

    pub fn set_volume_variation(&mut self, volume_variation: f32) {
        self.volume_variation = volume_variation;
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn volume_variation(&self) -> f32 {
        self.volume_variation
    }

    pub fn with_pitch(self, pitch: f32, pitch_variation: f32) -> Self {
        Self {
            pitch,
            pitch_variation,
            ..self
        }
    }

    pub fn set_pitch(&mut self, pitch: f32) {
        self.pitch = pitch;
    }

    pub fn set_pitch_variation(&mut self, pitch_variation: f32) {
        self.pitch_variation = pitch_variation;
    }

    pub fn pitch(&self) -> f32 {
        self.pitch
    }

    pub fn pitch_variation(&self) -> f32 {
        self.pitch_variation
    }

    pub fn with_distance(self, distance: f32) -> Self {
        Self { distance, ..self }
    }

    pub fn set_distance(&mut self, distance: f32) {
        self.distance = distance;
    }

    pub fn distance(&self) -> f32 {
        self.distance
    }

    pub fn with_chance(self, chance: f32) -> Self {
        Self { chance, ..self }
    }

    pub fn set_chance(&mut self, chance: f32) {
        self.chance = chance;
    }

    pub fn chance(&self) -> f32 {
        self.chance
    }

    pub fn with_fade(self, fade_in: f32, fade_out: f32) -> Self {
        Self {
            fade_in,
            fade_out,
            ..self
        }
    }

    pub fn set_fade_in(&mut self, fade_in: f32) {
        self.fade_in = fade_in;
    }

    pub fn set_fade_out(&mut self, fade_out: f32) {
        self.fade_out = fade_out;
    }

    pub fn fade_in(&self) -> f32 {
        self.fade_in
    }

    pub fn fade_out(&self) -> f32 {
        self.fade_out
    }

    pub fn with_channel(self, channel: AudioPlusMixerChannel) -> Self {
        Self { channel, ..self }
    }

    pub fn set_channel(&mut self, channel: AudioPlusMixerChannel) {
        self.channel = channel;
    }

    pub fn channel(&self) -> AudioPlusMixerChannel {
        self.channel
    }
}

impl From<Handle<bevy_kira_audio::AudioSource>> for AudioPlusSoundEffect {
    fn from(resource: Handle<bevy_kira_audio::AudioSource>) -> Self {
        AudioPlusSoundEffect::single(resource)
    }
}
