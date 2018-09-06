extern crate rballistics_flat;

use rballistics_flat::simulation::*;

use std::env;

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() <= 16 {
        eprintln!("error: wrong number of args");
        usage(argv[0].to_string());
        return;
    }

    let initial_velocity: f64 = argv[1].parse().unwrap(); // ft/s
    let los_angle: f64 = argv[2].parse().unwrap(); // degrees
    let scope_height: f64 = argv[3].parse().unwrap(); // inches
    let zero_distance: f64 = argv[4].parse().unwrap(); // yards
    let weight: f64 = argv[5].parse().unwrap(); // grains
    let caliber: f64 = argv[6].parse().unwrap(); // inches
    let bc: f64 = argv[7].parse().unwrap(); // dimensionless
    let drag_table: DragTableKind = argv[8].parse().unwrap(); // Desired drag table (G1, G7, etc.)
    let wind_velocity: f64 = argv[9].parse().unwrap(); // m/h
    let wind_angle: f64 = argv[10].parse().unwrap(); // degrees
    let temperature: f64 = argv[11].parse().unwrap(); // F
    let pressure: f64 = argv[12].parse().unwrap(); // inHg
    let humidity: f64 = argv[13].parse().unwrap(); // dimensionless, percentage
    let range: f64 = argv[14].parse().unwrap(); // range in yd
    let step: f64 = argv[15].parse().unwrap(); // step output in yd
    let step_factor: f64 = argv[16].parse().unwrap(); // factor to determine step size

    let time_step: f64 = 1.0 / (step_factor * initial_velocity);

    let mut simulation = PointMassModel::new(
        weight,
        caliber,
        bc,
        initial_velocity,
        scope_height,
        los_angle,
        drag_table,
        time_step,
        wind_velocity,
        wind_angle,
        temperature,
        pressure,
        humidity,
    );
    simulation.zero(zero_distance);
    // println!("{:#?}", simulation.first_zero());

    println!(
        "{:>12} {:>9} {:>12} {:>15} {:>8}",
        "Distance(yd)",
        "Drop(in)",
        "Windage(in)",
        "Velocity(ft/s)",
        "Time(s)"
    );
    let mut current_step: f64 = 0.0;
    for b in simulation.iter() {
        if b.distance() > current_step {
            println!(
                "{:>12.0} {:>9.1} {:>12.1} {:>15.2} {:>8.3}",
                b.distance(),
                b.drop(),
                b.windage(),
                b.velocity(),
                b.time(),
            );
            current_step += step;
        }
        if b.distance() > range {
            break;
        }
    }
}

fn usage(name: String) {
    println!(
        r#"
        Usage: {}
        velocity (ft/s)
        launch_angle (degrees)
        scope_height (inches)
        zero_range (yards)
        weight (grains)
        caliber (inches)
        bc
        dragtable
        wind_velocity (ft/s)
        wind_angle (degrees)
        temp (F)
        pressure (inHg)
        humidity (0-1)
        range (yards)
        step (yards)
        timestep_factor
        "#,
        name
    );
}
