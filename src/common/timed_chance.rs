use rand::prelude::*;
use rand::{rngs::StdRng, SeedableRng};

#[derive(Default)]
pub struct TimedChance {
    time: f32,
    seed: [u8; 32],
}

impl TimedChance {
    pub fn new() -> Self {
        Self {
            time: 0.,
            seed: Default::default(),
        }
    }

    pub fn check(&mut self, seconds_per: f32, variance: f32, dt: f32) -> bool {
        let mut rng = StdRng::from_seed(self.seed);
        let rand_variance = rng.gen::<f32>() * variance * 2. - variance;
        self.time += dt;
        if self.time > seconds_per + rand_variance {
            self.time = 0.;
            self.seed = rng.gen::<[u8; 32]>();
            true
        } else {
            false
        }
    }
}
