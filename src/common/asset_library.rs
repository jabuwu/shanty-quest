use crate::common::prelude::*;
use crate::common::sound_effects::{sound_effects_create, SoundEffects};
use asset_struct::AssetStruct;
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

#[derive(Default, AssetStruct)]
pub struct AssetLibrary {
    pub sound_effects: SoundEffects,

    /***************
     * Placeholder *
     ***************/
    #[asset("audio/sfx/placeholder.ogg")]
    pub audio_sfx_placeholder: Handle<AudioSource>,
    #[asset("audio/music/placeholder.ogg")]
    pub audio_music_placeholder: Handle<AudioSource>,

    /***********
     * Generic *
     ***********/
    #[asset("audio/sfx/amb_sea_01.ogg")]
    pub audio_sfx_sea: Handle<AudioSource>,
    #[asset("fonts/IMFellDoublePica-Regular.ttf")]
    pub font_default: Handle<Font>,
    #[asset("fonts/Pirate Kids.otf")]
    pub font_bold: Handle<Font>,
    #[asset("sprites/OverworldDialogBG.png")]
    pub sprite_dialogue_bg: Handle<Image>,
    #[asset("sprites/portrait_a.png")]
    pub sprite_dialogue_portrait_guitar: Handle<Image>,
    #[asset("audio/sfx/dialogue_voice_generic_01.ogg")]
    pub audio_sfx_dialogue_voice_generic_01: Handle<AudioSource>,
    #[asset("audio/sfx/dialogue_voice_generic_02.ogg")]
    pub audio_sfx_dialogue_voice_generic_02: Handle<AudioSource>,
    #[asset("audio/sfx/dialogue_voice_generic_03.ogg")]
    pub audio_sfx_dialogue_voice_generic_03: Handle<AudioSource>,
    #[asset("audio/sfx/dialogue_voice_generic_04.ogg")]
    pub audio_sfx_dialogue_voice_generic_04: Handle<AudioSource>,
    #[asset("audio/sfx/dialogue_voice_generic_05.ogg")]
    pub audio_sfx_dialogue_voice_generic_05: Handle<AudioSource>,
    #[asset("audio/sfx/dialogue_voice_generic_06.ogg")]
    pub audio_sfx_dialogue_voice_generic_06: Handle<AudioSource>,
    #[asset("audio/sfx/dialogue_voice_generic_07.ogg")]
    pub audio_sfx_dialogue_voice_generic_07: Handle<AudioSource>,
    #[asset("audio/sfx/dialogue_voice_generic_08.ogg")]
    pub audio_sfx_dialogue_voice_generic_08: Handle<AudioSource>,
    #[asset("audio/sfx/dialogue_voice_generic_09.ogg")]
    pub audio_sfx_dialogue_voice_generic_09: Handle<AudioSource>,

    /*********
     * Level *
     *********/
    #[asset("levels/level.ldtk")]
    pub level: Handle<LdtkAsset>,
    #[asset("levels/IslandTilemap.png")]
    pub levelisland_tilemap: Handle<Image>,

    #[asset("levels/WaterTileMap.png")]
    pub levelwater_tilemap: Handle<Image>,

    /*************
     * Main Menu *
     *************/
    #[asset("sprites/back_menu.png")]
    pub menu_sprite_back: Handle<Image>,
    #[asset("sprites/logo.png")]
    pub menu_sprite_logo: Handle<Image>,
    #[asset("sprites/vfx_shine.png")]
    pub menu_sprite_shine: Handle<Image>,
    #[asset("sprites/but_back_a.png")]
    pub menu_sprite_button_back: Handle<Image>,
    #[asset("sprites/but_play_normal.png")]
    pub menu_sprite_button_play_normal: Handle<Image>,
    #[asset("sprites/but_play_hover.png")]
    pub menu_sprite_button_play_hover: Handle<Image>,
    #[asset("sprites/but_play_press.png")]
    pub menu_sprite_button_play_press: Handle<Image>,
    #[asset("sprites/icon_skull.png")]
    pub menu_sprite_skull: Handle<Image>,
    #[asset("sprites/icon_volume.png")]
    pub menu_sprite_volume: Handle<Image>,

    #[asset("audio/music/menu.ogg")]
    pub menu_music: Handle<AudioSource>,
    #[asset("audio/sfx/ui_button_click_01.ogg")]
    pub menu_sfx_button_click: Handle<AudioSource>,
    #[asset("audio/sfx/ui_button_hover_01.ogg")]
    pub menu_sfx_button_hover: Handle<AudioSource>,
    #[asset("audio/sfx/ui_menu_play_01.ogg")]
    pub menu_sfx_play: Handle<AudioSource>,

