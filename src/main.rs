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
    #[clap(short = "t", long = "trial", default_value = "10000")]
    num_trials: usize,
    #[clap(short = "s", long = "step", default_value = "20")]
    step_by: usize,
    #[clap(short = "p", long = "print")]
    print: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    let seed: [u8; 32] = [42; 32];
    let mut rng = rng::make_rng(Some(seed));

    let pokemon_ids_base = shuffle::get_shuffled_ids(&mut rng);

    let results = simulate::exec_simulation(opts.step_by, &pokemon_ids_base, opts.num_trials);

    if opts.print {
        for r in &results {
            r.show();
            println!("\n---------------------------------------\n");
        }
    } else {
        println!("pokemon_num,first,second,third,fourth,fifth,nothing");
        for r in &results {
            r.to_csv();
        }
    }
}
