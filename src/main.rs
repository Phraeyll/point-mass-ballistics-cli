mod build;
mod cli;
mod printer;

use printer::{plain, pretty};

fn main() {
    let app = cli::parse().get_matches();
    let pretty = app.is_present("pretty");

    let builder = build::from_args(&app);

    // let simulation = builder.flat(0.0, 0.0);
    let simulation = builder.solve_for(
        app.value_of("zero-distance").unwrap().parse().unwrap(),
        app.value_of("zero-offset").unwrap().parse().unwrap(),
        app.value_of("zero-tolerance").unwrap().parse().unwrap(),
        app.value_of("pitch-offset").unwrap().parse().unwrap(),
        app.value_of("yaw-offset").unwrap().parse().unwrap(),
    );

    let table = simulation.table(
        app.value_of("table-step").unwrap().parse().unwrap(),
        app.value_of("table-start").unwrap().parse().unwrap(),
        app.value_of("table-end").unwrap().parse().unwrap(),
    );

    let output_tolerance = app.value_of("table-tolerance").unwrap().parse().unwrap();

    if pretty {
        pretty::print(table, output_tolerance);
    } else {
        plain::print(table, output_tolerance);
    }
}
