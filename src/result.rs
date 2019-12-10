use std::collections::BTreeMap;

use crate::award::Award;

#[derive(Debug)]
pub struct SimulationResult {
    pokemon_num: u32,
    award_count: BTreeMap<Award, u32>,
    simulation_count: u32,
}

impl SimulationResult {
    pub fn new(pokemon_num: u32) -> Self {
        let mut m = BTreeMap::new();
        m.insert(Award::First, 0);
        m.insert(Award::Second, 0);
        m.insert(Award::Third, 0);
        m.insert(Award::Fourth, 0);
        m.insert(Award::Fifth, 0);
        m.insert(Award::Losing, 0);
        Self {
            pokemon_num,
            award_count: m,
            simulation_count: 0,
        }
    }

    pub fn add_count(&mut self, award: Award) {
        self.simulation_count += 1;
        let val = self.award_count.get_mut(&award).unwrap();
        *val += 1;
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

    pub fn to_csv(&self) {
        let &first = self.award_count.get(&Award::First).unwrap();
        let &second = self.award_count.get(&Award::Second).unwrap();
        let &third = self.award_count.get(&Award::Third).unwrap();
        let &fourth = self.award_count.get(&Award::Fourth).unwrap();
        let &fifth = self.award_count.get(&Award::Fifth).unwrap();
        let &nothing = self.award_count.get(&Award::Losing).unwrap();

        let to_prob = |count: u32| (count as f64 / self.simulation_count as f64) * 100.0;
        println!(
            "{num},{first:.2},{second:.2},{third:.2},{fourth:.2},{fifth:.2},{nothing:.2}",
            num = self.pokemon_num,
            first = to_prob(first),
            second = to_prob(second),
            third = to_prob(third),
            fourth = to_prob(fourth),
            fifth = to_prob(fifth),
            nothing = to_prob(nothing),
        );
    }
}

#[test]
fn test_add_count() {
    let mut result = SimulationResult::new(100);
    assert_eq!(result.simulation_count, 0);

    result.add_count(Award::First);
    assert_eq!(result.award_count.get(&Award::First), Some(&1));
    result.add_count(Award::First);
    assert_eq!(result.award_count.get(&Award::First), Some(&2));

    assert_eq!(result.award_count.get(&Award::Second), Some(&0));
}
