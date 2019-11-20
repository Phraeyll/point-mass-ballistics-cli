use crate::cli::Options;

use point_mass_ballistics::Error;
use structopt::StructOpt;

// mod build;
mod cli;
mod printer {
    mod helper;
    pub mod plain;
    pub mod pretty;
}

fn main() -> Result<(), Error> {
    let opt = Options::from_args();
    let zero_builder = opt.shared_params()?;
    let firing_builder = opt.shared_params()?;

    let zero_simulation = opt.zero_scenario(zero_builder)?;
    let (pitch, yaw) = opt.try_zero(zero_simulation)?;
    dbg!((pitch, yaw));
    let firing_simulation = opt.firing_scenario(firing_builder, pitch, yaw)?;

    opt.print(&firing_simulation);

    Ok(())
}
