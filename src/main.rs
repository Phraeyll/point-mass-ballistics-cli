use crate::build::*;
use point_mass_ballistics::{Bc, BcKind::*, Error, Measurements, Natural, Numeric, Simulation};
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
    let bc = Bc::new(
        args.value_of("bc").unwrap_or("0.305").parse().unwrap(),
        match args
            .value_of("bc-type")
            .unwrap_or("G7")
            .to_ascii_uppercase()
            .as_ref()
        {
            "G1" => G1,
            "G2" => G2,
            "G5" => G5,
            "G6" => G6,
            "G7" => G7,
            "G8" => G8,
            "GI" => GI,
            "GS" => GS,
            _ => panic!("Invalid BC Type"),
        },
    )?;

    let mut builder = sim_before_zero(&args)?;
    builder = if args.is_present("flat") {
        builder
    } else {
        let builder_before = sim_before_zero(&args)?;
        let mut zero_simulation = builder_before.init_with_bc(&bc)?;
        let (pitch, yaw) = try_zero_simulation(&args, &mut zero_simulation)?;
        sim_after_zero(&args, builder, pitch, yaw)?
    };

    let simulation = builder.init_with_bc(&bc)?;

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
