use rand::prelude::*;
use rand::rngs::StdRng;

pub fn get_shuffled_ids(rng: &mut StdRng) -> Vec<u32> {
    let mut pokemon_ids_base = (0u32..1_000_000).collect::<Vec<_>>();
    pokemon_ids_base.shuffle(rng);
    pokemon_ids_base
}

#[test]
fn test_get_shuffled_ids() {
    let mut rng = StdRng::from_seed([0u8; 32]);

    assert_eq!(
        get_shuffled_ids(&mut rng).get(0..3),
        Some(&[159668, 343287, 402966][..])
    );
}
