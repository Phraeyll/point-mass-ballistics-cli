use self::args::SimulationKind;

use std::error::Error;

use structopt::StructOpt;

mod args;
mod commands;
mod printer;

fn main() -> Result<(), Box<dyn Error>> {
    let cmd = SimulationKind::from_args();
    cmd.run()?;
    Ok(())
}
