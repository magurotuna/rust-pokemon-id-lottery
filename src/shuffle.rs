use rand::prelude::*;
use rand::rngs::StdRng;

pub fn get_shuffled_ids(rng: &mut StdRng) -> Vec<u32> {
    let mut pokemon_ids_base = (0u32..1_000_000).collect::<Vec<_>>();
    pokemon_ids_base.shuffle(rng);
    pokemon_ids_base
}
