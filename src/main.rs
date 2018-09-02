extern crate rballistics_flat;

use rballistics_flat::simulation::*;

use std::env;

fn main() {
    let argv: Vec<String> = env::args().collect();

    if argv.len() <= 15 {
        eprintln!("error: wrong number of args");
        usage(argv[0].to_string());
        return;
    }

    let initial_velocity: f64 = argv[1].parse().unwrap(); // ft/s
    let launch_angle: f64 = argv[2].parse().unwrap(); // degrees
    let scope_height: f64 = argv[3].parse().unwrap(); // inches
    let weight: f64 = argv[4].parse().unwrap(); // grains
    let caliber: f64 = argv[5].parse().unwrap(); // inches
    let bc: f64 = argv[6].parse().unwrap(); // dimensionless
    let drag_table: DragTableKind = argv[7].parse().unwrap(); // Desired drag table (G1, G7, etc.)
    let wind_velocity: f64 = argv[8].parse().unwrap(); // m/h
    let wind_angle: f64 = argv[9].parse().unwrap(); // degrees
    let temperature: f64 = argv[10].parse().unwrap(); // F
    let pressure: f64 = argv[11].parse().unwrap(); // inHg
    let humidity: f64 = argv[12].parse().unwrap(); // dimensionless, percentage
    let range: f64 = argv[13].parse().unwrap(); // range in yd
    let step: f64 = argv[14].parse().unwrap(); // step output in yd
    let step_factor: f64 = argv[15].parse().unwrap(); // factor to determin step size

    let time_step: f64 = 1.0 / (step_factor * initial_velocity);

    let mut simulation = PointMassModel::new(
        weight,
        caliber,
        bc,
        initial_velocity,
        launch_angle,
        scope_height,
        drag_table,
        time_step,
        wind_velocity,
        wind_angle,
        temperature,
        pressure,
        humidity,
    );

    println!("time(s), velocity(ft/s), distance(yd), drop(in), windage(in)");
    let mut current_step: f64 = 0.0;
    while let Some(distance) = simulation.next() {
        if distance > current_step {
            println!(
                "{} {} {} {} {}",
                simulation.time(),
                simulation.relative_velocity(),
                simulation.relative_distance(),
                simulation.relative_drop(),
                simulation.relative_windage(),
            );
            current_step += step;
        }
        if distance > range {
            break;
        }
    }
}

fn usage(name: String) {
    println!(
        r#"
        Usage: {}
        velocity (ft/s)
        launch_angle (deg)
        weight (gr)
        caliber (in)
        bc
        dragtable
        wind_velocity (ft/s)
        wind_angle (deg)
        temp (F)
        pressure (inHg)
        humidity (0-1)
        range (yd)
        step (yd)
        timestep_factor
        "#,
        name
    );
}
