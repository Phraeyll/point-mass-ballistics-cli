use cli::args::options::Options;

use std::time::Instant;

use point_mass_ballistics::{radian, Angle, Error};
use structopt::StructOpt;

macro_rules! time {
    ($name:expr, {$($stmt:stmt)+}) => {
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
        let mut angles = (Angle::new::<radian>(0.0), Angle::new::<radian>(0.0));
        if !opt.flags().flat() {
            time!("Zeroing Simulation", {
                let zero_builder = opt.shared_params()?;
                let zero_simulation = opt.zero_scenario(zero_builder)?;
                angles = opt.try_zero(zero_simulation)?;
            });
        }
        time!("Firing Simulation", {
            let firing_builder = opt.shared_params()?;
            let firing_simulation = opt.firing_scenario(firing_builder, angles.0, angles.1)?;
            opt.print(&firing_simulation);
        });
    });
    Ok(())
}
