use self::args::SimulationKind;

use std::error::Error;

use clap::Parser;

mod args;
mod printer;

fn main() -> Result<(), Box<dyn Error>> {
    let cmd = SimulationKind::parse();
    cmd.run()?;
    Ok(())
}
