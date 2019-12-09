use rand::prelude::*;
use rand::rngs::StdRng;

type Seed = [u8; 32];

pub fn make_rng(seed: Option<Seed>) -> StdRng {
    match seed {
        Some(s) => StdRng::from_seed(s),
        None => StdRng::from_entropy(),
    }
}
