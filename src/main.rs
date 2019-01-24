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

    let flat_model_builder = flat_model_builder(&args);
    let flat_simulation = match flat_model_builder {
        Ok(result) => flat_simulation(&args, result),
        Err(err) => panic!(err),
    };
    let mut chosen = Simulation::from(if args.is_present("flat") {
        match flat_simulation {
            Ok(result) => SimulationBuilder::from(result),
            Err(err) => panic!(err),
        }
    } else {
        let zeroed_simulation = match flat_simulation {
            Ok(result) => zero_simulation(&args, result),
            Err(err) => panic!(err),
        };
        let solved_builder = match zeroed_simulation {
            Ok(result) => solution_builder(&args, result),
            Err(err) => panic!(err),
        };

        match solved_builder {
            Ok(result) => result,
            Err(err) => panic!(err),
        }
    });
    chosen.increment_scope_pitch(args.value_of("scope-pitch").unwrap_or("0").parse().unwrap());
    chosen.increment_scope_yaw(args.value_of("scope-yaw").unwrap_or("0").parse().unwrap());

    // let builder = flat_model_builder(&args);
    // let try_build = match builder {
    //     Ok(builder) => flat_simulation(&args, builder),
    //     Err(err) => panic!(err),
    // };
    // let try_zero = match try_build {
    //     Ok(built) => zero_simulation(&args, built),
    //     Err(err) => panic!(err),
    // };
    // let chosen = match try_zero {
    //     Ok(zeroed) => zeroed,
    //     Err(err) => panic!(err),
    // };


    // let flat = Simulation::from(test_builder());
    // let flat = Simulation::from(SimulationBuilder::default());
    // let chosen = match flat.zero(200.0, 0.0, 0.0, 0.001) {
    //     Ok(result) => result,
    //     Err(err) => panic!(err),
    // };
    // let mut chosen = Simulation::from(test_builder());
    // chosen.try_mut_zero(200.0, 0.0, 0.0, 0.001)?;

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
