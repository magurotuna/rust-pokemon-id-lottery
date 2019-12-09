use indicatif::ProgressBar;

mod award;
mod result;
mod rng;
mod shuffle;
mod simulate;

#[macro_use]
extern crate clap;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "maguro_tuna")]
struct Opts {
    #[clap(short = "t", long = "trial", default_value = "100000")]
    num_trials: usize,
    #[clap(short = "s", long = "step", default_value = "5")]
    step_by: usize,
}

fn main() {
    let opts: Opts = Opts::parse();

    let seed: [u8; 32] = [42; 32];
    let mut rng = rng::make_rng(Some(seed));

    let pokemon_ids_base = shuffle::get_shuffled_ids(&mut rng);

    let results =
        simulate::exec_simulation(opts.step_by, &pokemon_ids_base, opts.num_trials, &mut rng);

    for r in &results {
        r.show();
    }
}
