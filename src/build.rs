use clap::ArgMatches;

use rballistics_flat::model::point_mass::*;

pub fn from_args(args: &ArgMatches) -> Solver {
    Solver::new()
        .flags(
            Flags::new()
                .enable_coriolis(
                    !args.is_present("disable-coriolis"))
                .enable_gravity(
                    !args.is_present("disable-gravity"))
                .enable_drag(
                    !args.is_present("disable-drag"))
        )
        .time_step(
            args.value_of("time-step").unwrap_or("0.00005").parse().unwrap())
                .expect("time-step")
        .projectile(
            Projectile::new()
                .with_velocity(
                    args.value_of("velocity").unwrap_or("2710").parse().unwrap())
                    .expect("velocity")
                .with_grains(
                    args.value_of("grains").unwrap_or("140").parse().unwrap())
                    .expect("grains")
                .with_caliber(
                    args.value_of("caliber").unwrap_or("0.264").parse().unwrap())
                    .expect("caliber")
                .with_bc(
                    BallisticCoefficient::new(
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
                            _ => panic!("bc-type invalid - please use a valid variant <G1 G2 G5 G6 G7 G8 GI GS>"),
                        },
                    )
                    .expect("bc + bc-type")
                )
        )
        .scope(
            Scope::new()
                .with_height(
                    args.value_of("scope-height").unwrap_or("1.5").parse().unwrap())
                .with_offset(
                    args.value_of("scope-offset").unwrap_or("0.0").parse().unwrap())
        )
        .solve_conditions(
            Conditions::new()
                .with_temperature(
                    args.value_of("temperature").unwrap_or("68").parse().unwrap())
                    .expect("temperature")
                .with_pressure(
                    args.value_of("pressure").unwrap_or("29.92").parse().unwrap())
                    .expect("pressure")
                .with_humidity(
                    args.value_of("humidity").unwrap_or("0").parse().unwrap())
                    .expect("humidity")
                .with_wind_speed(
                    args.value_of("wind-speed").unwrap_or("0").parse().unwrap())
                    .expect("wind-speed")
                .with_wind_angle(
                    args.value_of("wind-angle").unwrap_or("0").parse().unwrap())
                    .expect("wind-angle")
                .with_shot_angle(
                    args.value_of("shot-angle").unwrap_or("0").parse().unwrap())
                    .expect("shot-angle")
                .with_lattitude(
                    args.value_of("lattitude").unwrap_or("0").parse().unwrap())
                    .expect("lattitude")
                .with_bearing(
                    args.value_of("bearing").unwrap_or("0").parse().unwrap())
                    .expect("bearing")
                .with_gravity(
                    args.value_of("gravity").unwrap_or("-32.174").parse().unwrap())
        )
        .zero_conditions(
            Conditions::new()
                .with_temperature(
                    args.value_of("zero-temperature").unwrap_or("68").parse().unwrap())
                    .expect("zero-temperature")
                .with_pressure(
                    args.value_of("zero-pressure").unwrap_or("29.92").parse().unwrap())
                    .expect("zero-pressure")
                .with_humidity(
                    args.value_of("zero-humidity").unwrap_or("0").parse().unwrap())
                    .expect("zero-humidity")
                .with_wind_speed(
                    args.value_of("zero-wind-speed").unwrap_or("0").parse().unwrap())
                    .expect("zero-wind-speed")
                .with_wind_angle(
                    args.value_of("zero-wind-angle").unwrap_or("0").parse().unwrap())
                    .expect("zero-wind-angle")
                .with_shot_angle(
                    args.value_of("zero-shot-angle").unwrap_or("0").parse().unwrap())
                    .expect("zero-shot-angle")
                .with_lattitude(
                    args.value_of("zero-lattitude").unwrap_or("0").parse().unwrap())
                    .expect("zero-lattitude")
                .with_bearing(
                    args.value_of("zero-bearing").unwrap_or("0").parse().unwrap())
                    .expect("zero-bearing")
                .with_gravity(
                    args.value_of("zero-gravity").unwrap_or("-32.174").parse().unwrap())
        )
}
