use build::*;
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

    let flat_builder = base(&args).with_zero_conditions(&args);
    let flat = flat_builder.init_with(
        Bc::with(
            args.value_of("bc").unwrap_or("0.305").parse().unwrap(),
            match args.value_of("bc-type").unwrap_or("g7") {
                "G1" | "g1" => G1,
                "G2" | "g2" => G2,
                "G5" | "g5" => G5,
                "G6" | "g6" => G6,
                "G7" | "g7" => G7,
                "G8" | "g8" => G8,
                "GI" | "gi" => GI,
                "GS" | "gs" => GS,
                _ => {
                    panic!("bc-type invalid - please use a valid variant <G1 G2 G5 G6 G7 G8 GI GS>")
                }
            },
        )
        .expect("bc + bc-type"),
    );
    let solved_before_conditions = flat
        .zero(
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
        )
        .expect("zero_err");
    let solved_builder = SimulationBuilder::from(solved_before_conditions)
        .with_conditions(&args)
        .increment_by(&args);
    let solved = Simulation::from(solved_builder);

    let table = solved.table(
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
