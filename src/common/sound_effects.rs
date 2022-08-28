pub use crate::common::prelude::*;
pub use audio_plus::prelude::*;

#[derive(Default)]
pub struct SoundEffects {
    // placeholder
    pub sfx_placeholder_music: AudioPlusSoundEffect,
    pub sfx_placeholder_sound: AudioPlusSoundEffect,

    // generic
    pub sfx_dialogue_start: AudioPlusSoundEffect,
    pub sfx_dialogue_proceed: AudioPlusSoundEffect,
    pub sfx_dialogue_repeat: AudioPlusSoundEffect,

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

    pub sfx_cutscene_outro_music: AudioPlusSoundEffect,
    pub sfx_cutscene_outro1: AudioPlusSoundEffect,
    pub sfx_cutscene_outro2: AudioPlusSoundEffect,
    pub sfx_cutscene_outro3: AudioPlusSoundEffect,

    // overworld
    pub sfx_overworld_ambient: AudioPlusSoundEffect,
    pub sfx_overworld_music: AudioPlusSoundEffect,
    pub sfx_overworld_town_enter: AudioPlusSoundEffect,
    pub sfx_overworld_town_exit: AudioPlusSoundEffect,
    pub sfx_overworld_dash: AudioPlusSoundEffect,
    pub sfx_overworld_experience: AudioPlusSoundEffect,
    pub sfx_overworld_player_damage: AudioPlusSoundEffect,
    pub sfx_overworld_enemy_damage: AudioPlusSoundEffect,
    pub sfx_overworld_map_open: AudioPlusSoundEffect,
    pub sfx_overworld_map_close: AudioPlusSoundEffect,
    pub sfx_overworld_level_up: AudioPlusSoundEffect,

    // attacks
    pub sfx_overworld_attack_forward_cannons: AudioPlusSoundEffect,
    pub sfx_overworld_attack_shotgun_cannons: AudioPlusSoundEffect,
    pub sfx_overworld_attack_shockwave: AudioPlusSoundEffect,
    pub sfx_overworld_attack_bombs: AudioPlusSoundEffect,
    pub sfx_overworld_attack_bomb_throw: AudioPlusSoundEffect,
    pub sfx_overworld_attack_bomb_explode: AudioPlusSoundEffect,
    pub sfx_overworld_attack_kraken: AudioPlusSoundEffect,
    pub sfx_overworld_attack_tentacle: AudioPlusSoundEffect,

