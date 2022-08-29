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
    #[asset("sprites/BottomFade.png")]
    pub sprite_dialogue_fade: Handle<Image>,
    #[asset("sprites/OverworldDialogBG.png")]
    pub sprite_dialogue_bg: Handle<Image>,
    #[asset("sprites/portrait_a.png")]
    pub sprite_dialogue_portrait_jagerossa: Handle<Image>,
    #[asset("sprites/portrait_elvis.png")]
    pub sprite_dialogue_portrait_elvis: Handle<Image>,
    #[asset("sprites/portrait_bowie.png")]
    pub sprite_dialogue_portrait_bowie: Handle<Image>,
    #[asset("sprites/portrait_ringo.png")]
    pub sprite_dialogue_portrait_ringo: Handle<Image>,
    #[asset("sprites/portrait_barkeep.png")]
    pub sprite_dialogue_portrait_barkeep: Handle<Image>,
    #[asset("sprites/portrait_governor.png")]
    pub sprite_dialogue_portrait_governor: Handle<Image>,
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

    #[asset("levels/DecorTilemap.png")]
    pub decor_tilemap: Handle<Image>,

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

    #[asset("sprites/cutscene_outro1.png")]
    pub cutscene_image_outro1: Handle<Image>,
    #[asset("sprites/cutscene_outro2.png")]
    pub cutscene_image_outro2: Handle<Image>,
    #[asset("sprites/cutscene_outro3.png")]
    pub cutscene_image_outro3: Handle<Image>,
    #[asset("audio/voice/outro1.ogg")]
    pub cutscene_voice_outro1: Handle<AudioSource>,
    #[asset("audio/voice/outro2.ogg")]
    pub cutscene_voice_outro2: Handle<AudioSource>,
    #[asset("audio/voice/outro3.ogg")]
    pub cutscene_voice_outro3: Handle<AudioSource>,

    /*************
     * Overworld *
     *************/
    #[asset("sprites/Ships/Ship_Red.png")]
    pub sprite_ship_red: Handle<Image>,
    pub sprite_ship_red_atlas: Handle<TextureAtlas>,

    #[asset("sprites/Ships/Ship_Blue.png")]
    pub sprite_ship_blue: Handle<Image>,
    pub sprite_ship_blue_atlas: Handle<TextureAtlas>,

    #[asset("sprites/Ships/Ship_Green.png")]
    pub sprite_ship_green: Handle<Image>,
    pub sprite_ship_green_atlas: Handle<TextureAtlas>,

    #[asset("sprites/Ships/Ship_Purple.png")]
    pub sprite_ship_purple: Handle<Image>,
    pub sprite_ship_purple_atlas: Handle<TextureAtlas>,

    #[asset("sprites/Ships/Ship_Brown.png")]
    pub sprite_ship_brown: Handle<Image>,
    pub sprite_ship_brown_atlas: Handle<TextureAtlas>,

    #[asset("sprites/ExperienceParticle.png")]
    pub sprite_exp_particle: Handle<Image>,

    #[asset("sprites/Rubble.png")]
    pub sprite_rubble: Handle<Image>,

    #[asset("sprites/OverworldCity.png")]
    pub sprite_overworld_city: Handle<Image>,

    #[asset("sprites/WaterOverlay.png")]
    pub sprite_water_overlay: Handle<Image>,
    #[asset("sprites/WaterOverlay2.png")]
    pub sprite_water_overlay2: Handle<Image>,

    #[asset("sprites/WaterRingVFX.png")]
    pub sprite_water_ring_vfx: Handle<Image>,

    #[asset("sprites/Turtle.png")]
    pub sprite_turtle: Handle<Image>,
    pub sprite_turtle_atlas: Handle<TextureAtlas>,

    #[asset("sprites/Octopus.png")]
    pub sprite_octopus_easy: Handle<Image>,
    pub sprite_octopus_easy_atlas: Handle<TextureAtlas>,
    #[asset("sprites/Octopus_Blue.png")]
    pub sprite_octopus_medium: Handle<Image>,
    pub sprite_octopus_medium_atlas: Handle<TextureAtlas>,
    #[asset("sprites/Octopus_Red.png")]
    pub sprite_octopus_hard: Handle<Image>,
    pub sprite_octopus_hard_atlas: Handle<TextureAtlas>,

    #[asset("sprites/Turtle.png")]
    pub sprite_turtle_easy: Handle<Image>,
    pub sprite_turtle_easy_atlas: Handle<TextureAtlas>,
    #[asset("sprites/Turtle_Blue.png")]
    pub sprite_turtle_medium: Handle<Image>,
    pub sprite_turtle_medium_atlas: Handle<TextureAtlas>,
    #[asset("sprites/Turtle_Red.png")]
    pub sprite_turtle_hard: Handle<Image>,
    pub sprite_turtle_hard_atlas: Handle<TextureAtlas>,

    #[asset("sprites/AnglerFish.png")]
    pub sprite_angler_fish: Handle<Image>,
    pub sprite_angler_fish_atlas: Handle<TextureAtlas>,

    #[asset("sprites/Bottle2.png")]
    pub sprite_rum_bottle: Handle<Image>,

    #[asset("audio/sfx/ui_xp_collect_01.ogg")]
    pub audio_sfx_xp_01: Handle<AudioSource>,
    #[asset("audio/sfx/ui_xp_collect_02.ogg")]
    pub audio_sfx_xp_02: Handle<AudioSource>,
    #[asset("audio/sfx/ui_xp_collect_03.ogg")]
    pub audio_sfx_xp_03: Handle<AudioSource>,

    #[asset("audio/sfx/player_take_damage_01.ogg")]
    pub audio_sfx_player_damage_01: Handle<AudioSource>,
    #[asset("audio/sfx/player_take_damage_02.ogg")]
    pub audio_sfx_player_damage_02: Handle<AudioSource>,
    #[asset("audio/sfx/player_take_damage_03.ogg")]
    pub audio_sfx_player_damage_03: Handle<AudioSource>,
    #[asset("audio/sfx/player_died_jingle_01.ogg")]
    pub audio_sfx_player_died: Handle<AudioSource>,

    #[asset("audio/sfx/enemy_take_damage_01.ogg")]
    pub audio_sfx_enemy_damage_01: Handle<AudioSource>,
    #[asset("audio/sfx/enemy_take_damage_02.ogg")]
    pub audio_sfx_enemy_damage_02: Handle<AudioSource>,
    #[asset("audio/sfx/enemy_take_damage_03.ogg")]
    pub audio_sfx_enemy_damage_03: Handle<AudioSource>,

    #[asset("audio/sfx/ui_map_open_01.ogg")]
    pub audio_sfx_map_open_01: Handle<AudioSource>,

    #[asset("audio/sfx/ui_map_close_01.ogg")]
    pub audio_sfx_map_close_01: Handle<AudioSource>,

    #[asset("audio/sfx/ui_town_enter_01.ogg")]
    pub audio_sfx_town_enter: Handle<AudioSource>,

    #[asset("audio/sfx/ui_town_leave_01.ogg")]
    pub audio_sfx_town_leave: Handle<AudioSource>,

    /****************
     * Overworld UI *
     ****************/
    #[asset("sprites/WorldQuestMarker_Icon.png")]
    pub sprite_world_quest_marker_icon: Handle<Image>,
    #[asset("sprites/WorldQuestMarker_Arrow.png")]
    pub sprite_world_quest_marker_arrow: Handle<Image>,
    #[asset("sprites/WorldTownMarker_Icon.png")]
    pub sprite_world_town_marker_icon: Handle<Image>,
    #[asset("sprites/HUD_ObjectiveHeader.png")]
    pub sprite_objective_bg: Handle<Image>,
    #[asset("sprites/ScreenEdges.png")]
    pub sprite_screen_edges: Handle<Image>,

    #[asset("sprites/Map_BG.png")]
    pub sprite_map_bg: Handle<Image>,
    #[asset("sprites/Map_Icon_Boat.png")]
    pub sprite_map_icon_boat: Handle<Image>,
    #[asset("sprites/Map_Icon_Quest.png")]
    pub sprite_map_icon_quest: Handle<Image>,
    #[asset("sprites/Map_Icon_Town.png")]
    pub sprite_map_icon_town: Handle<Image>,
    #[asset("sprites/Map_CompassRose.png")]
    pub sprite_map_compass: Handle<Image>,

    #[asset("sprites/Bottle.png")]
    pub sprite_health_bottle: Handle<Image>,
    pub sprite_health_bottle_atlas: Handle<TextureAtlas>,

    #[asset("sprites/HUD_Dash_Ability.png")]
    pub sprite_controls_dash: Handle<Image>,
    #[asset("sprites/HUD_Open_Map.png")]
    pub sprite_controls_map: Handle<Image>,
    #[asset("sprites/HUD_Jam_Ability.png")]
    pub sprite_controls_jam: Handle<Image>,
    pub sprite_controls_jam_atlas: Handle<TextureAtlas>,
    #[asset("sprites/HUD_Jam_Ability_Keybind.png")]
    pub sprite_controls_jam_key: Handle<Image>,

    #[asset("sprites/HUD_Exp_Bar_BG.png")]
    pub sprite_experience_bar_bg: Handle<Image>,

    #[asset("sprites/HUD_Skill_Points_Notify_BG.png")]
    pub sprite_experience_skill_point_bg: Handle<Image>,

    #[asset("audio/sfx/ui_level_up_01.ogg")]
    pub audio_sfx_level_up: Handle<AudioSource>,

    /***********
     * Attacks *
     ***********/
    #[asset("sprites/BulletNote.png")]
    pub sprite_bullet_note: Handle<Image>,
    #[asset("sprites/ShockwaveVFX.png")]
    pub sprite_shockwave_vfx: Handle<Image>,
    #[asset("sprites/Bomb2.png")]
    pub sprite_bomb: Handle<Image>,
    pub sprite_bomb_atlas: Handle<TextureAtlas>,
    #[asset("sprites/BombExplosion.png")]
    pub sprite_bomb_explosion: Handle<Image>,
    #[asset("sprites/Tentacle.png")]
    pub sprite_tentacle: Handle<Image>,
    pub sprite_tentacle_atlas: Handle<TextureAtlas>,

    #[asset("audio/music/shoot_drums_01.ogg")]
    pub audio_sfx_attack_player_01: Handle<AudioSource>,
    #[asset("audio/music/shoot_drums_02.ogg")]
    pub audio_sfx_attack_player_02: Handle<AudioSource>,
    #[asset("audio/music/shoot_drums_03.ogg")]
    pub audio_sfx_attack_player_03: Handle<AudioSource>,
    #[asset("audio/music/shoot_electricguitar_01.ogg")]
    pub audio_sfx_attack_jagerossa_01: Handle<AudioSource>,
    #[asset("audio/music/shoot_electricguitar_02.ogg")]
    pub audio_sfx_attack_jagerossa_02: Handle<AudioSource>,
    #[asset("audio/music/shoot_electricguitar_03.ogg")]
    pub audio_sfx_attack_jagerossa_03: Handle<AudioSource>,
    #[asset("audio/music/shoot_flute_01.ogg")]
    pub audio_sfx_attack_ringo_01: Handle<AudioSource>,
    #[asset("audio/music/shoot_flute_02.ogg")]
    pub audio_sfx_attack_ringo_02: Handle<AudioSource>,
    #[asset("audio/music/shoot_flute_03.ogg")]
    pub audio_sfx_attack_ringo_03: Handle<AudioSource>,
    #[asset("audio/music/shoot_harmonica_01.ogg")]
    pub audio_sfx_attack_plank_01: Handle<AudioSource>,
    #[asset("audio/music/shoot_harmonica_02.ogg")]
    pub audio_sfx_attack_plank_02: Handle<AudioSource>,
    #[asset("audio/music/shoot_harmonica_03.ogg")]
    pub audio_sfx_attack_plank_03: Handle<AudioSource>,
    #[asset("audio/music/shoot_accordion_01.ogg")]
    pub audio_sfx_attack_davy_01: Handle<AudioSource>,
    #[asset("audio/music/shoot_accordion_02.ogg")]
    pub audio_sfx_attack_davy_02: Handle<AudioSource>,
    #[asset("audio/music/shoot_accordion_03.ogg")]
    pub audio_sfx_attack_davy_03: Handle<AudioSource>,

    #[asset("audio/sfx/bomb_explode_01.ogg")]
    pub audio_sfx_bomb_explode_01: Handle<AudioSource>,
    #[asset("audio/sfx/bomb_explode_02.ogg")]
    pub audio_sfx_bomb_explode_02: Handle<AudioSource>,
    #[asset("audio/sfx/bomb_explode_03.ogg")]
    pub audio_sfx_bomb_explode_03: Handle<AudioSource>,
    #[asset("audio/sfx/bomb_throw_01.ogg")]
    pub audio_sfx_bomb_throw_01: Handle<AudioSource>,
    #[asset("audio/sfx/bomb_throw_02.ogg")]
    pub audio_sfx_bomb_throw_02: Handle<AudioSource>,
    #[asset("audio/sfx/bomb_throw_03.ogg")]
    pub audio_sfx_bomb_throw_03: Handle<AudioSource>,

    #[asset("audio/sfx/tentacle_appear_01.ogg")]
    pub audio_sfx_tentacle_01: Handle<AudioSource>,
    #[asset("audio/sfx/tentacle_appear_02.ogg")]
    pub audio_sfx_tentacle_02: Handle<AudioSource>,
    #[asset("audio/sfx/tentacle_appear_03.ogg")]
    pub audio_sfx_tentacle_03: Handle<AudioSource>,

    /********
     * Dash *
     ********/
    #[asset("audio/sfx/player_dash_01.ogg")]
    pub audio_sfx_dash_01: Handle<AudioSource>,
    #[asset("audio/sfx/player_dash_02.ogg")]
    pub audio_sfx_dash_02: Handle<AudioSource>,
    #[asset("audio/sfx/player_dash_03.ogg")]
    pub audio_sfx_dash_03: Handle<AudioSource>,

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
    #[asset("sprites/WorldTavernMarker_Icon.png")]
    pub sprite_town_tavern_notify: Handle<Image>,
    #[asset("sprites/WorldQuestMarker_Icon.png")]
    pub sprite_town_mayor_notify: Handle<Image>,
    #[asset("sprites/WorldSkillMarker_Icon.png")]
    pub sprite_town_concert_hall_notify: Handle<Image>,
    #[asset("audio/sfx/ui_rum_refill_jingle_01.ogg")]
    pub audio_sfx_town_rum_refill_jingle: Handle<AudioSource>,
    #[asset("audio/sfx/ui_rum_refill_clank_01.ogg")]
    pub audio_sfx_town_rum_refill_clank_01: Handle<AudioSource>,
    #[asset("audio/sfx/ui_rum_refill_clank_02.ogg")]
    pub audio_sfx_town_rum_refill_clank_02: Handle<AudioSource>,
    #[asset("audio/sfx/ui_rum_refill_clank_03.ogg")]
    pub audio_sfx_town_rum_refill_clank_03: Handle<AudioSource>,
    #[asset("audio/sfx/ui_confirm_01.ogg")]
    pub audio_sfx_upgrade_01: Handle<AudioSource>,

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

    /*******************
     * Town / Upgrades *
     *******************/
    #[asset("sprites/UpgradeScreen/UpgradeBand_BG.png")]
    pub sprite_upgrades_bg: Handle<Image>,
    #[asset("sprites/UpgradeScreen/UpgradeBand_AbilityBG.png")]
    pub sprite_upgrades_ability_bg: Handle<Image>,
    pub sprite_upgrades_ability_bg_atlas: Handle<TextureAtlas>,
    #[asset("sprites/UpgradeScreen/GUI_Instrument_Guitar.png")]
    pub sprite_upgrades_ability_guitar: Handle<Image>,
    #[asset("sprites/UpgradeScreen/GUI_Instrument_Drums.png")]
    pub sprite_upgrades_ability_drums: Handle<Image>,
    #[asset("sprites/UpgradeScreen/GUI_Instrument_Flute.png")]
    pub sprite_upgrades_ability_flute: Handle<Image>,
    #[asset("sprites/UpgradeScreen/GUI_Instrument_Harmonica.png")]
    pub sprite_upgrades_ability_harmonica: Handle<Image>,
    #[asset("sprites/UpgradeScreen/GUI_Instrument_Accordion.png")]
    pub sprite_upgrades_ability_accordion: Handle<Image>,
    #[asset("sprites/UpgradeScreen/GUI_Instrument_Defense.png")]
    pub sprite_upgrades_ability_defense: Handle<Image>,
    #[asset("sprites/UpgradeScreen/UpgradeBand_UpgradeBtn.png")]
    pub sprite_upgrades_button: Handle<Image>,
    pub sprite_upgrades_button_atlas: Handle<TextureAtlas>,
    #[asset("sprites/UpgradeScreen/UpgradeBand_Star.png")]
    pub sprite_upgrades_star: Handle<Image>,

    /********
     * Dead *
     ********/
    #[asset("sprites/DeathSkull.png")]
    pub sprite_dead: Handle<Image>,
}

