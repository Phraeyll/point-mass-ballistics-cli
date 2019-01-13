use clap::{App, Arg, ArgMatches};
use rballistics_flat::{
    model::point_mass::{builder::*, iter::Output, *},
    Numeric,
};

// use std::env;

fn main() {
    let app = getopts();
    let pretty = app.is_present("pretty");
    let initial_velocity: Numeric = app
        .value_of("velocity")
        .expect("no velocity")
        .parse()
        .expect("velocity invalid");
    let weight: Numeric = app
        .value_of("grains")
        .expect("missing grains")
        .parse()
        .expect("grains invalid");
    let caliber: Numeric = app
        .value_of("caliber")
        .expect("missing caliber")
        .parse()
        .expect("");
    let bc: Numeric = app
        .value_of("bc")
        .expect("missing bc")
        .parse()
        .expect("bc invalid");
    let drag_table: &str = app.value_of("bc-type").expect("missing bc-type");
    let wind_velocity: Numeric = app
        .value_of("wind-speed")
        .expect("missing wind-speed")
        .parse()
        .expect("wind-speed invalid");
    let wind_angle: Numeric = app
        .value_of("wind-angle")
        .expect("missing wind-angle")
        .parse()
        .expect("wind-angle invalid");
    let temperature: Numeric = app
        .value_of("temperature")
        .expect("missing temperature")
        .parse()
        .expect("temperature invalid");
    let pressure: Numeric = app
        .value_of("pressure")
        .expect("missing pressure")
        .parse()
        .expect("pressure invalid");
    let humidity: Numeric = app
        .value_of("humidity")
        .expect("missing humidity")
        .parse()
        .expect("humidity invalid");
    let lattitude: Numeric = app
        .value_of("lattitude")
        .expect("missing lattitude")
        .parse()
        .expect("lattitude invalid");
    let azimuth: Numeric = app
        .value_of("bearing")
        .expect("missing bearing")
        .parse()
        .expect("bearing invalid");
    let los_angle: Numeric = app
        .value_of("shot-angle")
        .expect("missing shot-angle")
        .parse()
        .expect("shot-angle invalid");
    let scope_height: Numeric = app
        .value_of("scope-height")
        .expect("missing scope-height")
        .parse()
        .expect("");
    let zero_distance: Numeric = app
        .value_of("zero-distance")
        .expect("missing zero-distance")
        .parse()
        .expect("zero-distance invalid");
    let zero_offset: Numeric = app
        .value_of("zero-offset")
        .expect("missing zero-offset")
        .parse()
        .expect("zero-offset invalid");
    let zero_tolerance: Numeric = app
        .value_of("zero-tolerance")
        .expect("missing zero-tolerance")
        .parse()
        .expect("zero-tolerance invalid");
    let range_start: u32 = app
        .value_of("table-start")
        .expect("missing table-start")
        .parse()
        .expect("table-start invalid");
    let range_end: u32 = app
        .value_of("table-end")
        .expect("missing table-end")
        .parse()
        .expect("table-end invalid");
    let step: u32 = app
        .value_of("table-step")
        .expect("no table-step")
        .parse()
        .expect("table-step invalid");
    let output_tolerance: Numeric = app
        .value_of("table-tolerance")
        .expect("")
        .parse()
        .expect("table-tolerance invalid");
    let step_factor: Numeric = app
        .value_of("factor")
        .expect("missing factor")
        .parse()
        .expect("factor invalid");
    let pitch_offset: Numeric = app
        .value_of("pitch-offset")
        .expect("missing pitch-offset")
        .parse()
        .expect("pitch-offset invalid");
    let yaw_offset: Numeric = app
        .value_of("yaw-offset")
        .expect("missing yaw-offset")
        .parse()
        .expect("yaw-offset invalid");

    let time_step: Numeric = 1.0 / (step_factor * initial_velocity);

    // Ugly - this needs to be handle in library, parsing bc as "G7(0.305)" for example
    let bc_enum = match drag_table {
        "G1" => BallisticCoefficient::G1(bc),
        "G2" => BallisticCoefficient::G2(bc),
        "G5" => BallisticCoefficient::G5(bc),
        "G6" => BallisticCoefficient::G6(bc),
        "G7" => BallisticCoefficient::G7(bc),
        "G8" => BallisticCoefficient::G8(bc),
        "GI" => BallisticCoefficient::GI(bc),
        "GS" => BallisticCoefficient::GS(bc),
        _ => BallisticCoefficient::G1(bc),
    };

    let projectile_both = Projectile::new(weight, caliber, bc_enum, initial_velocity);
    let scope_both = Scope::new(scope_height);

    let atmosphere_both = Atmosphere::new(temperature, pressure, humidity);

    let wind_solve = Wind::new(wind_velocity, wind_angle);
    let wind_zero = Wind::new(0.0, 0.0);

    let other_solve = Other::new(los_angle, lattitude, azimuth, None);
    let other_zero = Other::new(0.0, lattitude, azimuth, None);

    let zero_conditions = Conditions::new(&wind_zero, &atmosphere_both, &other_zero);
    let solve_conditions = Conditions::new(&wind_solve, &atmosphere_both, &other_solve);
    let builder = SimulationBuilder::new(
        &projectile_both,
        &scope_both,
        &zero_conditions,
        &solve_conditions,
        time_step,
    );
    let simulation = builder.solve_for(
        zero_distance,
        zero_offset,
        zero_tolerance,
        pitch_offset,
        yaw_offset,
    );
    // let simulation = builder.flat();

    let table = simulation.table(step, range_start, range_end);

    if pretty {
        println!("+--------------+----------+---------------+-------------+------------+------------+----------------+--------------+----------+");
        println!(
            "| {:>12} | {:>8} | {:>13} | {:>11} | {:>10} | {:>10} | {:>14} | {:>12} | {:>8} |",
            "Distance(yd)",
            "MOA",
            "Elevation(in)",
            "Windage(in)",
            "Vertical",
            "Horizontal",
            "Velocity(ft/s)",
            "Energy(ftlb)",
            "Time(s)",
        );
    } else {
        println!(
            "{:>12} {:>8} {:>13} {:>11} {:>10} {:>10} {:>14} {:>12} {:>7}",
            "Distance(yd)",
            "MOA",
            "Elevation(in)",
            "Windage(in)",
            "Vertical",
            "Horizontal",
            "Velocity(ft/s)",
            "Energy(ftlb)",
            "Time(s)",
        );
    }
    for (distance, p) in table.iter() {
        let (elevation, windage, velocity, energy, moa, vertical_moa, horizontal_moa, time) = (
            p.elevation(),
            p.windage(),
            p.velocity(),
            p.energy(),
            p.moa(),
            p.vertical_moa(),
            p.horizontal_moa(),
            p.time(),
        );
        let vertical = Elevation(&elevation).adjustment(output_tolerance);
        let horizontal = Windage(&windage).adjustment(output_tolerance);
        if pretty {
            println!("+--------------+----------+---------------+-------------+------------+------------+----------------+--------------+----------+");
            println!(
            "| {:>12.0} | {:>8.2} | {:>11.2} {} | {:>9.2} {} | {:>8.2} {} | {:>8.2} {} | {:>14.2} | {:>12.2} | {:>8.3} |",
            distance,
            moa,
            elevation.abs(),
            vertical,
            windage.abs(),
            horizontal,
            vertical_moa,
            vertical,
            horizontal_moa,
            horizontal,
            velocity,
            energy,
            time,
        );
        } else {
            println!(
            "{:>12.0} {:>8.2} {:>11.2} {} {:>9.2} {} {:>8.2} {} {:>8.2} {} {:>14.2} {:>12.2} {:>8.3}",
            distance,
            moa,
            elevation.abs(),
            vertical,
            windage.abs(),
            horizontal,
            vertical_moa,
            vertical,
            horizontal_moa,
            horizontal,
            velocity,
            energy,
            time,
        );
        }
    }
    if pretty {
        println!("+--------------+----------+---------------+-------------+------------+------------+----------------+--------------+----------+");
    }
}

