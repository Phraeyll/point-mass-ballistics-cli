use clap::ArgMatches;
use rballistics_flat::model::point_mass::*;

pub fn from_args(app: &ArgMatches) -> SimulationBuilder {
    SimulationBuilder::new()
        .time_step(
            app.value_of("time-step").unwrap().parse().unwrap())
        .projectile(
            Projectile::new()
                .with_velocity(
                    app.value_of("velocity").unwrap().parse().unwrap())
                .with_grains(
                    app.value_of("grains").unwrap().parse().unwrap())
                .with_caliber(
                    app.value_of("caliber").unwrap().parse().unwrap())
                .with_bc(
                    BallisticCoefficient::new(
                        app.value_of("bc").unwrap().parse().unwrap(),
                        match app.value_of("bc-type").unwrap() {
                            "G1" | "g1" => G1,
                            "G2" | "g2" => G2,
                            "G5" | "g5" => G5,
                            "G6" | "g6" => G6,
                            "G7" | "g7" => G7,
                            "G8" | "g8" => G8,
                            "GI" | "gi" => GI,
                            "GS" | "gs" => GS,
                            _ => panic!("bc-type invalid - please use a valid variant <G1 G2 G5 G6 G7 G8 GI GS>"),
                        },
                    )
                )
        )
        .scope(
            Scope::new()
                .with_height(
                    app.value_of("scope-height").unwrap().parse().unwrap())
        )
        .solve_conditions(
            Conditions::new()
                .with_temperature(
                    app.value_of("temperature").unwrap().parse().unwrap())
                .with_pressure(
                    app.value_of("pressure").unwrap().parse().unwrap())
                .with_humidity(
                    app.value_of("humidity").unwrap().parse().unwrap())
                .with_wind_speed(
                    app.value_of("wind-speed").unwrap().parse().unwrap())
                .with_wind_angle(
                    app.value_of("wind-angle").unwrap().parse().unwrap())
                .with_shot_angle(
                    app.value_of("shot-angle").unwrap().parse().unwrap())
                .with_lattitude(
                    app.value_of("lattitude").unwrap().parse().unwrap())
                .with_bearing(
                    app.value_of("bearing").unwrap().parse().unwrap())
        )
        .zero_conditions(
            Conditions::new()
                .with_temperature(
                    app.value_of("zero-temperature").unwrap().parse().unwrap())
                .with_pressure(
                    app.value_of("zero-pressure").unwrap().parse().unwrap())
                .with_humidity(
                    app.value_of("zero-humidity").unwrap().parse().unwrap())
                .with_wind_speed(
                    app.value_of("zero-wind-speed").unwrap().parse().unwrap())
                .with_wind_angle(
                    app.value_of("zero-wind-angle").unwrap().parse().unwrap())
                .with_shot_angle(
                    app.value_of("zero-shot-angle").unwrap().parse().unwrap())
                .with_lattitude(
                    app.value_of("zero-lattitude").unwrap().parse().unwrap())
                .with_bearing(
                    app.value_of("zero-bearing").unwrap().parse().unwrap())
        )
}
