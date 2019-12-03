use cli::args::options::SimulationKind;

use std::error::Error;

use structopt::StructOpt;

fn main() -> Result<(), Box<dyn Error>> {
    let cmd = SimulationKind::from_args();
    cmd.run()?;
    Ok(())
}
