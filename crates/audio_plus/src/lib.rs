use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use channels::add_audio_channels;
use mixer::AudioPlusMixer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
pub enum AudioPlusSystems {
    UpdateAudioSources,
}

pub struct AudioPlusPlugin;

impl Plugin for AudioPlusPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .init_resource::<AudioPlusMixer>()
            .add_system(source::update_audio_sources.label(AudioPlusSystems::UpdateAudioSources));
        add_audio_channels(app);
    }
}

pub mod channels;
pub mod effect;
pub mod listener;
pub mod mixer;
pub mod prelude;
pub mod source;
pub mod voice;
