use bevy::prelude::*;

#[derive(Clone, Copy)]
pub(crate) struct AudioPlusVoiceHandle {
    pub(crate) entity: Entity,
    pub(crate) index: usize,
}

pub(crate) struct AudioPlusVoice {
    pub(crate) should_assign: bool,
    pub(crate) assigned: bool,
    pub(crate) audio_source: Option<Handle<bevy_kira_audio::AudioSource>>,
    pub(crate) volume: f32,
    pub(crate) volume_multiplier: f32,
    pub(crate) volume_fade: f32,
    pub(crate) panning: f32,
    pub(crate) playback_rate: f32,
    pub(crate) state: AudioPlusVoiceState,
    pub(crate) state_dirty: bool,
    pub(crate) stopping: bool,
    pub(crate) status: AudioPlusVoiceStatus,
}

#[derive(Default)]
pub(crate) struct AudioPlusVoiceStatus {
    pub(crate) initialized: bool,
    pub(crate) playing: bool,
}

impl AudioPlusVoice {
    pub(crate) fn new() -> Self {
        Self {
            should_assign: false,
            assigned: false,
            audio_source: None,
            volume: 0.,
            volume_multiplier: 1.,
            volume_fade: 0.,
            panning: 0.5,
            playback_rate: 1.,
            state: AudioPlusVoiceState::Stopped,
            state_dirty: false,
            stopping: false,
            status: AudioPlusVoiceStatus::default(),
        }
    }

    pub(crate) fn reset(&mut self) {
        self.should_assign = false;
        self.assigned = false;
        self.audio_source = None;
        self.volume = 0.;
        self.volume_multiplier = 1.;
        self.volume_fade = 0.;
        self.panning = 0.5;
        self.playback_rate = 1.;
        self.state = AudioPlusVoiceState::Stopped;
        self.stopping = false;
        self.status = AudioPlusVoiceStatus::default();
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AudioPlusVoiceState {
    #[default]
    Stopped,
    Playing,
    Looping,
}
