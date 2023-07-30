use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use channels::add_audio_channels;
use mixer::AudioPlusMixer;
use source::AudioPlusSource;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub enum AudioPlusSystem {
    UpdateAudioSources,
    Debug,
}

pub struct AudioPlusPlugin;

impl Plugin for AudioPlusPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .init_resource::<AudioPlusMixer>()
            .add_systems(
                Update,
                (
                    source::update_audio_sources.in_set(AudioPlusSystem::UpdateAudioSources),
                    debug.in_set(AudioPlusSystem::Debug),
                ),
            );
        add_audio_channels(app);
    }
}

fn debug(mut _query: Query<&mut AudioPlusSource>) {
    /*let mut unassigned_count = 0;
    let mut total = 0;
    for mut source in query.iter_mut() {
        for voice in source.voices.iter_mut() {
            total += 1;
            if voice.should_assign && !voice.assigned {
                unassigned_count += 1;
            }
        }
    }
    if unassigned_count > 0 {
        println!(
            "unassigned sfx: {} (total: {}) - {}",
            unassigned_count,
            total,
            rand::random::<f32>()
        );
    }*/
}

pub mod channels;
pub mod effect;
pub mod listener;
pub mod mixer;
pub mod prelude;
pub mod source;
pub mod voice;
