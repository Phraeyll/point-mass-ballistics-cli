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
    let los_angle: f64 = argv[2].parse().unwrap(); // degrees
    let scope_height: f64 = argv[3].parse().unwrap(); // inches
    let zero_distance: f64 = argv[4].parse().unwrap(); // yards
    let weight: f64 = argv[5].parse().unwrap(); // grains
    let caliber: f64 = argv[6].parse().unwrap(); // inches
    let bc: f64 = argv[7].parse().unwrap(); // dimensionless
                                            //let drag_table: DragTableKind = argv[8].parse().unwrap(); // Desired drag table (G1, G7, etc.)
    let wind_velocity: f64 = argv[8].parse().unwrap(); // m/h
    let wind_angle: f64 = argv[9].parse().unwrap(); // degrees
    let temperature: f64 = argv[10].parse().unwrap(); // F
    let pressure: f64 = argv[11].parse().unwrap(); // inHg
    let humidity: f64 = argv[12].parse().unwrap(); // dimensionless, percentage
    let range: f64 = argv[13].parse().unwrap(); // range in yd
    let step: f64 = argv[14].parse().unwrap(); // step output in yd
    let step_factor: f64 = argv[15].parse().unwrap(); // factor to determine step size

    let time_step: f64 = 1.0 / (step_factor * initial_velocity);

    let zero_conditions = Conditions::new(
        wind_velocity,
        wind_angle,
        temperature,
        pressure,
        humidity,
        0.0,
    );

    let drop_table_conditions = Conditions::new(
        wind_velocity,
        wind_angle,
        temperature,
        pressure,
        humidity,
        los_angle,
    );

    let model = Model::new(
        weight,
        caliber,
        BallisticCoefficient::G7(bc),
        time_step,
        initial_velocity,
        scope_height,
    );

    let mut simulation = Simulator::new(model, zero_conditions, drop_table_conditions);

    let results = simulation.gimme_drop_table(zero_distance, step, range);

    //simulation.zero(zero_distance, &zero_conditions, &drop_table_conditions);
    // println!("{:#?}", simulation.first_zero());

    println!(
        "{:>12} {:>9} {:>12} {:>15} {:>8} {:>8}",
        "Distance(yd)", "Drop(in)", "Windage(in)", "Velocity(ft/s)", "Time(s)", "Energy(ftlbs)"
    );
    for (distance, drop, windage, velocity, time, energy) in results.0.iter() {
        println!(
            "{:>12.0} {:>9.2} {:>12.2} {:>15.2} {:>8.3} {:>8.2}",
            distance, drop, windage, velocity, time, energy,
        );
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