    /*************
     * Cutscenes *
     *************/
    #[asset("sprites/cutscene_intro1.png")]
    pub cutscene_image_intro1: Handle<Image>,
    #[asset("sprites/cutscene_intro2.png")]
    pub cutscene_image_intro2: Handle<Image>,
    #[asset("sprites/cutscene_intro3.png")]
    pub cutscene_image_intro3: Handle<Image>,
    #[asset("sprites/cutscene_intro4.png")]
    pub cutscene_image_intro4: Handle<Image>,
    #[asset("sprites/cutscene_intro5.png")]
    pub cutscene_image_intro5: Handle<Image>,
    #[asset("audio/voice/intro1.ogg")]
    pub cutscene_voice_intro1: Handle<AudioSource>,
    #[asset("audio/voice/intro2.ogg")]
    pub cutscene_voice_intro2: Handle<AudioSource>,
    #[asset("audio/voice/intro3.ogg")]
    pub cutscene_voice_intro3: Handle<AudioSource>,
    #[asset("audio/voice/intro4.ogg")]
    pub cutscene_voice_intro4: Handle<AudioSource>,
    #[asset("audio/voice/intro5.ogg")]
    pub cutscene_voice_intro5: Handle<AudioSource>,

    /*************
     * Overworld *
     *************/
    #[asset("sprites/Ship1.png")]
    pub sprite_ship: Handle<Image>,
    pub sprite_ship_atlas: Handle<TextureAtlas>,

    #[asset("sprites/OverworldCity.png")]
    pub sprite_overworld_city: Handle<Image>,

    #[asset("sprites/WaterOverlay.png")]
    pub sprite_water_overlay: Handle<Image>,
    #[asset("sprites/WaterOverlay2.png")]
    pub sprite_water_overlay2: Handle<Image>,

    #[asset("sprites/BulletNote.png")]
    pub sprite_bullet_note: Handle<Image>,

    #[asset("sprites/WaterRingVFX.png")]
    pub sprite_water_ring_vfx: Handle<Image>,

    #[asset("sprites/ShockwaveVFX.png")]
    pub sprite_shockwave_vfx: Handle<Image>,

    #[asset("sprites/Turtle.png")]
    pub sprite_turtle: Handle<Image>,
    pub sprite_turtle_atlas: Handle<TextureAtlas>,

    #[asset("sprites/Octopus.png")]
    pub sprite_octopus: Handle<Image>,
    pub sprite_octopus_atlas: Handle<TextureAtlas>,

    /********
     * Town *
     ********/
    #[asset("sprites/City1_BG.png")]
    pub sprite_town_bg: Handle<Image>,
    #[asset("sprites/City1_BG_Hole.png")]
    pub sprite_town_bg_hole: Handle<Image>,
    #[asset("sprites/City1_Tavern_Outline.png")]
    pub sprite_town_tavern_outline: Handle<Image>,
    #[asset("sprites/City1_Mayor_Outline.png")]
    pub sprite_town_mayor_outline: Handle<Image>,
    #[asset("sprites/City1_ConcertHall_Outline.png")]
    pub sprite_town_concert_hall_outline: Handle<Image>,

    /***********************
     * Town / Concert Hall *
     ***********************/
    #[asset("sprites/BandSelection_BG.png")]
    pub sprite_band_selection_bg: Handle<Image>,
    #[asset("sprites/BandSelectionGUI_LockedSlot.png")]
    pub sprite_band_selection_slot_locked: Handle<Image>,
    #[asset("sprites/BandSelectionGUI_Slot_Guitar_Active.png")]
    pub sprite_band_selection_slot_guitar_active: Handle<Image>,
    #[asset("sprites/BandSelectionGUI_Slot_Guitar_Inactive.png")]
    pub sprite_band_selection_slot_guitar_inactive: Handle<Image>,
    #[asset("sprites/BandSelectionGUI_Slot_Drums_Active.png")]
    pub sprite_band_selection_slot_drums_active: Handle<Image>,
    #[asset("sprites/BandSelectionGUI_Slot_Drums_Inactive.png")]
    pub sprite_band_selection_slot_drums_inactive: Handle<Image>,
    #[asset("sprites/BandSelectionGUI_Slot_Flute_Active.png")]
    pub sprite_band_selection_slot_flute_active: Handle<Image>,
    #[asset("sprites/BandSelectionGUI_Slot_Flute_Inactive.png")]
    pub sprite_band_selection_slot_flute_inactive: Handle<Image>,
    #[asset("sprites/AbilityIcon_Shotgun.png")]
    pub sprite_band_selection_slot_ability_shotgun: Handle<Image>,
}

impl AssetLibrary {
    pub fn create_texture_atlases(&mut self, texture_atlas_assets: &mut Assets<TextureAtlas>) {
        let texture_atlas =
            TextureAtlas::from_grid(self.sprite_ship.clone(), Vec2::new(250., 350.), 5, 1);
        self.sprite_ship_atlas = texture_atlas_assets.add(texture_atlas);

        let texture_atlas =
            TextureAtlas::from_grid(self.sprite_octopus.clone(), Vec2::new(162., 130.), 2, 1);
        self.sprite_octopus_atlas = texture_atlas_assets.add(texture_atlas);

        let texture_atlas =
            TextureAtlas::from_grid(self.sprite_turtle.clone(), Vec2::new(162., 130.), 2, 1);
        self.sprite_turtle_atlas = texture_atlas_assets.add(texture_atlas);
    }

    pub fn create_sound_effects(&mut self) {
        self.sound_effects = sound_effects_create(&self);
    }
}
