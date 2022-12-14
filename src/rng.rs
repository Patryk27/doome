use bevy::prelude::*;
use rand::distributions::{Distribution, Standard};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

#[cfg(not(target_arch = "wasm32"))]
const SEED: [u8; 32] = *b"Hail Tynan Sylvester the RNG God";

#[cfg(target_arch = "wasm32")]
const SEED: [u8; 16] = *b"Hail Tynan Sylve";

pub struct RngPlugin;

impl Plugin for RngPlugin {
    fn build(&self, app: &mut App) {
        let rng = SmallRng::from_seed(SEED);

        app.insert_resource(RngState { rng });
    }
}

#[derive(Resource)]
pub struct RngState {
    pub rng: SmallRng,
}

impl RngState {
    pub fn gen<T>(&mut self) -> T
    where
        Standard: Distribution<T>,
    {
        self.rng.gen()
    }
}
