use clap::ArgMatches;

use point_mass_ballistics::model::builder::*;

pub fn from_args(args: &ArgMatches) -> SimulationBuilder {
    SimulationBuilder::default()
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
                .set_velocity(
                    args.value_of("velocity").unwrap_or("2710").parse().unwrap())
                    .expect("velocity")
                .set_grains(
                    args.value_of("grains").unwrap_or("140").parse().unwrap())
                    .expect("grains")
                .set_caliber(
                    args.value_of("caliber").unwrap_or("0.264").parse().unwrap())
                    .expect("caliber")
                .set_bc(
                    Bc::new(
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
                .set_height(
                    args.value_of("scope-height").unwrap_or("1.5").parse().unwrap())
                .set_offset(
                    args.value_of("scope-offset").unwrap_or("0.0").parse().unwrap())
        )
        .conditions(
            Conditions::new()
                .set_temperature(
                    args.value_of("temperature").unwrap_or("68").parse().unwrap())
                    .expect("temperature")
                .set_pressure(
                    args.value_of("pressure").unwrap_or("29.92").parse().unwrap())
                    .expect("pressure")
                .set_humidity(
                    args.value_of("humidity").unwrap_or("0").parse().unwrap())
                    .expect("humidity")
                .set_wind_speed(
                    args.value_of("wind-speed").unwrap_or("0").parse().unwrap())
                    .expect("wind-speed")
                .set_wind_angle(
                    args.value_of("wind-angle").unwrap_or("0").parse().unwrap())
                    .expect("wind-angle")
                .set_shot_angle(
                    args.value_of("shot-angle").unwrap_or("0").parse().unwrap())
                    .expect("shot-angle")
                .set_lattitude(
                    args.value_of("lattitude").unwrap_or("0").parse().unwrap())
                    .expect("lattitude")
                .set_bearing(
                    args.value_of("bearing").unwrap_or("0").parse().unwrap())
                    .expect("bearing")
                .set_gravity(
                    args.value_of("gravity").unwrap_or("-32.174").parse().unwrap())
        )
}
