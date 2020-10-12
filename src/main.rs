use self::util::args::SimulationKind;

use std::error::Error;

use structopt::StructOpt;

mod util {
    pub mod args;
    pub mod commands;
    pub mod printer;
}

fn main() -> Result<(), Box<dyn Error>> {
    let cmd = SimulationKind::from_args();
    cmd.run()?;
    Ok(())
}
