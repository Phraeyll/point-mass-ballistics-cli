use point_mass_ballistics::model::*;
use printer::{plain, pretty};

mod build;
mod cli;
mod printer {
    mod helper;
    pub mod plain;
    pub mod pretty;
}

fn main() {
    let args = cli::parse().get_matches();

    let builder = build::from_args(&args);

    let simulation = if args.is_present("flat") {
        MySimulation(builder.using_zero_conditions(
            args.value_of("pitch").unwrap_or("0").parse().unwrap(),
            args.value_of("yaw").unwrap_or("0").parse().unwrap(),
        ))
    } else {
        MySimulation(builder.solve_for(
            args.value_of("zero-distance")
                .unwrap_or("200")
                .parse()
                .unwrap(),
            args.value_of("zero-height").unwrap_or("0").parse().unwrap(),
            args.value_of("zero-offset").unwrap_or("0").parse().unwrap(),
            args.value_of("zero-tolerance")
                .unwrap_or("0.001")
                .parse()
                .unwrap(),
            args.value_of("pitch").unwrap_or("0").parse().unwrap(),
            args.value_of("yaw").unwrap_or("0").parse().unwrap(),
        ))
    };

    let table = simulation.table(
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
}

struct MySimulation<'a>(Simulation<'a>);
impl MySimulation<'_> {
    fn table(
        &self,
        step: Natural,
        range_start: Natural,
        range_end: Natural,
    ) -> Vec<Packet<'_>> {
        let mut iter = self.0.into_iter().fuse();
        (range_start..=range_end)
            .step_by(step as usize)
            .filter_map(|current_step| {
                iter.by_ref()
                    .find(|p| p.distance() >= Numeric::from(current_step))
            })
            .collect::<Vec<_>>()
    }
}
