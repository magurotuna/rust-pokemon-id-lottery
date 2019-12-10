use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufWriter};
use std::path::Path;

use crate::result::SimulationResult;

pub fn print_stdout(results: &[SimulationResult]) {
    for r in results {
        r.show();
        println!("\n---------------------------------------\n");
    }
}

pub fn to_csv<P>(path: P, results: &[SimulationResult]) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writeln!(
        writer,
        "pokemon_num,first,second,third,fourth,fifth,nothing"
    )?;
    for r in results {
        writeln!(writer, "{}", r.to_csv())?;
    }

    writer.flush()?;
    Ok(())
}
