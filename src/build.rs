use clap::ArgMatches;

use point_mass_ballistics::model::builder::*;

pub trait StagedBuilder {
    fn with_conditions(self, args: &ArgMatches) -> Self;
    fn with_zero_conditions(self, args: &ArgMatches) -> Self;
    fn increment_by(self, args: &ArgMatches) -> Self;
}

pub fn base(args: &ArgMatches) -> SimulationBuilder {
    SimulationBuilder::default()
        .time_step(
            args.value_of("time-step")
                .unwrap_or("0.00005")
                .parse()
                .unwrap(),
        )
        .expect("time-step")
        .set_velocity(args.value_of("velocity").unwrap_or("2710").parse().unwrap())
        .expect("velocity")
        .set_grains(args.value_of("grains").unwrap_or("140").parse().unwrap())
        .expect("grains")
        .set_caliber(args.value_of("caliber").unwrap_or("0.264").parse().unwrap())
        .expect("caliber")
        .set_height(
            args.value_of("scope-height")
                .unwrap_or("1.5")
                .parse()
                .unwrap(),
        )
        .set_offset(
            args.value_of("scope-offset")
                .unwrap_or("0.0")
                .parse()
                .unwrap(),
        )
        .use_coriolis(!args.is_present("disable-coriolis"))
        .use_gravity(!args.is_present("disable-gravity"))
        .use_drag(!args.is_present("disable-drag"))
}
impl StagedBuilder for SimulationBuilder {
    fn with_conditions(self, args: &ArgMatches) -> Self {
        self.set_temperature(
            args.value_of("temperature")
                .unwrap_or("68")
                .parse()
                .unwrap(),
        )
        .expect("temperature")
        .set_pressure(
            args.value_of("pressure")
                .unwrap_or("29.92")
                .parse()
                .unwrap(),
        )
        .expect("pressure")
        .set_humidity(args.value_of("humidity").unwrap_or("0").parse().unwrap())
        .expect("humidity")
        .set_wind_speed(args.value_of("wind-speed").unwrap_or("0").parse().unwrap())
        .expect("wind-speed")
        .set_wind_angle(args.value_of("wind-angle").unwrap_or("0").parse().unwrap())
        .expect("wind-angle")
        .set_shot_angle(args.value_of("shot-angle").unwrap_or("0").parse().unwrap())
        .expect("shot-angle")
        .set_lattitude(args.value_of("lattitude").unwrap_or("0").parse().unwrap())
        .expect("lattitude")
        .set_bearing(args.value_of("bearing").unwrap_or("0").parse().unwrap())
        .expect("bearing")
        .set_gravity(
            args.value_of("gravity")
                .unwrap_or("-32.174")
                .parse()
                .unwrap(),
        )
    }
    fn with_zero_conditions(self, args: &ArgMatches) -> Self {
        self.set_temperature(
            args.value_of("zero-temperature")
                .unwrap_or("68")
                .parse()
                .unwrap(),
        )
        .expect("zero-temperature")
        .set_pressure(
            args.value_of("zero-pressure")
                .unwrap_or("29.92")
                .parse()
                .unwrap(),
        )
        .expect("zero-pressure")
        .set_humidity(
            args.value_of("zero-humidity")
                .unwrap_or("0")
                .parse()
                .unwrap(),
        )
        .expect("zero-humidity")
        .set_wind_speed(
            args.value_of("zero-wind-speed")
                .unwrap_or("0")
                .parse()
                .unwrap(),
        )
        .expect("zero-wind-speed")
        .set_wind_angle(
            args.value_of("zero-wind-angle")
                .unwrap_or("0")
                .parse()
                .unwrap(),
        )
        .expect("zero-wind-angle")
        .set_shot_angle(
            args.value_of("zero-shot-angle")
                .unwrap_or("0")
                .parse()
                .unwrap(),
        )
        .expect("zero-shot-angle")
        .set_lattitude(
            args.value_of("zero-lattitude")
                .unwrap_or("0")
                .parse()
                .unwrap(),
        )
        .expect("zero-lattitude")
        .set_bearing(
            args.value_of("zero-bearing")
                .unwrap_or("0")
                .parse()
                .unwrap(),
        )
        .expect("zero-bearing")
        .set_gravity(
            args.value_of("zero-gravity")
                .unwrap_or("-32.174")
                .parse()
                .unwrap(),
        )
    }
    fn increment_by(self, args: &ArgMatches) -> Self {
        self.increment_pitch(args.value_of("pitch").unwrap_or("0").parse().unwrap())
            .increment_yaw(args.value_of("yaw").unwrap_or("0").parse().unwrap())
    }
}
