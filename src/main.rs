use crate::build::*;
use point_mass_ballistics::{
    inch, yard, Error, Length, Measurements, Natural, Numeric, Simulation,
};
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

    let mut builder = sim_before_zero(&args)?;
    builder = if args.is_present("flat") {
        builder
    } else {
        let builder_before = sim_before_zero(&args)?;
        let mut zero_simulation = builder_before.init();
        let (pitch, yaw) = try_zero_simulation(&args, &mut zero_simulation)?;
        sim_after_zero(&args, builder, pitch, yaw)?
    };

    let simulation = builder.init();

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
    let output_tolerance = Length::new::<inch>(
        args.value_of("table-tolerance")
            .unwrap_or("0.005")
            .parse()
            .unwrap(),
    );
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
                .find(|p| p.distance() >= Length::new::<yard>(Numeric::from(current_step)))
        })
}