    // town
    pub sfx_town_ambient: AudioPlusSoundEffect,
    pub sfx_town_music: AudioPlusSoundEffect,
    pub sfx_town_outside_hover: AudioPlusSoundEffect,
    pub sfx_town_outside_click: AudioPlusSoundEffect,
    pub sfx_town_rum_refill_jingle: AudioPlusSoundEffect,
    pub sfx_town_rum_refill_clank: AudioPlusSoundEffect,
    pub sfx_town_upgrade: AudioPlusSoundEffect,
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
        sfx_dialogue_start: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.audio_sfx_placeholder.clone()],
            volume: 0.0,
            ..Default::default()
        },
        sfx_dialogue_proceed: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.menu_sfx_button_click.clone()],
            volume: 1.0,
            ..Default::default()
        },
        sfx_dialogue_repeat: AudioPlusSoundEffect {
            audio_sources: vec![
                asset_library.audio_sfx_dialogue_voice_generic_01.clone(),
                asset_library.audio_sfx_dialogue_voice_generic_02.clone(),
                asset_library.audio_sfx_dialogue_voice_generic_03.clone(),
                asset_library.audio_sfx_dialogue_voice_generic_04.clone(),
                asset_library.audio_sfx_dialogue_voice_generic_05.clone(),
                asset_library.audio_sfx_dialogue_voice_generic_06.clone(),
                asset_library.audio_sfx_dialogue_voice_generic_07.clone(),
                asset_library.audio_sfx_dialogue_voice_generic_08.clone(),
                asset_library.audio_sfx_dialogue_voice_generic_09.clone(),
            ],
            volume: 0.6,
            pitch_variation: 0.2,
            chance: 0.85,
            ..Default::default()
        },
        sfx_menu_ambient: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.audio_sfx_sea.clone()],
            volume: 0.6,
            fade_in: 1.,
            fade_out: 1.,
            ..Default::default()
        },
        sfx_menu_music: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.menu_music.clone()],
            volume: 0.65,
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
        sfx_cutscene_outro_music: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.menu_music.clone()],
            volume: 0.1,
            fade_out: 1.0,
            ..Default::default()
        },
        sfx_cutscene_outro1: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.cutscene_voice_outro1.clone()],
            volume: 0.7,
            fade_out: 0.5,
            ..Default::default()
        },
        sfx_cutscene_outro2: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.cutscene_voice_outro2.clone()],
            volume: 0.7,
            fade_out: 0.5,
            ..Default::default()
        },
        sfx_cutscene_outro3: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.cutscene_voice_outro3.clone()],
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
            audio_sources: vec![asset_library.audio_sfx_town_enter.clone()],
            volume: 0.8,
            ..Default::default()
        },
        sfx_overworld_town_exit: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.audio_sfx_town_leave.clone()],
            volume: 0.8,
            ..Default::default()
        },
        sfx_overworld_dash: AudioPlusSoundEffect {
            audio_sources: vec![
                asset_library.audio_sfx_dash_01.clone(),
                asset_library.audio_sfx_dash_02.clone(),
                asset_library.audio_sfx_dash_03.clone(),
            ],
            volume: 0.25,
            ..Default::default()
        },
        sfx_overworld_experience: AudioPlusSoundEffect {
            audio_sources: vec![
                asset_library.audio_sfx_xp_01.clone(),
                asset_library.audio_sfx_xp_02.clone(),
                asset_library.audio_sfx_xp_03.clone(),
            ],
            volume: 0.85,
            ..Default::default()
        },
        sfx_overworld_player_damage: AudioPlusSoundEffect {
            audio_sources: vec![
                asset_library.audio_sfx_player_damage_01.clone(),
                asset_library.audio_sfx_player_damage_02.clone(),
                asset_library.audio_sfx_player_damage_03.clone(),
            ],
            volume: 0.6,
            ..Default::default()
        },
        sfx_overworld_enemy_damage: AudioPlusSoundEffect {
            audio_sources: vec![
                asset_library.audio_sfx_enemy_damage_01.clone(),
                asset_library.audio_sfx_enemy_damage_02.clone(),
                asset_library.audio_sfx_enemy_damage_03.clone(),
            ],
            positional: true,
            volume: 0.6,
            ..Default::default()
        },
        sfx_overworld_map_open: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.audio_sfx_map_open_01.clone()],
            volume: 0.5,
            ..Default::default()
        },
        sfx_overworld_map_close: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.audio_sfx_map_close_01.clone()],
            volume: 0.5,
            ..Default::default()
        },
        sfx_overworld_level_up: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.audio_sfx_level_up.clone()],
            volume: 0.8,
            ..Default::default()
        },
        sfx_overworld_attack_forward_cannons: AudioPlusSoundEffect {
            audio_sources: vec![
                asset_library.audio_sfx_attack_player_01.clone(),
                asset_library.audio_sfx_attack_player_02.clone(),
                asset_library.audio_sfx_attack_player_03.clone(),
            ],
            volume: 0.4,
            positional: true,
            ..Default::default()
        },
        sfx_overworld_attack_shotgun_cannons: AudioPlusSoundEffect {
            audio_sources: vec![
                asset_library.audio_sfx_attack_jagerossa_01.clone(),
                asset_library.audio_sfx_attack_jagerossa_02.clone(),
                asset_library.audio_sfx_attack_jagerossa_03.clone(),
            ],
            volume: 0.3,
            positional: true,
            ..Default::default()
        },
        sfx_overworld_attack_shockwave: AudioPlusSoundEffect {
            audio_sources: vec![
                asset_library.audio_sfx_attack_ringo_01.clone(),
                asset_library.audio_sfx_attack_ringo_02.clone(),
                asset_library.audio_sfx_attack_ringo_03.clone(),
            ],
            volume: 0.3,
            positional: true,
            ..Default::default()
        },
        sfx_overworld_attack_bombs: AudioPlusSoundEffect {
            audio_sources: vec![
                asset_library.audio_sfx_attack_plank_01.clone(),
                asset_library.audio_sfx_attack_plank_02.clone(),
                asset_library.audio_sfx_attack_plank_03.clone(),
            ],
            volume: 0.1,
            positional: true,
            ..Default::default()
        },
        sfx_overworld_attack_bomb_throw: AudioPlusSoundEffect {
            audio_sources: vec![
                asset_library.audio_sfx_bomb_throw_01.clone(),
                asset_library.audio_sfx_bomb_throw_02.clone(),
                asset_library.audio_sfx_bomb_throw_03.clone(),
            ],
            volume: 0.1,
            positional: true,
            ..Default::default()
        },
        sfx_overworld_attack_bomb_explode: AudioPlusSoundEffect {
            audio_sources: vec![
                asset_library.audio_sfx_bomb_explode_01.clone(),
                asset_library.audio_sfx_bomb_explode_02.clone(),
                asset_library.audio_sfx_bomb_explode_03.clone(),
            ],
            volume: 0.2,
            positional: true,
            ..Default::default()
        },
        sfx_overworld_attack_kraken: AudioPlusSoundEffect {
            audio_sources: vec![
                asset_library.audio_sfx_attack_davy_01.clone(),
                asset_library.audio_sfx_attack_davy_02.clone(),
                asset_library.audio_sfx_attack_davy_03.clone(),
            ],
            volume: 0.2,
            positional: true,
            ..Default::default()
        },
        sfx_overworld_attack_tentacle: AudioPlusSoundEffect {
            audio_sources: vec![
                asset_library.audio_sfx_tentacle_01.clone(),
                asset_library.audio_sfx_tentacle_02.clone(),
                asset_library.audio_sfx_tentacle_03.clone(),
            ],
            volume: 0.06,
            positional: true,
            pitch_variation: 0.1,
            ..Default::default()
        },
        sfx_overworld_music: AudioPlusSoundEffect::none(),
        sfx_town_ambient: AudioPlusSoundEffect::none(),
        sfx_town_music: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.menu_music.clone()],
            volume: 0.2,
            fade_in: 0.2,
            fade_out: 1.0,
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
        sfx_town_rum_refill_jingle: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.audio_sfx_town_rum_refill_jingle.clone()],
            volume: 1.0,
            ..Default::default()
        },
        sfx_town_rum_refill_clank: AudioPlusSoundEffect {
            audio_sources: vec![
                asset_library.audio_sfx_town_rum_refill_clank_01.clone(),
                asset_library.audio_sfx_town_rum_refill_clank_02.clone(),
                asset_library.audio_sfx_town_rum_refill_clank_03.clone(),
            ],
            volume: 1.0,
            ..Default::default()
        },
        sfx_town_upgrade: AudioPlusSoundEffect {
            audio_sources: vec![asset_library.audio_sfx_upgrade_01.clone()],
            volume: 1.0,
            ..Default::default()
        },
    }
}
