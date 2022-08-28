use crate::game::prelude::*;

#[derive(Clone, Debug)]
pub struct GameState {
    pub town: TownData,
    pub band_members: [BandMember; 2],
    pub band_unlocked_count: usize,
    pub showed_example_text: bool,
    pub quests: Quests,
    pub dangerous_seas: bool,
    pub attacks: Attacks,
    pub checkpoint_notification: bool,
    pub health: f32,
    pub health_max: f32,
    pub defense: u32,
    pub experience: f32,
    pub level: u32,
    pub skill_points: u32,

    pub checkpoint: Option<Box<GameState>>,
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
            attacks: Attacks {
                forward_cannons: 1,
                shotgun_cannons: 0,
                shockwave: 0,
                bombs: 0,
                kraken: 0,
            },
            health: 20.,
            health_max: 20.,
            defense: 1,
            experience: 0.,
            level: 0,
            skill_points: 0,
            checkpoint_notification: false,
            checkpoint: None,
        }
    }
}

impl GameState {
    pub fn checkpoint(&mut self) {
        self.checkpoint_notification = true;
        self.checkpoint = Some(Box::new(self.clone()));
    }

    pub fn restore_checkpoint(&mut self) -> bool {
        if let Some(checkpoint) = self.checkpoint.take() {
            *self = *checkpoint.clone();
            self.checkpoint = Some(checkpoint);
            self.checkpoint_notification = false;
            true
        } else {
            false
        }
    }

    pub fn member_in_band(&self, band_member: BandMember) -> bool {
        for i in 0..2 {
            if self.band_members[i] == band_member {
                return true;
            }
        }
        false
    }

    pub fn add_experience(&mut self, amt: f32) -> bool {
        self.experience += amt;
        if self.experience >= self.experience_max() {
            self.experience -= self.experience_max();
            self.level += 1;
            true
        } else {
            false
        }
    }

    pub fn experience_max(&self) -> f32 {
        25. + self.level as f32 * 10.
    }

    pub fn apply_defense_upgrade(&mut self) {
        self.health *= 1.2;
        self.health_max *= 1.2;
        self.defense += 1;
    }
}
