use cli::args::options::Options;

use std::{stringify, time::Instant};

use point_mass_ballistics::{radian, Angle, Error};
use structopt::StructOpt;

macro_rules! time {
    ($expr:expr) => {{
        let start = Instant::now();
        match $expr {
            tmp => {
                println!("Finished '{}' in: {:#?}", stringify!($expr), start.elapsed());
                tmp
            }
        }
    }};
}

fn main() -> Result<(), Error> {
    time!({
        let opt = time!(Options::from_args());
        let mut angles = (Angle::new::<radian>(0.0), Angle::new::<radian>(0.0));
        if !opt.flags().flat() {
            time!({
                let zero_builder = opt.shared_params()?;
                let zero_simulation = opt.zero_scenario(zero_builder)?;
                angles = opt.try_zero(zero_simulation)?;
            });
        }
        time!({
            let firing_builder = opt.shared_params()?;
            let firing_simulation = opt.firing_scenario(firing_builder, angles.0, angles.1)?;
            opt.print(&firing_simulation);
        });
    });
    Ok(())
}
