use crate::game::prelude::*;

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
}
