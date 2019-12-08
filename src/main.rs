extern crate indicatif;
extern crate rand;

use indicatif::ProgressBar;
use rand::prelude::*;
use std::collections::BTreeMap;
use std::fmt;

const BOX_MAX: u32 = 960;
const SIMULATION_MAX: u64 = 100;

fn main() {
    let mut pokemon_ids_base = (0u32..1_000_000).collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    pokemon_ids_base.shuffle(&mut rng);

    let pb_pokemon_num = ProgressBar::new(BOX_MAX as u64);

    let mut results = Vec::with_capacity(BOX_MAX as usize);

    // TODO parallel
    for pokemon_num in 900..=BOX_MAX as usize {
        let pokemon_ids: &[u32] = &pokemon_ids_base[..pokemon_num];
        let pb_day_num = ProgressBar::new(SIMULATION_MAX);
        let mut sim_result = SimulationResult::default();
        for _ in 0..SIMULATION_MAX {
            let rnd_number: u32 = rng.gen_range(0, 100_000);
            let result = simulate_oneday(pokemon_ids, rnd_number);
            sim_result.add_count(result);
            pb_day_num.inc(1);
        }
        results.push(sim_result);
        pb_pokemon_num.inc(1);
    }

    for r in &results {
        r.show();
    }
}

#[derive(Debug, Default)]
struct SimulationResult {
    award_count: BTreeMap<Award, u32>,
    simulation_count: u32,
}

impl SimulationResult {
    fn add_count(&mut self, award: Award) {
        self.simulation_count += 1;
        let entry = self.award_count.entry(award).or_insert(0);
        *entry += 1;
    }

    fn show(&self) {
        println!("The number of simulations: {}\n", self.simulation_count);
        for (k, &v) in &self.award_count {
            println!(
                "{}\t{:>3}\t{:5.2}%",
                k,
                v,
                (v as f64 / self.simulation_count as f64) * 100.1
            );
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Award {
    First = 5,  // master ball
    Second = 4, // rare candy
    Third = 3,  // pp max
    Fourth = 2, // pp up
    Fifth = 1,  // moomoo milk
    Losing = 0, // lose lottery...
}

impl fmt::Display for Award {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<11}",
            match self {
                Award::First => "Master Ball",
                Award::Second => "Rare Candy",
                Award::Third => "PP Max",
                Award::Fourth => "PP Up",
                Award::Fifth => "Moomoo Milk",
                Award::Losing => "Nothing",
            }
        )
    }
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
