use rballistics_flat::{model::point_mass::params, simulator::*, Numeric};

use std::env;

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() <= 19 {
        eprintln!("error: wrong number of args");
        usage(&argv[0]);
        return;
    }

    let initial_velocity: Numeric = argv[1].parse().unwrap(); // ft/s
    let los_angle: Numeric = argv[2].parse().unwrap(); // degrees
    let scope_height: Numeric = argv[3].parse().unwrap(); // inches
    let zero_distance: Numeric = argv[4].parse().unwrap(); // yards
    let weight: Numeric = argv[5].parse().unwrap(); // grains
    let caliber: Numeric = argv[6].parse().unwrap(); // inches
    let bc: Numeric = argv[7].parse().unwrap(); // dimensionless
    let drag_table: &str = &argv[8]; // Desired drag table (G1, G7, etc.)
    let wind_velocity: Numeric = argv[9].parse().unwrap(); // m/h
    let wind_angle: Numeric = argv[10].parse().unwrap(); // degrees
    let temperature: Numeric = argv[11].parse().unwrap(); // F
    let pressure: Numeric = argv[12].parse().unwrap(); // inHg
    let humidity: Numeric = argv[13].parse().unwrap(); // dimensionless, percentage
    let range: u32 = argv[14].parse().unwrap(); // range in yd
    let step: u32 = argv[15].parse().unwrap(); // step output in yd
    let step_factor: Numeric = argv[16].parse().unwrap(); // factor to determine step size
    let lattitude: Numeric = argv[17].parse().unwrap(); // Current lattitude in degrees
    let azimuth: Numeric = argv[18].parse().unwrap(); // Bearing relative to north (0 degrees north, 90 east, etc.)
    let offset: Numeric = argv[19].parse().unwrap(); // Angle offset in MOA for testing
                                                     // let gravity: Numeric = argv[20].parse().unwrap();

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

    let projectile = params::Projectile::new(weight, caliber, bc_enum, initial_velocity);
    let scope = params::Scope::new(scope_height);
    let zero_conditions = params::Conditions::new(
        params::Wind::new(0.0, 0.0),
        params::Atmosphere::new(temperature, pressure, humidity),
        params::Other::new(0.0, lattitude, azimuth, None),
    );
    let solve_conditions = params::Conditions::new(
        params::Wind::new(wind_velocity, wind_angle),
        params::Atmosphere::new(temperature, pressure, humidity),
        params::Other::new(los_angle, lattitude, azimuth, None),
    );
    let simulation = Simulator::new(
        &projectile,
        &scope,
        &zero_conditions,
        &solve_conditions,
        time_step,
    );

    let results = simulation.drop_table(zero_distance, step, range, offset);

    //simulation.zero(zero_distance, &zero_conditions, &drop_table_conditions);
    // println!("{:#?}", simulation.first_zero());

    println!(
        "{:>12} {:>14} {:>12} {:>15} {:>13} {:>8} {:>8}",
        "Distance(yd)",
        "Elevation(in)",
        "Windage(in)",
        "Velocity(ft/s)",
        "Energy(ftlb)",
        "MOA",
        "Time(s)",
    );
    for (distance, (elevation, windage, velocity, energy, moa, time)) in results.0.iter() {
        println!(
            "{:>12.0} {:>12.2} {} {:>10.2} {} {:>15.2} {:>13.2} {:>8.2} {:>8.3}",
            distance,
            elevation.abs(),
            Elevation(elevation).adjustment(),
            windage.abs(),
            Windage(windage).adjustment(),
            velocity,
            energy,
            moa,
            time,
        );
    }
}

pub use self::Adjustment::*;
pub enum Adjustment<'n> {
    Elevation(&'n Numeric),
    Windage(&'n Numeric),
}

impl Adjustment<'_> {
    // I have no idea if this should return static str or String
    fn adjustment(&self) -> &'static str {
        const MAX: Numeric = 0.001;
        match self {
            Elevation(&m) => {
                if m > -MAX && m < MAX {
                    " "
                } else if m.is_sign_positive() {
                    "D"
                } else {
                    "U"
                }
            }
            Windage(&m) => {
                if m > -MAX && m < MAX {
                    " "
                } else if m.is_sign_positive() {
                    "L"
                } else {
                    "R"
                }
            }
        }
    }
}

fn usage(name: &str) {
    println!(
        r#"
        Usage: {}
        velocity (ft/s)
        line_of_sight (degrees)
        scope_height (inches)
        zero_range (yards)
        weight (grains)
        caliber (inches)
        bc
        bc type
        wind_velocity (ft/s)
        wind_angle (degrees)
        temp (F)
        pressure (inHg)
        humidity (0-1)
        range (yards)
        step (yards)
        timestep_factor
        lattitude
        azimuth
        offset (moa)
        "#,
        name
    );
}
