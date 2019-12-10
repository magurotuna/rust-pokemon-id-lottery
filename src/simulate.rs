use indicatif::ParallelProgressIterator;
use rand::prelude::*;
use rayon::prelude::*;

use crate::award::Award;
use crate::result::SimulationResult;

pub fn exec_simulation(
    start: usize,
    end: usize,
    step: usize,
    pokemon_ids_base: &[u32],
    num_trials: usize,
) -> Vec<SimulationResult> {
    (start..=end)
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

#[test]
fn test_simulate_oneday() {
    let ids = vec![0u32, 111111, 222222, 345678];
    // 5 digits match
    assert_eq!(simulate_oneday(&ids, 0), Award::First);
    assert_eq!(simulate_oneday(&ids, 11111), Award::First);
    assert_eq!(simulate_oneday(&ids, 22222), Award::First);
    assert_eq!(simulate_oneday(&ids, 34567), Award::First);
    assert_eq!(simulate_oneday(&ids, 45678), Award::First);

    // last 4 digits match
    assert_eq!(simulate_oneday(&ids, 1111), Award::Second);
    assert_eq!(simulate_oneday(&ids, 2222), Award::Second);
    assert_eq!(simulate_oneday(&ids, 91111), Award::Second);
    assert_eq!(simulate_oneday(&ids, 92222), Award::Second);
    assert_eq!(simulate_oneday(&ids, 95678), Award::Second);

    // last 3 digits match
    assert_eq!(simulate_oneday(&ids, 111), Award::Third);
    assert_eq!(simulate_oneday(&ids, 222), Award::Third);
    assert_eq!(simulate_oneday(&ids, 99111), Award::Third);
    assert_eq!(simulate_oneday(&ids, 99222), Award::Third);
    assert_eq!(simulate_oneday(&ids, 99678), Award::Third);

    // last 2 digits match
    assert_eq!(simulate_oneday(&ids, 11), Award::Fourth);
    assert_eq!(simulate_oneday(&ids, 22), Award::Fourth);
    assert_eq!(simulate_oneday(&ids, 99911), Award::Fourth);
    assert_eq!(simulate_oneday(&ids, 99922), Award::Fourth);
    assert_eq!(simulate_oneday(&ids, 99978), Award::Fourth);

    // last 1 digit matches
    assert_eq!(simulate_oneday(&ids, 1), Award::Fifth);
    assert_eq!(simulate_oneday(&ids, 2), Award::Fifth);
    assert_eq!(simulate_oneday(&ids, 99991), Award::Fifth);
    assert_eq!(simulate_oneday(&ids, 99992), Award::Fifth);
    assert_eq!(simulate_oneday(&ids, 99998), Award::Fifth);

    // no digit matches
    assert_eq!(simulate_oneday(&ids, 11119), Award::Losing);
    assert_eq!(simulate_oneday(&ids, 22229), Award::Losing);
    assert_eq!(simulate_oneday(&ids, 99999), Award::Losing);
}
