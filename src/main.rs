use printer::{plain, pretty};
use rballistics_flat::model::point_mass::builder::SimulationBuilder;

mod build;
mod cli;
mod printer;

fn main() {
    let args = cli::parse().get_matches();

    let builder = build::from_args(&args);

    let simulation = if args.is_present("flat") {
        builder.using_zero_conditions(
            args.value_of("pitch-offset").unwrap().parse().unwrap(),
            args.value_of("yaw-offset").unwrap().parse().unwrap(),
        )
    } else {
        builder.solve_for(
            args.value_of("zero-distance").unwrap().parse().unwrap(),
            args.value_of("zero-elevation-offset")
                .unwrap()
                .parse()
                .unwrap(),
            args.value_of("zero-windage-offset")
                .unwrap()
                .parse()
                .unwrap(),
            args.value_of("zero-tolerance").unwrap().parse().unwrap(),
            args.value_of("pitch-offset").unwrap().parse().unwrap(),
            args.value_of("yaw-offset").unwrap().parse().unwrap(),
        )
    };

    let table = simulation.table(
        args.value_of("table-step").unwrap().parse().unwrap(),
        args.value_of("table-start").unwrap().parse().unwrap(),
        args.value_of("table-end").unwrap().parse().unwrap(),
    );

    let output_tolerance = args.value_of("table-tolerance").unwrap().parse().unwrap();
    if args.is_present("pretty") {
        pretty::print(table, output_tolerance);
    } else {
        plain::print(table, output_tolerance);
    }
}
