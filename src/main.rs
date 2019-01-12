use rballistics_flat::{
    model::point_mass::{builder::*, iter::Output, *},
    Numeric,
};

use std::env;

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() <= 24 {
        eprintln!("error: wrong number of args");
        usage(&argv[0]);
        return;
    }

    let initial_velocity: Numeric = argv[1].parse().unwrap(); // ft/s
    let los_angle: Numeric = argv[2].parse().unwrap(); // degrees
    let scope_height: Numeric = argv[3].parse().unwrap(); // inches
    let weight: Numeric = argv[4].parse().unwrap(); // grains
    let caliber: Numeric = argv[5].parse().unwrap(); // inches
    let bc: Numeric = argv[6].parse().unwrap(); // dimensionless
    let drag_table: &str = &argv[7]; // Desired drag table (G1, G7, etc.)
    let wind_velocity: Numeric = argv[8].parse().unwrap(); // m/h
    let wind_angle: Numeric = argv[9].parse().unwrap(); // degrees
    let temperature: Numeric = argv[10].parse().unwrap(); // F
    let pressure: Numeric = argv[11].parse().unwrap(); // inHg
    let humidity: Numeric = argv[12].parse().unwrap(); // dimensionless, percentage
    let range_start: u32 = argv[13].parse().unwrap(); // range start in yd
    let range_end: u32 = argv[14].parse().unwrap(); // range end in yd
    let step: u32 = argv[15].parse().unwrap(); // step output in yd
    let step_factor: Numeric = argv[16].parse().unwrap(); // factor to determine step size
    let lattitude: Numeric = argv[17].parse().unwrap(); // Current lattitude in degrees
    let azimuth: Numeric = argv[18].parse().unwrap(); // Bearing relative to north (0 degrees north, 90 east, etc.)
    let zero_distance: Numeric = argv[19].parse().unwrap(); // yards
    let zero_offset: Numeric = argv[20].parse().unwrap(); // Number in inches to 'zero' for
    let zero_tolerance: Numeric = argv[21].parse().unwrap(); // Tolerance in inches for zeroing
    let pitch_offset: Numeric = argv[22].parse().unwrap(); // Pitch offset in MOA for drop output
    let yaw_offset: Numeric = argv[23].parse().unwrap(); // Yaw offset in MOA for drop output
    let output_tolerance: Numeric = argv[24].parse().unwrap(); // Tolerance for LRDU / PBR calculations

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
    let simulation = builder.solve_for(zero_distance, zero_offset, zero_tolerance, pitch_offset, yaw_offset);

    let table = simulation.table(step, range_start, range_end);

    //simulation.zero(zero_distance, &zero_conditions, &drop_table_conditions);
    // println!("{:#?}", simulation.first_zero());

    // println!("+--------------+---------+---------------+-------------+-----------+------------+----------------+--------------+---------+");
    println!(
        "{:>12} {:>7} {:>13} {:>11} {:>9} {:>9} {:>14} {:>12} {:>7}",
        // "| {:>12} | {:>7} | {:>13} | {:>11} | {:>9} | {:>9} | {:>14} | {:>12} | {:>7} |",
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
        // println!("|--------------+---------+---------------+-------------+-----------+------------+----------------+--------------+---------|");
        println!(
            // "| {:>12.0} | {:>7.2} | {:>11.2} {} | {:>9.2} {} | {:>7.2} {} | {:>8.3} {} | {:>14.2} | {:>12.2} | {:>7.3} |",
            "{:>12.0} {:>7.2} {:>11.2} {} {:>9.2} {} {:>7.2} {} {:>8.3} {} {:>14.2} {:>12.2} {:>7.3}",
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
    // println!("+--------------+---------+---------------+-------------+-----------+------------+----------------+--------------+---------+");
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

fn usage(name: &str) {
    println!(
        r#"
        Usage: {}
        velocity (ft/s)
        line_of_sight (degrees)
        scope_height (inches)
        weight (grains)
        caliber (inches)
        bc
        bc type
        wind_velocity (ft/s)
        wind_angle (degrees)
        temp (F)
        pressure (inHg)
        humidity (0-1)
        range_start (yards)
        range_end (yards)
        step (yards)
        timestep_factor
        lattitude
        azimuth
        zero_distance (yards)
        zero_offset (inches)
        zero_tolerance (inches)
        pitch_offset (moa)
        yaw_offset (moa)
        output_tolerance (inches)
        "#,
        name
    );
}
