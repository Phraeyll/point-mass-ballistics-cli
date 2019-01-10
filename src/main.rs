use rballistics_flat::{
    model::point_mass::{builder::*, iter::Output, *},
    Numeric,
};

use std::env;

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() <= 21 {
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
    let tolerance: Numeric = argv[19].parse().unwrap(); // Tolerance in inches for zeroing and LDUR adjustments
    let zero: Numeric = argv[20].parse().unwrap(); // Number to 'zero' for
    let offset: Numeric = argv[21].parse().unwrap(); // Angle offset in MOA for testing

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

    let projectile = Projectile::new(weight, caliber, bc_enum, initial_velocity);
    let scope = Scope::new(scope_height);

    let atmosphere_both = Atmosphere::new(temperature, pressure, humidity);

    let wind = Wind::new(wind_velocity, wind_angle);
    let zero_wind = Wind::new(0.0, 0.0);

    let other = Other::new(los_angle, lattitude, azimuth, None);
    let zero_other = Other::new(0.0, lattitude, azimuth, None);

    let zero_conditions = Conditions::new(&zero_wind, &atmosphere_both, &zero_other);
    let solve_conditions = Conditions::new(&wind, &atmosphere_both, &other);
    let builder = SimulationBuilder::new(
        &projectile,
        &scope,
        &zero_conditions,
        &solve_conditions,
        zero_distance,
        time_step,
    );
    let simulation = builder.solution_simulation(tolerance, zero, offset);

    let table = simulation.drop_table(step, range);

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
    for (distance, p) in table.iter() {
        let (elevation, windage, velocity, energy, moa, time) = (
            p.elevation(),
            p.windage(),
            p.velocity(),
            p.energy(),
            p.moa(),
            p.time(),
        );
        println!(
            "{:>12.0} {:>12.2} {} {:>10.2} {} {:>15.2} {:>13.2} {:>8.2} {:>8.3}",
            distance,
            elevation.abs(),
            Elevation(&elevation).adjustment(tolerance),
            windage.abs(),
            Windage(&windage).adjustment(tolerance),
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
    fn adjustment(&self, tolerance: Numeric) -> char {
        match self {
            Elevation(&m) => {
                if m > -tolerance && m < tolerance {
                    ' '
                } else if m.is_sign_positive() {
                    'D'
                } else {
                    'U'
                }
            }
            Windage(&m) => {
                if m > -tolerance && m < tolerance {
                    ' '
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
        tolerance (inches)
        zero
        offset (moa)
        "#,
        name
    );
}
