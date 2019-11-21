use cli::args::options::Options;

use std::time::Instant;

use point_mass_ballistics::Error;
use structopt::StructOpt;

macro_rules! time {
    ($name:expr, {$($stmt:stmt;)+}) => {
        let start = Instant::now();
        $(
            $stmt
        )+
        println!("Finished '{}' in: {:#?}", $name, start.elapsed());
    };
}

fn main() -> Result<(), Error> {
    time!("Entire Program", {
        time!("Parsing Options", {
            let opt = Options::from_args();
        });
        time!("Building Builders", {
            let zero_builder = opt.shared_params()?;
            let firing_builder = opt.shared_params()?;
        });
        time!("Zeroing Simulation", {
            let zero_simulation = opt.zero_scenario(zero_builder)?;
            let (pitch, yaw) = opt.try_zero(zero_simulation)?;
        });
        time!("Firing Simulation", {
            let firing_simulation = opt.firing_scenario(firing_builder, pitch, yaw)?;
            opt.print(&firing_simulation);
        });
    });
    Ok(())
}
