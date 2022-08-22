pub use crate::common::prelude::*;
pub use audio_plus::prelude::*;

#[derive(Default)]
pub struct SoundEffects {
    pub sfx_menu_ambient: AudioPlusSoundEffect,
    pub sfx_menu_button_hover: AudioPlusSoundEffect,
    pub sfx_menu_button_click: AudioPlusSoundEffect,
    pub sfx_menu_button_click_confirm: AudioPlusSoundEffect,
    pub sfx_overworld_ambient: AudioPlusSoundEffect,
    pub sfx_jam_guitar_drums: AudioPlusSoundEffect,
    pub sfx_jam_guitar_flute: AudioPlusSoundEffect,
    pub sfx_jam_drums_flute: AudioPlusSoundEffect,
}

pub fn sound_effects_create(asset_library: &AssetLibrary) -> SoundEffects {
    SoundEffects {
        sfx_menu_ambient: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.audio_sfx_sea.clone()],
            volume: 0.2,
            ..Default::default()
        },
        sfx_menu_button_hover: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.audio_sfx_placeholder.clone()],
            volume: 0.6,
            ..Default::default()
        },
        sfx_menu_button_click: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.audio_sfx_placeholder.clone()],
            volume: 0.6,
            ..Default::default()
        },
        sfx_menu_button_click_confirm: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.audio_sfx_placeholder.clone()],
            volume: 0.6,
            ..Default::default()
        },
        sfx_overworld_ambient: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.audio_sfx_sea.clone()],
            volume: 0.2,
            ..Default::default()
        },
        sfx_jam_guitar_drums: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.music_guitar_drums.clone()],
            volume: 0.2,
            ..Default::default()
        },
        sfx_jam_guitar_flute: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.music_guitar_flute.clone()],
            volume: 0.2,
            ..Default::default()
        },
        sfx_jam_drums_flute: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.music_drums_flute.clone()],
            volume: 0.2,
            ..Default::default()
        },
    }
}