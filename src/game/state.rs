use crate::common::prelude::*;
use crate::game::prelude::*;
use audio_plus::prelude::*;

pub struct GameState {
    pub town: TownData,
    pub band_members: [BandMember; 2],
    pub band_unlocked_count: usize,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            town: TownData::default(),
            band_members: [BandMember::from_index(0), BandMember::from_index(1)],
            band_unlocked_count: 3,
        }
    }
}

impl GameState {
    pub fn member_in_band(&self, band_member: BandMember) -> bool {
        for i in 0..2 {
            if self.band_members[i] == band_member {
                return true;
            }
        }
        false
    }

    pub fn band_attack_type(&self) -> Attack {
        let (band_member_a, band_member_b) =
            if self.band_members[0].index() < self.band_members[1].index() {
                (self.band_members[0], self.band_members[1])
            } else {
                (self.band_members[1], self.band_members[0])
            };
        match band_member_a {
            BandMember::Guitar => match band_member_b {
                BandMember::Drums => Attack::ShotgunCannons,
                BandMember::Flute => Attack::DashAttack,
                BandMember::Accordion => unimplemented!(),
                BandMember::Harmonica => unimplemented!(),
                _ => unreachable!(),
            },
            BandMember::Drums => match band_member_b {
                BandMember::Flute => Attack::Shockwave,
                BandMember::Accordion => unimplemented!(),
                BandMember::Harmonica => unimplemented!(),
                _ => unreachable!(),
            },
            BandMember::Flute => match band_member_b {
                BandMember::Accordion => unimplemented!(),
                BandMember::Harmonica => unimplemented!(),
                _ => unreachable!(),
            },
            BandMember::Accordion => match band_member_b {
                BandMember::Harmonica => unimplemented!(),
                _ => unreachable!(),
            },
            BandMember::Harmonica => {
                unreachable!();
            }
        }
    }

    pub fn sfx_jam(&self, asset_library: &AssetLibrary) -> AudioPlusSoundEffect {
        let (band_member_a, band_member_b) =
            if self.band_members[0].index() < self.band_members[1].index() {
                (self.band_members[0], self.band_members[1])
            } else {
                (self.band_members[1], self.band_members[0])
            };
        match band_member_a {
            BandMember::Guitar => match band_member_b {
                BandMember::Drums => asset_library.sound_effects.sfx_jam_guitar_drums.clone(),
                BandMember::Flute => asset_library.sound_effects.sfx_jam_guitar_flute.clone(),
                BandMember::Accordion => unimplemented!(),
                BandMember::Harmonica => unimplemented!(),
                _ => unreachable!(),
            },
            BandMember::Drums => match band_member_b {
                BandMember::Flute => asset_library.sound_effects.sfx_jam_drums_flute.clone(),
                BandMember::Accordion => unimplemented!(),
                BandMember::Harmonica => unimplemented!(),
                _ => unreachable!(),
            },
            BandMember::Flute => match band_member_b {
                BandMember::Accordion => unimplemented!(),
                BandMember::Harmonica => unimplemented!(),
                _ => unreachable!(),
            },
            BandMember::Accordion => match band_member_b {
                BandMember::Harmonica => unimplemented!(),
                _ => unreachable!(),
            },
            BandMember::Harmonica => {
                unreachable!();
            }
        }
    }
}