pub use self::Adjustment::*;
pub enum Adjustment<'n> {
    Elevation(&'n Numeric),
    Windage(&'n Numeric),
}

impl Adjustment<'_> {
    fn adjustment(&self, output_tolerance: Numeric) -> char {
        let tolerance = output_tolerance;
        match self {
            Elevation(&m) => {
                if m > -tolerance && m < tolerance {
                    '*'
                } else if m.is_sign_positive() {
                    'D'
                } else {
                    'U'
                }
            }
            Windage(&m) => {
                if m > -tolerance && m < tolerance {
                    '*'
                } else if m.is_sign_positive() {
                    'L'
                } else {
                    'R'
                }
            }
        }
    }
}
fn getopts<'a>() -> ArgMatches<'a> {
    App::new("Ballistics Solver")
        .version("0.0.1")
        .author("Phraeyll <Phraeyll@users.no-reply.github.com>")
        .about(
            r#"
            Produces range table from vector based simulation of Newtons Equations
            using point mass model of ballistics.
            Currently, this accounts for drag, gravity, and Coriolis/Eotovos forces.
            This does not currently factor in gyroscopic drift, nor aerodynamic jump.
            "#,
        )
        .arg(
            Arg::with_name("pretty")
                .long("pretty")
                .help("Pretty Output"),
        )
        .arg(
            Arg::with_name("velocity")
                .long("velocity")
                .required(true)
                .takes_value(true)
                .help("Projectile Velocity (ft/s)"),
        )
        .arg(
            Arg::with_name("grains")
                .long("grains")
                .required(true)
                .takes_value(true)
                .help("Projectile Weight (grains)"),
        )
        .arg(
            Arg::with_name("caliber")
                .long("caliber")
                .required(true)
                .takes_value(true)
                .help("Caliber (inches)"),
        )
        .arg(
            Arg::with_name("bc")
                .long("bc")
                .required(true)
                .takes_value(true)
                .help("Ballistic Coefficient"),
        )
        .arg(
            Arg::with_name("bc-type")
                .long("bc-type")
                .required(true)
                .takes_value(true)
                .help("BC Type [G1 G2 G5 G6 G7 G8 GI GS]"),
        )
        .arg(
            Arg::with_name("wind-speed")
                .long("wind-speed")
                .required(true)
                .takes_value(true)
                .help("Wind Speed (miles/hour)"),
        )
        .arg(
            Arg::with_name("wind-angle")
                .long("wind-angle")
                .required(true)
                .takes_value(true)
                .help("Wind Angle (Decimal Degrees)"),
        )
        .arg(
            Arg::with_name("temperature")
                .long("temperature")
                .required(true)
                .takes_value(true)
                .help("Temperature (Fahrenheit)"),
        )
        .arg(
            Arg::with_name("pressure")
                .long("pressure")
                .required(true)
                .takes_value(true)
                .help("Pressure (InHg)"),
        )
        .arg(
            Arg::with_name("humidity")
                .long("humidity")
                .required(true)
                .takes_value(true)
                .help("Humidity (Value between 0 & 1) [0 => 0%; 1 => 100%]"),
        )
        .arg(
            Arg::with_name("lattitude")
                .long("lattitude")
                .required(true)
                .takes_value(true)
                .help("Lattitude (Decimal Degrees)"),
        )
        .arg(
            Arg::with_name("bearing")
                .long("bearing")
                .required(true)
                .takes_value(true)
                .help("Azimuthal Bearing (Decimal Degrees)"),
        )
        .arg(
            Arg::with_name("shot-angle")
                .long("shot-angle")
                .required(true)
                .takes_value(true)
                .help("Line of Sight Angle (Decimal Degrees)"),
        )
        .arg(
            Arg::with_name("scope-height")
                .long("scope-height")
                .required(true)
                .takes_value(true)
                .help("Scope Height above Boreline (Inches)"),
        )
        .arg(
            Arg::with_name("zero-distance")
                .long("zero-distance")
                .required(true)
                .takes_value(true)
                .help("Zeroed Distance (Yards)"),
        )
        .arg(
            Arg::with_name("zero-offset")
                .long("zero-offset")
                .required(true)
                .takes_value(true)
                .help("Zero Offset (Inches)"),
        )
        .arg(
            Arg::with_name("zero-tolerance")
                .long("zero-tolerance")
                .required(true)
                .takes_value(true)
                .help("Zero Tolerance (Inches)"),
        )
        .arg(
            Arg::with_name("pitch-offset")
                .long("pitch-offset")
                .required(true)
                .takes_value(true)
                .help("Pitch Offset (MOA - Minutes of Angle)"),
        )
        .arg(
            Arg::with_name("yaw-offset")
                .long("yaw-offset")
                .required(true)
                .takes_value(true)
                .help("Yaw Offset (MOA - Minutes of Angle)"),
        )
        .arg(
            Arg::with_name("table-start")
                .long("table-start")
                .required(true)
                .takes_value(true)
                .help("Table Start Distance (Yards)"),
        )
        .arg(
            Arg::with_name("table-end")
                .long("table-end")
                .required(true)
                .takes_value(true)
                .help("Table End Distance (Yards)"),
        )
        .arg(
            Arg::with_name("table-step")
                .long("table-step")
                .required(true)
                .takes_value(true)
                .help("Table Step Distance (Yards)"),
        )
        .arg(
            Arg::with_name("table-tolerance")
                .long("table-tolerance")
                .required(true)
                .takes_value(true)
                .help("Table Adjustments Tolerance (Inches)"),
        )
        .arg(
            Arg::with_name("factor")
                .long("factor")
                .required(true)
                .takes_value(true)
                .help("Simulation Factor (Higher Numbers for slower, more accurate simulations)"),
        )
        .get_matches()
}