impl AssetLibrary {
    pub fn create_texture_atlases(&mut self, texture_atlas_assets: &mut Assets<TextureAtlas>) {
        let texture_atlas =
            TextureAtlas::from_grid(self.sprite_ship_red.clone(), Vec2::new(250., 350.), 5, 1);
        self.sprite_ship_red_atlas = texture_atlas_assets.add(texture_atlas);

        let texture_atlas =
            TextureAtlas::from_grid(self.sprite_ship_brown.clone(), Vec2::new(250., 350.), 5, 1);
        self.sprite_ship_brown_atlas = texture_atlas_assets.add(texture_atlas);
        let texture_atlas =
            TextureAtlas::from_grid(self.sprite_ship_green.clone(), Vec2::new(250., 350.), 5, 1);
        self.sprite_ship_green_atlas = texture_atlas_assets.add(texture_atlas);
        let texture_atlas =
            TextureAtlas::from_grid(self.sprite_ship_blue.clone(), Vec2::new(250., 350.), 5, 1);
        self.sprite_ship_blue_atlas = texture_atlas_assets.add(texture_atlas);
        let texture_atlas =
            TextureAtlas::from_grid(self.sprite_ship_purple.clone(), Vec2::new(250., 350.), 5, 1);
        self.sprite_ship_purple_atlas = texture_atlas_assets.add(texture_atlas);

        let texture_atlas = TextureAtlas::from_grid(
            self.sprite_octopus_easy.clone(),
            Vec2::new(131., 130.),
            2,
            1,
        );
        self.sprite_octopus_easy_atlas = texture_atlas_assets.add(texture_atlas);

        let texture_atlas = TextureAtlas::from_grid(
            self.sprite_octopus_medium.clone(),
            Vec2::new(131., 130.),
            2,
            1,
        );
        self.sprite_octopus_medium_atlas = texture_atlas_assets.add(texture_atlas);

        let texture_atlas = TextureAtlas::from_grid(
            self.sprite_octopus_hard.clone(),
            Vec2::new(131., 130.),
            2,
            1,
        );
        self.sprite_octopus_hard_atlas = texture_atlas_assets.add(texture_atlas);

        let texture_atlas =
            TextureAtlas::from_grid(self.sprite_turtle_easy.clone(), Vec2::new(131., 130.), 2, 1);
        self.sprite_turtle_easy_atlas = texture_atlas_assets.add(texture_atlas);

        let texture_atlas = TextureAtlas::from_grid(
            self.sprite_turtle_medium.clone(),
            Vec2::new(131., 130.),
            2,
            1,
        );
        self.sprite_turtle_medium_atlas = texture_atlas_assets.add(texture_atlas);

        let texture_atlas =
            TextureAtlas::from_grid(self.sprite_turtle_hard.clone(), Vec2::new(131., 130.), 2, 1);
        self.sprite_turtle_hard_atlas = texture_atlas_assets.add(texture_atlas);

        let texture_atlas =
            TextureAtlas::from_grid(self.sprite_turtle.clone(), Vec2::new(131., 130.), 2, 1);
        self.sprite_turtle_atlas = texture_atlas_assets.add(texture_atlas);

        let texture_atlas =
            TextureAtlas::from_grid(self.sprite_bomb.clone(), Vec2::new(131., 130.), 4, 1);
        self.sprite_bomb_atlas = texture_atlas_assets.add(texture_atlas);

        let texture_atlas =
            TextureAtlas::from_grid(self.sprite_tentacle.clone(), Vec2::new(131., 211.), 4, 1);
        self.sprite_tentacle_atlas = texture_atlas_assets.add(texture_atlas);

        let texture_atlas = TextureAtlas::from_grid(
            self.sprite_health_bottle.clone(),
            Vec2::new(102., 163.),
            3,
            1,
        );
        self.sprite_health_bottle_atlas = texture_atlas_assets.add(texture_atlas);

        let texture_atlas = TextureAtlas::from_grid(
            self.sprite_controls_jam.clone(),
            Vec2::new(547., 547.),
            5,
            1,
        );
        self.sprite_controls_jam_atlas = texture_atlas_assets.add(texture_atlas);

        let texture_atlas = TextureAtlas::from_grid(
            self.sprite_upgrades_ability_bg.clone(),
            Vec2::new(1115., 484.),
            1,
            3,
        );
        self.sprite_upgrades_ability_bg_atlas = texture_atlas_assets.add(texture_atlas);

        let texture_atlas = TextureAtlas::from_grid(
            self.sprite_upgrades_button.clone(),
            Vec2::new(132., 111.),
            4,
            1,
        );
        self.sprite_upgrades_button_atlas = texture_atlas_assets.add(texture_atlas);
    }

    pub fn create_sound_effects(&mut self) {
        self.sound_effects = sound_effects_create(&self);
    }
}
