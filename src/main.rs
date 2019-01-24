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
    let chosen = if args.is_present("flat") {
        simulation
    } else {
        try_zero_simulation(&args, &mut simulation)?;
        solution_after_zero(&args, simulation)?
    };

    let table = chosen.table(
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

trait Tabular
where
    Self: IntoIterator,
{
    type Collection: IntoIterator<Item = <Self as IntoIterator>::Item>;
    fn table(self, step: Natural, range_start: Natural, range_end: Natural) -> Self::Collection;
}
impl<'s> Tabular for &'s Simulation {
    type Collection = Vec<<Self as IntoIterator>::Item>;
    fn table(self, step: Natural, range_start: Natural, range_end: Natural) -> Self::Collection {
        let mut iter = self.into_iter().fuse();
        (range_start..=range_end)
            .step_by(step as usize)
            .filter_map(|current_step| {
                iter.by_ref()
                    .find(|p| p.distance() >= Numeric::from(current_step))
            })
            .collect::<Self::Collection>()
    }
}
