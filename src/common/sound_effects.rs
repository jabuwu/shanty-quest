pub use crate::common::prelude::*;
pub use audio_plus::prelude::*;

#[derive(Default)]
pub struct SoundEffects {
    // placeholder
    pub sfx_placeholder_music: AudioPlusSoundEffect,
    pub sfx_placeholder_sound: AudioPlusSoundEffect,

    // generic
    pub sfx_dialogue_progress: AudioPlusSoundEffect,

    // main menu
    pub sfx_menu_ambient: AudioPlusSoundEffect,
    pub sfx_menu_music: AudioPlusSoundEffect,
    pub sfx_menu_button_hover: AudioPlusSoundEffect,
    pub sfx_menu_button_click: AudioPlusSoundEffect,
    pub sfx_menu_button_click_confirm: AudioPlusSoundEffect,

    // cutscenes
    pub sfx_cutscene_intro_music: AudioPlusSoundEffect,
    pub sfx_cutscene_intro1: AudioPlusSoundEffect,
    pub sfx_cutscene_intro2: AudioPlusSoundEffect,
    pub sfx_cutscene_intro3: AudioPlusSoundEffect,
    pub sfx_cutscene_intro4: AudioPlusSoundEffect,
    pub sfx_cutscene_intro5: AudioPlusSoundEffect,

    // overworld
    pub sfx_overworld_ambient: AudioPlusSoundEffect,
    pub sfx_overworld_music: AudioPlusSoundEffect,
    pub sfx_overworld_town_enter: AudioPlusSoundEffect,
    pub sfx_overworld_town_exit: AudioPlusSoundEffect,

    // overworld attacks
    pub sfx_overworld_attack_forward_cannons: AudioPlusSoundEffect,

    // town
    pub sfx_town_ambient: AudioPlusSoundEffect,
    pub sfx_town_music: AudioPlusSoundEffect,
    pub sfx_town_outside_hover: AudioPlusSoundEffect,
    pub sfx_town_outside_click: AudioPlusSoundEffect,
}

pub fn sound_effects_create(asset_library: &AssetLibrary) -> SoundEffects {
    SoundEffects {
        sfx_placeholder_music: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.audio_music_placeholder.clone()],
            volume: 1.0,
            ..Default::default()
        },
        sfx_placeholder_sound: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.audio_sfx_placeholder.clone()],
            volume: 1.0,
            ..Default::default()
        },
        sfx_dialogue_progress: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.audio_sfx_placeholder.clone()],
            volume: 1.0,
            ..Default::default()
        },
        sfx_menu_ambient: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.audio_sfx_sea.clone()],
            volume: 0.5,
            fade_in: 1.,
            fade_out: 1.,
            ..Default::default()
        },
        sfx_menu_music: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.menu_music.clone()],
            volume: 1.,
            fade_out: 1.7,
            ..Default::default()
        },
        sfx_menu_button_hover: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.menu_sfx_button_hover.clone()],
            volume: 1.,
            ..Default::default()
        },
        sfx_menu_button_click: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.menu_sfx_button_click.clone()],
            volume: 1.,
            ..Default::default()
        },
        sfx_menu_button_click_confirm: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.menu_sfx_play.clone()],
            volume: 1.,
            ..Default::default()
        },
        sfx_cutscene_intro_music: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.menu_music.clone()],
            volume: 0.1,
            fade_out: 1.0,
            ..Default::default()
        },
        sfx_cutscene_intro1: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.cutscene_voice_intro1.clone()],
            volume: 0.7,
            fade_out: 0.5,
            ..Default::default()
        },
        sfx_cutscene_intro2: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.cutscene_voice_intro2.clone()],
            volume: 0.7,
            fade_out: 0.5,
            ..Default::default()
        },
        sfx_cutscene_intro3: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.cutscene_voice_intro3.clone()],
            volume: 0.7,
            fade_out: 0.5,
            ..Default::default()
        },
        sfx_cutscene_intro4: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.cutscene_voice_intro4.clone()],
            volume: 0.7,
            fade_out: 0.5,
            ..Default::default()
        },
        sfx_cutscene_intro5: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.cutscene_voice_intro5.clone()],
            volume: 0.7,
            fade_out: 0.5,
            ..Default::default()
        },
        sfx_overworld_ambient: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.audio_sfx_sea.clone()],
            volume: 0.4,
            fade_in: 1.,
            fade_out: 1.,
            ..Default::default()
        },
        sfx_overworld_town_enter: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.menu_sfx_play.clone()],
            volume: 0.5,
            ..Default::default()
        },
        sfx_overworld_town_exit: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.menu_sfx_play.clone()],
            volume: 0.5,
            ..Default::default()
        },
        sfx_overworld_attack_forward_cannons: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.audio_sfx_placeholder.clone()],
            volume: 0.5,
            positional: true,
            ..Default::default()
        },
        sfx_overworld_music: AudioPlusSoundEffect::none(),
        sfx_town_ambient: AudioPlusSoundEffect::none(),
        sfx_town_music: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.menu_music.clone()],
            volume: 0.2,
            fade_in: 0.2,
            fade_out: 0.2,
            ..Default::default()
        },
        sfx_town_outside_hover: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.menu_sfx_button_hover.clone()],
            volume: 1.,
            ..Default::default()
        },
        sfx_town_outside_click: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.menu_sfx_button_click.clone()],
            volume: 1.,
            ..Default::default()
        },
    }
}
