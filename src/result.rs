use std::collections::BTreeMap;

use crate::award::Award;

#[derive(Debug, Default)]
pub struct SimulationResult {
    award_count: BTreeMap<Award, u32>,
    simulation_count: u32,
}

impl SimulationResult {
    pub fn add_count(&mut self, award: Award) {
        self.simulation_count += 1;
        let entry = self.award_count.entry(award).or_insert(0);
        *entry += 1;
    }

    pub fn show(&self) {
        println!("The number of simulations: {}\n", self.simulation_count);
        for (k, &v) in &self.award_count {
            println!(
                "{}\t{:>3}\t{:5.2}%",
                k,
                v,
                (v as f64 / self.simulation_count as f64) * 100.0
            );
        }
    }
}
