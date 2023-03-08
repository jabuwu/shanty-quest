use crate::common::prelude::*;
use audio_plus::prelude::*;
use bevy::prelude::*;

pub struct VolumeControlPlugin;

impl Plugin for VolumeControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(volume_control);
    }
}

fn volume_control(
    mut audio_mixer: ResMut<AudioPlusMixer>,
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    let volume = audio_mixer.get_master_volume();
    let mut play_sample = false;
    if input.just_pressed(KeyCode::P) {
        audio_mixer.set_master_volume(volume + 0.1);
        if audio_mixer.get_master_volume() != volume {
            play_sample = true;
        }
    }
    if input.just_pressed(KeyCode::O) {
        audio_mixer.set_master_volume((volume - 0.1).max(0.1));
        if audio_mixer.get_master_volume() != volume {
            play_sample = true;
        }
    }
    if play_sample {
        commands.spawn((
            AudioPlusSource::new(asset_library.sound_effects.sfx_audio_preview.clone())
                .as_playing(),
            Persistent,
            TimeToLive { seconds: 3. },
        ));
    }
}
