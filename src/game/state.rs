use crate::game::prelude::*;

pub struct GameState {
    pub town: TownData,
    pub band_members: [BandMember; 2],
    pub band_unlocked_count: usize,
    pub showed_example_text: bool,
    pub quests: Quests,
    pub dangerous_seas: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            town: TownData::default(),
            band_members: [BandMember::from_index(0), BandMember::from_index(1)],
            band_unlocked_count: 3,
            showed_example_text: false,
            quests: Quests::default(),
            dangerous_seas: false,
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

    pub fn band_special_attack_type(&self) -> SpecialAttack {
        let (band_member_a, band_member_b) =
            if self.band_members[0].index() < self.band_members[1].index() {
                (self.band_members[0], self.band_members[1])
            } else {
                (self.band_members[1], self.band_members[0])
            };
        match band_member_a {
            BandMember::Guitar => match band_member_b {
                BandMember::Drums => SpecialAttack::ShotgunCannons,
                BandMember::Flute => SpecialAttack::DashAttack,
                BandMember::Accordion => unimplemented!(),
                BandMember::Harmonica => unimplemented!(),
                _ => unreachable!(),
            },
            BandMember::Drums => match band_member_b {
                BandMember::Flute => SpecialAttack::Shockwave,
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
