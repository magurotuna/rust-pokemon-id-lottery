use indicatif::ParallelProgressIterator;
use rand::prelude::*;
use rayon::prelude::*;

use crate::award::Award;
use crate::result::SimulationResult;

const BOX_MAX: u32 = 960;

pub fn exec_simulation(
    step: usize,
    pokemon_ids_base: &[u32],
    num_trials: usize,
) -> Vec<SimulationResult> {
    (1..=BOX_MAX as usize)
        .into_par_iter()
        .progress()
        .filter(|pokemon_num| pokemon_num % step == 0)
        .map_init(
            || rand::thread_rng(),
            |rng, pokemon_num| {
                let pokemon_ids = &pokemon_ids_base[..pokemon_num];
                let mut sim_result = SimulationResult::new(pokemon_num as u32);
                for _ in 0..num_trials {
                    let rnd_number: u32 = rng.gen_range(0, 100_000);
                    let result = simulate_oneday(pokemon_ids, rnd_number);
                    sim_result.add_count(result);
                }
                sim_result
            },
        )
        .collect()
}

// TODO: test this function
fn simulate_oneday(ids: &[u32], number_this_day: u32) -> Award {
    let mut award = Award::Losing;
    let win_number = format!("{:05}", number_this_day);
    for id in ids {
        let id = format!("{:06}", id);
        if id.starts_with(&win_number) || id.ends_with(&win_number) {
            award = Award::First;
            break;
        } else if id.ends_with(&win_number[1..]) {
            award = Award::Second;
        } else if id.ends_with(&win_number[2..]) && award < Award::Third {
            award = Award::Third;
        } else if id.ends_with(&win_number[3..]) && award < Award::Fourth {
            award = Award::Fourth;
        } else if id.ends_with(&win_number[4..]) && award < Award::Fifth {
            award = Award::Fifth;
        }
    }
    award
}
