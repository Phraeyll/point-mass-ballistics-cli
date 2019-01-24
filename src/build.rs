use clap::ArgMatches;

use point_mass_ballistics::error::Result;
use point_mass_ballistics::model::core::*;

pub fn test_simulation(args: &ArgMatches) -> Simulation {
    Simulation::from(SimulationBuilder {
        flags: FlagsBuilder {
            coriolis: true,
            drag: true,
            gravity: true,
        },
        projectile: ProjectileBuilder {
            weight: WeightMass::Grains(140.0),
            caliber: Length::Inches(0.264),
            bc: set_bc(0.305, G7),
            velocity: Velocity::Fps(2710.0),
        },
        scope: ScopeBuilder {
            yaw: Angle::Minutes(0.0),
            pitch: Angle::Minutes(0.0),
            roll: Angle::Degrees(0.0),
            height: Length::Inches(1.5),
            offset: Length::Inches(0.0),
        },
        atmosphere: AtmosphereBuilder {
            temperature: Temperature::F(68.0),
            pressure: Pressure::Inhg(29.92),
            humidity: 0.0,
        },
        wind: WindBuilder {
            yaw: Angle::Degrees(0.0),
            pitch: Angle::Degrees(0.0),
            roll: Angle::Degrees(0.0),
            velocity: Velocity::Mph(0.0),
        },
        shooter: ShooterBuilder {
            yaw: Angle::Minutes(0.0),
            pitch: Angle::Minutes(0.0),
            roll: Angle::Degrees(0.0),
            lattitude: Angle::Degrees(0.0),
            gravity: default_gravity(),
        },
        time_step: 0.000_01,
    })
}

pub fn flat_model_builder(args: &ArgMatches) -> Result<SimulationBuilder> {
    Ok(SimulationBuilder::default()
        .time_step(
            args.value_of("time-step")
                .unwrap_or("0.00005")
                .parse()
                .unwrap(),
        )?
        .use_coriolis(!args.is_present("disable-coriolis"))?
        .use_gravity(!args.is_present("disable-gravity"))?
        .use_drag(!args.is_present("disable-drag"))?
        .set_velocity(args.value_of("velocity").unwrap_or("2710").parse().unwrap())?
        .set_grains(args.value_of("grains").unwrap_or("140").parse().unwrap())?
        .set_caliber(args.value_of("caliber").unwrap_or("0.264").parse().unwrap())?
        .set_height(
            args.value_of("scope-height")
                .unwrap_or("1.5")
                .parse()
                .unwrap(),
        )?
        .set_offset(
            args.value_of("scope-offset")
                .unwrap_or("0.0")
                .parse()
                .unwrap(),
        )?
        .set_roll(
            args.value_of("scope-cant")
                .unwrap_or("0.0")
                .parse()
                .unwrap(),
        )?
        .set_temperature(
            args.value_of("zero-temperature")
                .unwrap_or("68")
                .parse()
                .unwrap(),
        )?
        .set_pressure(
            args.value_of("zero-pressure")
                .unwrap_or("29.92")
                .parse()
                .unwrap(),
        )?
        .set_humidity(
            args.value_of("zero-humidity")
                .unwrap_or("0")
                .parse()
                .unwrap(),
        )?
        .set_wind_speed(
            args.value_of("zero-wind-speed")
                .unwrap_or("0")
                .parse()
                .unwrap(),
        )?
        .set_wind_angle(
            args.value_of("zero-wind-angle")
                .unwrap_or("0")
                .parse()
                .unwrap(),
        )?
        .set_shot_angle(
            args.value_of("zero-shot-angle")
                .unwrap_or("0")
                .parse()
                .unwrap(),
        )?
        .set_lattitude(
            args.value_of("zero-lattitude")
                .unwrap_or("0")
                .parse()
                .unwrap(),
        )?
        .set_bearing(
            args.value_of("zero-bearing")
                .unwrap_or("0")
                .parse()
                .unwrap(),
        )?
        .set_gravity(
            args.value_of("zero-gravity")
                .unwrap_or("-32.174")
                .parse()
                .unwrap(),
        )?)
}
pub fn flat_simulation(args: &ArgMatches, builder: SimulationBuilder) -> Result<Simulation> {
    Ok(builder.init_with(
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
            _ => panic!("Invalid BC Type"),
        },
    )?)
}
pub fn zero_simulation(args: &ArgMatches, simulation: Simulation) -> Result<Simulation> {
    Ok(simulation.zero(
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
    )?)
}
pub fn solution_builder(args: &ArgMatches, simulation: Simulation) -> Result<SimulationBuilder> {
    Ok(SimulationBuilder::from(simulation)
        .set_temperature(
            args.value_of("temperature")
                .unwrap_or("68")
                .parse()
                .unwrap(),
        )?
        .set_pressure(
            args.value_of("pressure")
                .unwrap_or("29.92")
                .parse()
                .unwrap(),
        )?
        .set_humidity(args.value_of("humidity").unwrap_or("0").parse().unwrap())?
        .set_wind_speed(args.value_of("wind-speed").unwrap_or("0").parse().unwrap())?
        .set_wind_angle(args.value_of("wind-angle").unwrap_or("0").parse().unwrap())?
        .set_shot_angle(args.value_of("shot-angle").unwrap_or("0").parse().unwrap())?
        .set_lattitude(args.value_of("lattitude").unwrap_or("0").parse().unwrap())?
        .set_bearing(args.value_of("bearing").unwrap_or("0").parse().unwrap())?
        .set_gravity(
            args.value_of("gravity")
                .unwrap_or("-32.174")
                .parse()
                .unwrap(),
        )?)
}
