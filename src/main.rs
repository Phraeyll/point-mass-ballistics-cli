use build::*;
use point_mass_ballistics::error::Error;
use point_mass_ballistics::model::*;
use printer::{plain, pretty};

mod build;
mod cli;
mod printer {
    mod helper;
    pub mod plain;
    pub mod pretty;
}

fn main() -> Result<(), Error> {
    let args = cli::parse().get_matches();

    let mut simulation = builder(&args)?;
    let simulation = if args.is_present("flat") {
        simulation
    } else {
        try_zero_simulation(&args, &mut simulation)?;
        solution_after_zero(&args, simulation)?
    };

    let table = table(
        &simulation,
        args.value_of("table-step")
            .unwrap_or("100")
            .parse()
            .unwrap(),
        args.value_of("table-start").unwrap_or("0").parse().unwrap(),
        args.value_of("table-end")
            .unwrap_or("1000")
            .parse()
            .unwrap(),
    );
    let output_tolerance = args
        .value_of("table-tolerance")
        .unwrap_or("0.005")
        .parse()
        .unwrap();
    if args.is_present("pretty") {
        pretty::print(table, output_tolerance);
    } else {
        plain::print(table, output_tolerance);
    }

    Ok(())
}

fn table<'s>(
    simulation: &'s Simulation,
    step: Natural,
    range_start: Natural,
    range_end: Natural,
) -> impl IntoIterator<Item = impl Measurements + 's> + 's {
    let mut iter = simulation.into_iter().fuse();
    (range_start..=range_end)
        .step_by(step as usize)
        .filter_map(move |current_step| {
            iter.by_ref()
                .find(|p| p.distance() >= Numeric::from(current_step))
        })
}
