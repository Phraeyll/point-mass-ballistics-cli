use cli::args::options::Options;

use std::{stringify, time::Instant};

use point_mass_ballistics::{radian, Angle, Error};
use structopt::StructOpt;

macro_rules! time {
    ($expr:expr) => {{
        let start = Instant::now();
        match $expr {
            tmp => {
                println!("'{}': {:#?}", stringify!($expr), start.elapsed());
                tmp
            }
        }
    }};
}

fn main() -> Result<(), Error> {
    let opt = time!(Options::from_args());
    let mut angles = (Angle::new::<radian>(0.0), Angle::new::<radian>(0.0));
    if !opt.flags().flat() {
        let zero_builder = time!(opt.shared_params()?);
        let zero_simulation = time!(opt.zero_scenario(zero_builder)?);
        angles = time!(opt.try_zero(zero_simulation)?);
    };
    let firing_builder = time!(opt.shared_params()?);
    let firing_simulation = time!(opt.firing_scenario(firing_builder, angles.0, angles.1)?);
    time!(opt.print(&firing_simulation));
    Ok(())
}
