extern crate indicatif;
extern crate rand;

use rand::prelude::*;
use std::collections::BTreeMap;
use std::fmt;

fn main() {
    let pokemon_num = 500;

    let mut pokemon_ids_base = (0u32..1_000_000).collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    pokemon_ids_base.shuffle(&mut rng);

    let pokemon_ids: &[u32] = &pokemon_ids_base[..pokemon_num];
    println!("{:?}", pokemon_ids);

    let simulation_max = 10000;
    let pb = indicatif::ProgressBar::new(simulation_max);
    let mut sim_result = SimulationResult::default();
    for _ in 0..simulation_max {
        let rnd_number: u32 = rng.gen_range(0, 100_000);
        let result = simulate_oneday(pokemon_ids, rnd_number);
        sim_result.add_count(result);
        pb.inc(1);
    }
    sim_result.show();
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
                "{}\t{}\t{:.2}%",
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
            "{}",
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
