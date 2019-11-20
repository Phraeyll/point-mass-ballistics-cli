use point_mass_ballistics::{
    degree, fahrenheit, foot_per_second, foot_per_second_squared, grain, inch, inch_of_mercury,
    mile_per_hour, moa, second, yard, Acceleration, Angle, BcKind::*, Length, Mass, Numeric,
    Pressure, Result, Simulation, SimulationBuilder, ThermodynamicTemperature, Time, Velocity,
};

pub fn sim_before_zero<'t>(args: &ArgMatches) -> Result<SimulationBuilder<'t>> {
    let builder = SimulationBuilder::new();
    Ok(builder
        .set_time_step(Time::new::<second>(
            args.value_of("time-step")
                .unwrap_or("0.00005")
                .parse::<Numeric>()
                .unwrap(),
        ))?
        .set_bc(
            args.value_of("bc")
                .unwrap_or("0.305")
                .parse::<Numeric>()
                .unwrap(),
            match args
                .value_of("bc-type")
                .unwrap_or("G7")
                .to_ascii_uppercase()
                .as_ref()
            {
                "G1" => G1,
                "G2" => G2,
                "G5" => G5,
                "G6" => G6,
                "G7" => G7,
                "G8" => G8,
                "GI" => GI,
                "GS" => GS,
                _ => panic!("Invalid BC Type"),
            },
        )?
        .use_coriolis(!args.is_present("disable-coriolis"))
        .use_gravity(!args.is_present("disable-gravity"))
        .use_drag(!args.is_present("disable-drag"))
        .set_velocity(Velocity::new::<foot_per_second>(
            args.value_of("velocity")
                .unwrap_or("2710")
                .parse::<Numeric>()
                .unwrap(),
        ))?
        .set_mass(Mass::new::<grain>(
            args.value_of("grains")
                .unwrap_or("140")
                .parse::<Numeric>()
                .unwrap(),
        ))?
        .set_caliber(Length::new::<inch>(
            args.value_of("caliber")
                .unwrap_or("0.264")
                .parse::<Numeric>()
                .unwrap(),
        ))?
        .set_scope_height(Length::new::<inch>(
            args.value_of("scope-height")
                .unwrap_or("1.5")
                .parse::<Numeric>()
                .unwrap(),
        ))
        .set_scope_offset(Length::new::<inch>(
            args.value_of("scope-offset")
                .unwrap_or("0.0")
                .parse::<Numeric>()
                .unwrap(),
        ))
        .set_scope_roll(Angle::new::<moa>(
            args.value_of("scope-cant")
                .unwrap_or("0.0")
                .parse::<Numeric>()
                .unwrap(),
        ))
        .set_temperature(ThermodynamicTemperature::new::<fahrenheit>(
            args.value_of("zero-temperature")
                .unwrap_or("68")
                .parse::<Numeric>()
                .unwrap(),
        ))?
        .set_pressure(Pressure::new::<inch_of_mercury>(
            args.value_of("zero-pressure")
                .unwrap_or("29.92")
                .parse::<Numeric>()
                .unwrap(),
        ))?
        .set_humidity(
            args.value_of("zero-humidity")
                .unwrap_or("0")
                .parse::<Numeric>()
                .unwrap(),
        )?
        .set_wind_speed(Velocity::new::<mile_per_hour>(
            args.value_of("zero-wind-speed")
                .unwrap_or("0")
                .parse::<Numeric>()
                .unwrap(),
        ))?
        .set_wind_angle(Angle::new::<degree>(
            args.value_of("zero-wind-angle")
                .unwrap_or("0")
                .parse::<Numeric>()
                .unwrap(),
        ))?
        .set_shot_angle(Angle::new::<degree>(
            args.value_of("zero-shot-angle")
                .unwrap_or("0")
                .parse::<Numeric>()
                .unwrap(),
        ))?
        .set_lattitude(Angle::new::<degree>(
            args.value_of("zero-lattitude")
                .unwrap_or("0")
                .parse::<Numeric>()
                .unwrap(),
        ))?
        .set_bearing(Angle::new::<degree>(
            args.value_of("zero-bearing")
                .unwrap_or("0")
                .parse::<Numeric>()
                .unwrap(),
        ))?
        .set_gravity(Acceleration::new::<foot_per_second_squared>(
            args.value_of("zero-gravity")
                .unwrap_or("-32.1740")
                .parse::<Numeric>()
                .unwrap(),
        ))?)
}
pub fn try_zero_simulation<'t>(
    args: &ArgMatches,
    simulation: &'t mut Simulation<'t>,
) -> Result<(Angle, Angle)> {
    let (pitch, yaw) = simulation.find_zero_angles(
        Length::new::<yard>(
            args.value_of("zero-distance")
                .unwrap_or("200")
                .parse::<Numeric>()
                .unwrap(),
        ),
        Length::new::<inch>(
            args.value_of("zero-height")
                .unwrap_or("0")
                .parse::<Numeric>()
                .unwrap(),
        ),
        Length::new::<inch>(
            args.value_of("zero-offset")
                .unwrap_or("0")
                .parse::<Numeric>()
                .unwrap(),
        ),
        Length::new::<inch>(
            args.value_of("zero-tolerance")
                .unwrap_or("0.001")
                .parse::<Numeric>()
                .unwrap(),
        ),
    )?;
    Ok((pitch, yaw))
}
pub fn sim_after_zero<'t>(
    args: &ArgMatches,
    builder: SimulationBuilder<'t>,
    pitch: Angle,
    yaw: Angle,
) -> Result<SimulationBuilder<'t>> {
    Ok(builder
        .set_scope_pitch(
            pitch
                + Angle::new::<moa>(
                    args.value_of("scope-pitch")
                        .unwrap_or("0")
                        .parse::<Numeric>()
                        .unwrap(),
                ),
        )
        .set_scope_yaw(
            yaw + Angle::new::<moa>(
                args.value_of("scope-yaw")
                    .unwrap_or("0")
                    .parse::<Numeric>()
                    .unwrap(),
            ),
        )
        .set_temperature(ThermodynamicTemperature::new::<fahrenheit>(
            args.value_of("temperature")
                .unwrap_or("68")
                .parse::<Numeric>()
                .unwrap(),
        ))?
        .set_pressure(Pressure::new::<inch_of_mercury>(
            args.value_of("pressure")
                .unwrap_or("29.92")
                .parse::<Numeric>()
                .unwrap(),
        ))?
        .set_humidity(
            args.value_of("humidity")
                .unwrap_or("0")
                .parse::<Numeric>()
                .unwrap(),
        )?
        .set_wind_speed(Velocity::new::<mile_per_hour>(
            args.value_of("wind-speed")
                .unwrap_or("0")
                .parse::<Numeric>()
                .unwrap(),
        ))?
        .set_wind_angle(Angle::new::<degree>(
            args.value_of("wind-angle")
                .unwrap_or("0")
                .parse::<Numeric>()
                .unwrap(),
        ))?
        .set_shot_angle(Angle::new::<degree>(
            args.value_of("shot-angle")
                .unwrap_or("0")
                .parse::<Numeric>()
                .unwrap(),
        ))?
        .set_lattitude(Angle::new::<degree>(
            args.value_of("lattitude")
                .unwrap_or("0")
                .parse::<Numeric>()
                .unwrap(),
        ))?
        .set_bearing(Angle::new::<degree>(
            args.value_of("bearing")
                .unwrap_or("0")
                .parse::<Numeric>()
                .unwrap(),
        ))?
        .set_gravity(Acceleration::new::<foot_per_second_squared>(
            args.value_of("gravity")
                .unwrap_or("-32.1740")
                .parse::<Numeric>()
                .unwrap(),
        ))?)
}
