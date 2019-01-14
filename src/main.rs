use clap::{App, Arg};
use rballistics_flat::{
    model::point_mass::{builder::*, iter::Output, *},
    Numeric,
};

fn main() {
    let app = cli().get_matches();
    let pretty = app.is_present("pretty");
    let initial_velocity = app.value_of("velocity").unwrap().parse().unwrap();
    let factor: Numeric = app.value_of("factor").unwrap().parse().unwrap();
    let time_step: Numeric =
        1.0 /  (factor * initial_velocity);

    // Ugly - this needs to be handle in library, parsing bc as "G7(0.305)" for example
    let bc = app.value_of("bc").unwrap().parse().unwrap();
    let bc_enum = match app.value_of("bc-type").unwrap() {
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

    let projectile_both = Projectile::new(
        app.value_of("grains").unwrap().parse().unwrap(),
        app.value_of("caliber").unwrap().parse().unwrap(),
        bc_enum,
        initial_velocity,
    );

    let scope_both = Scope::new(app.value_of("scope-height").unwrap().parse().unwrap());

    let atmosphere_both = Atmosphere::new(
        app.value_of("temperature").unwrap().parse().unwrap(),
        app.value_of("pressure").unwrap().parse().unwrap(),
        app.value_of("humidity").unwrap().parse().unwrap(),
    );

    let wind_solve = Wind::new(
        app.value_of("wind-speed").unwrap().parse().unwrap(),
        app.value_of("wind-angle").unwrap().parse().unwrap(),
    );
    let wind_zero = Wind::new(0.0, 0.0);

    let lattitude = app.value_of("lattitude").unwrap().parse().unwrap();
    let azimuth = app.value_of("bearing").unwrap().parse().unwrap();
    let other_solve = Other::new(
        app.value_of("shot-angle").unwrap().parse().unwrap(),
        lattitude,
        azimuth,
        None,
    );
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
        app.value_of("zero-distance").unwrap().parse().unwrap(),
        app.value_of("zero-offset").unwrap().parse().unwrap(),
        app.value_of("zero-tolerance").unwrap().parse().unwrap(),
        app.value_of("pitch-offset").unwrap().parse().unwrap(),
        app.value_of("yaw-offset").unwrap().parse().unwrap(),
    );
    // let simulation = builder.flat();

    let table = simulation.table(
        app.value_of("table-step").unwrap().parse().unwrap(),
        app.value_of("table-start").unwrap().parse().unwrap(),
        app.value_of("table-end").unwrap().parse().unwrap(),
    );

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
            "{:>12} {:>8} {:>13} {:>11} {:>10} {:>10} {:>14} {:>12} {:>8}",
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
        let output_tolerance = app.value_of("table-tolerance").unwrap().parse().unwrap();
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
fn cli<'a, 'b>() -> App<'a, 'b> {
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
                .allow_hyphen_values(true)
                .long("wind-angle")
                .required(true)
                .takes_value(true)
                .help("Wind Angle (Decimal Degrees)"),
        )
        .arg(
            Arg::with_name("temperature")
                .allow_hyphen_values(true)
                .long("temperature")
                .required(true)
                .takes_value(true)
                .help("Temperature (Fahrenheit)"),
        )
        .arg(
            Arg::with_name("pressure")
                .allow_hyphen_values(true)
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
                .allow_hyphen_values(true)
                .long("lattitude")
                .required(true)
                .takes_value(true)
                .help("Lattitude (Decimal Degrees)"),
        )
        .arg(
            Arg::with_name("bearing")
                .allow_hyphen_values(true)
                .long("bearing")
                .required(true)
                .takes_value(true)
                .help("Azimuthal Bearing (Decimal Degrees)"),
        )
        .arg(
            Arg::with_name("shot-angle")
                .allow_hyphen_values(true)
                .long("shot-angle")
                .required(true)
                .takes_value(true)
                .help("Line of Sight Angle (Decimal Degrees)"),
        )
        .arg(
            Arg::with_name("scope-height")
                .allow_hyphen_values(true)
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
                .allow_hyphen_values(true)
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
                .allow_hyphen_values(true)
                .long("pitch-offset")
                .required(true)
                .takes_value(true)
                .help("Pitch Offset (MOA - Minutes of Angle)"),
        )
        .arg(
            Arg::with_name("yaw-offset")
                .allow_hyphen_values(true)
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
}
