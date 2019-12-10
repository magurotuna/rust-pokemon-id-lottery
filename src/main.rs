mod award;
mod output;
mod result;
mod rng;
mod shuffle;
mod simulate;

#[macro_use]
extern crate clap;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "maguro_tuna")]
struct Opts {
    /// The number of trials per one simulation.
    #[clap(short = "t", long = "trial", default_value = "10000")]
    num_trials: usize,

    /// At this step simulation will be executed.
    /// e.g. If this value is set to 300, three simulation as follows will be done: 1) You have 300 pokemons, 2) 600 pokemons, 3) 900 pokemons
    #[clap(short = "s", long = "step", default_value = "20")]
    step_by: usize,

    /// File that results output to.
    #[clap(short = "o", long = "output", default_value = "result.csv")]
    output_file_path: String,

    /// Print results to stdout.
    #[clap(short = "p", long = "print")]
    print: bool,

    /// Dry run mode. If set, results will not be output to file.
    #[clap(long = "dry-run")]
    dry_run: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    let seed: [u8; 32] = [42; 32];
    let mut rng = rng::make_rng(Some(seed));

    let pokemon_ids_base = shuffle::get_shuffled_ids(&mut rng);

    let results = simulate::exec_simulation(opts.step_by, &pokemon_ids_base, opts.num_trials);

    if opts.print {
        output::print_stdout(&results);
    }

    if !opts.dry_run {
        output::to_csv(&opts.output_file_path, &results)?;
    }

    Ok(())
}
