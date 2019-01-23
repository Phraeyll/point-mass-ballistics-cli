use clap::{App, Arg};

pub fn parse<'a, 'b>() -> App<'a, 'b> {
    App::new("Ballistic Solver")
        .version("0.0.1")
        .author("Phraeyll <Phraeyll@users.no-reply.github.com>")
        .about(
            r#"
            Produces range table from vector based simulation of Newtons Equations
            using standard, unmodified, point mass model of ballistics.
            Currently, this accounts for drag, gravity, and Coriolis/Eotvos forces.
            This does not currently factor in gyroscopic drift, nor aerodynamic jump.
            Drag tables obtained from JBM Ballistics, and methodologies are mostly from
            Robert L. McCoy's "Modern Exterior Ballistics" ISBN 978-0-7643-3825-0

            The eventual goal of this program is to support modified point mass trajectories
            as well, for factoring in gyroscopic drift and aerodynamic jump (4-DOF models)
            "#,
        )
        .arg(Arg::with_name("flat").long("flat").help("Flat Model"))
        .arg(
            Arg::with_name("disable-drag")
                .long("disable-drag")
                .help("Disable drag"),
        )
        .arg(
            Arg::with_name("disable-coriolis")
                .long("disable-coriolis")
                .help("Disable coriolis"),
        )
        .arg(
            Arg::with_name("disable-gravity")
                .long("disable-gravity")
                .help("Disable gravity"),
        )
        .arg(
            Arg::with_name("pretty")
                .long("pretty")
                .help("Pretty Output"),
        )
        .arg(
            Arg::with_name("velocity")
                .long("velocity")
                .takes_value(true)
                .help("Projectile Velocity (ft/s)"),
        )
        .arg(
            Arg::with_name("grains")
                .long("grains")
                .takes_value(true)
                .help("Projectile Weight (grains)"),
        )
        .arg(
            Arg::with_name("caliber")
                .long("caliber")
                .takes_value(true)
                .help("Caliber (inches)"),
        )
        .arg(
            Arg::with_name("bc")
                .long("bc")
                .takes_value(true)
                .help("Ballistic Coefficient"),
        )
        .arg(
            Arg::with_name("bc-type")
                .long("bc-type")
                .takes_value(true)
                .help("BC Type [G1 G2 G5 G6 G7 G8 GI GS]"),
        )
        .arg(
            Arg::with_name("scope-height")
                .allow_hyphen_values(true)
                .long("scope-height")
                .takes_value(true)
                .help("Scope Height above/below Boreline (Inches)"),
        )
        .arg(
            Arg::with_name("scope-offset")
                .allow_hyphen_values(true)
                .long("scope-offset")
                .takes_value(true)
                .help("Scope Offset left/right of Boreline (Inches)"),
        )
        .arg(
            Arg::with_name("scope-cant")
                .allow_hyphen_values(true)
                .long("scope-cant")
                .takes_value(true)
                .help("Scope Cant/Roll Clockwise from Shooter"),
        )
        .arg(
            Arg::with_name("wind-speed")
                .long("wind-speed")
                .takes_value(true)
                .help("Wind Speed (miles/hour)"),
        )
        .arg(
            Arg::with_name("wind-angle")
                .allow_hyphen_values(true)
                .long("wind-angle")
                .takes_value(true)
                .help("Wind Angle (Decimal Degrees)"),
        )
        .arg(
            Arg::with_name("zero-wind-speed")
                .long("zero-wind-speed")
                .takes_value(true)
                .help("Wind Speed (miles/hour)"),
        )
        .arg(
            Arg::with_name("zero-wind-angle")
                .allow_hyphen_values(true)
                .long("zero-wind-angle")
                .takes_value(true)
                .help("Wind Angle (Decimal Degrees)"),
        )
        .arg(
            Arg::with_name("temperature")
                .allow_hyphen_values(true)
                .long("temperature")
                .takes_value(true)
                .help("Temperature (Fahrenheit)"),
        )
        .arg(
            Arg::with_name("pressure")
                .allow_hyphen_values(true)
                .long("pressure")
                .takes_value(true)
                .help("Pressure (InHg)"),
        )
        .arg(
            Arg::with_name("humidity")
                .long("humidity")
                .takes_value(true)
                .help("Humidity (Value between 0 & 1) [0 => 0%; 1 => 100%]"),
        )
        .arg(
            Arg::with_name("zero-temperature")
                .allow_hyphen_values(true)
                .long("zero-temperature")
                .takes_value(true)
                .help("Temperature (Fahrenheit)"),
        )
        .arg(
            Arg::with_name("zero-pressure")
                .allow_hyphen_values(true)
                .long("zero-pressure")
                .takes_value(true)
                .help("Pressure (InHg)"),
        )
        .arg(
            Arg::with_name("zero-humidity")
                .long("zero-humidity")
                .takes_value(true)
                .help("Humidity (Value between 0 & 1) [0 => 0%; 1 => 100%]"),
        )
        .arg(
            Arg::with_name("lattitude")
                .allow_hyphen_values(true)
                .long("lattitude")
                .takes_value(true)
                .help("Lattitude (Decimal Degrees)"),
        )
        .arg(
            Arg::with_name("bearing")
                .allow_hyphen_values(true)
                .long("bearing")
                .takes_value(true)
                .help("Azimuthal Bearing (Decimal Degrees)"),
        )
        .arg(
            Arg::with_name("shot-angle")
                .allow_hyphen_values(true)
                .long("shot-angle")
                .takes_value(true)
                .help("Line of Sight Angle (Decimal Degrees)"),
        )
        .arg(
            Arg::with_name("gravity")
                .allow_hyphen_values(true)
                .long("gravity")
                .takes_value(true)
                .help("Gravity (ft/s^2)"),
        )
        .arg(
            Arg::with_name("zero-gravity")
                .allow_hyphen_values(true)
                .long("zero-gravity")
                .takes_value(true)
                .help("Gravity (ft/s^2)"),
        )
        .arg(
            Arg::with_name("zero-lattitude")
                .allow_hyphen_values(true)
                .long("zero-lattitude")
                .takes_value(true)
                .help("Lattitude (Decimal Degrees)"),
        )
        .arg(
            Arg::with_name("zero-bearing")
                .allow_hyphen_values(true)
                .long("zero-bearing")
                .takes_value(true)
                .help("Azimuthal Bearing (Decimal Degrees)"),
        )
        .arg(
            Arg::with_name("zero-shot-angle")
                .allow_hyphen_values(true)
                .long("zero-shot-angle")
                .takes_value(true)
                .help("Line of Sight Angle (Decimal Degrees)"),
        )
        .arg(
            Arg::with_name("zero-distance")
                .long("zero-distance")
                .takes_value(true)
                .help("Zeroed Distance (Yards)"),
        )
        .arg(
            Arg::with_name("zero-height")
                .allow_hyphen_values(true)
                .long("zero-height")
                .takes_value(true)
                .help("Vertical Zero Offset (height) (Inches)"),
        )
        .arg(
            Arg::with_name("zero-offset")
                .allow_hyphen_values(true)
                .long("zero-offset")
                .takes_value(true)
                .help("Horizontal Zero Offset (windage) (Inches)"),
        )
        .arg(
            Arg::with_name("zero-tolerance")
                .long("zero-tolerance")
                .takes_value(true)
                .help("Zero Tolerance (Inches)"),
        )
        .arg(
            Arg::with_name("pitch")
                .allow_hyphen_values(true)
                .long("pitch")
                .takes_value(true)
                .help("Manual Pitch Adjustment (MOA - Minutes of Angle)"),
        )
        .arg(
            Arg::with_name("yaw")
                .allow_hyphen_values(true)
                .long("yaw")
                .takes_value(true)
                .help("Manual Yaw Adjustment (MOA - Minutes of Angle)"),
        )
        .arg(
            Arg::with_name("table-start")
                .long("table-start")
                .takes_value(true)
                .help("Table Start Distance (Yards)"),
        )
        .arg(
            Arg::with_name("table-end")
                .long("table-end")
                .takes_value(true)
                .help("Table End Distance (Yards)"),
        )
        .arg(
            Arg::with_name("table-step")
                .long("table-step")
                .takes_value(true)
                .help("Table Step Distance (Yards)"),
        )
        .arg(
            Arg::with_name("table-tolerance")
                .long("table-tolerance")
                .takes_value(true)
                .help("Table Adjustments Tolerance (Inches)"),
        )
        .arg(
            Arg::with_name("time-step")
                .long("time-step")
                .takes_value(true)
                .help(
                    "Simulation Time Step (smaller numbers for slower, more accurate simulation)",
                ),
        )
}
