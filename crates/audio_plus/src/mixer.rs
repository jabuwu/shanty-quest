use bevy::prelude::*;

const CHANNEL_COUNT: usize = 2;
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum AudioPlusMixerChannel {
    None = 0,
    Music = 1,
    Sfx = 2,
}

#[derive(Resource)]
pub struct AudioPlusMixer {
    master_volume: f32,
    volumes: [f32; CHANNEL_COUNT],
}

impl Default for AudioPlusMixer {
    fn default() -> Self {
        Self {
            master_volume: 1.,
            volumes: [1.; CHANNEL_COUNT],
        }
    }
}

impl AudioPlusMixer {
    pub fn set_master_volume(&mut self, volume: f32) {
        self.master_volume = volume.clamp(0., 1.);
    }
    pub fn get_master_volume(&self) -> f32 {
        self.master_volume
    }
    pub fn set_volume(&mut self, channel: AudioPlusMixerChannel, volume: f32) {
        let channel_id = channel as usize;
        if channel_id != 0 && channel_id <= CHANNEL_COUNT {
            self.volumes[channel_id - 1] = volume;
        }
    }
    pub fn get_volume(&self, channel: AudioPlusMixerChannel) -> f32 {
        let channel_id = channel as usize;
        if channel_id != 0 && channel_id <= CHANNEL_COUNT {
            self.volumes[channel_id - 1]
        } else {
            1.
        }
    }
}
