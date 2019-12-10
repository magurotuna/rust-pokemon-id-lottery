# rust-pokemon-id-lottery

## Usege

```
USAGE:
    rust-pokemon-id-lottery [FLAGS] [OPTIONS]

FLAGS:
        --dry-run    Dry run mode. If set, results will not be output to file.
    -h, --help       Prints help information
    -p, --print      Print results to stdout.
    -V, --version    Prints version information

OPTIONS:
    -b, --begin <begin_from>           Simulation begins from this number. [default: 1]
    -e, --end <end_at>                 Simulation finishes at this number. [default: 960]
    -t, --trial <num_trials>           The number of trials per one simulation. [default: 10000]
    -o, --output <output_file_path>    File that results output to. [default: result.csv]
    -s, --step <step_by>               At this step simulation will be executed. e.g. If this value is set to 300, three
                                       simulation as follows will be done: 1) You have 300 pokemons, 2) 600 pokemons, 3)
                                       900 pokemons [default: 20]
```

## Sample simulation

```
rust-pokemon-id-lottery on  master [!] is 📦 v0.1.0 via 🦀 v1.39.0 took 2s
❯ cargo run --release -- -s 1 -t 1000000 --output result_example.csv                                                                                          2685ms  火 12/10 23:51:35 2019
    Finished release [optimized] target(s) in 0.04s
     Running `target/release/rust-pokemon-id-lottery -s 1 -t 1000000 --output result_example.csv`

rust-pokemon-id-lottery on  master [!?] is 📦 v0.1.0 via 🦀 v1.39.0 took 6h18m36s
❯                                                                                                                                                              6.31h  水 12/11 06:12:07 2019
```

Execution time was 6h31m in my MBP on the following condition:

|   OPTION   |               VALUE |
| :--------: | ------------------: |
|  step_by   |                   1 |
| num_trials |           1,000,000 |
| begin_from |                   1 |
|   end_at   | 960 (=Box max size) |

You can see a sample output in `result_example.csv`.
