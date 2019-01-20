use ordered_float::OrderedFloat;

use super::Adjustment::*;
use rballistics_flat::model::point_mass::*;

pub fn print<'a>(
    table: impl IntoIterator<Item = (OrderedFloat<Numeric>, Packet<'a>)>,
    output_tolerance: Numeric,
) {
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
    for (_, p) in table.into_iter() {
        let vertical_adjustment = Elevation(&p.elevation()).adjustment(output_tolerance);
        let horizontal_adjustment = Windage(&p.windage()).adjustment(output_tolerance);
        println!("+--------------+----------+---------------+-------------+------------+------------+----------------+--------------+----------+");
        println!(
            "| {:>12.0} | {:>8.2} | {:>11.2} {} | {:>9.2} {} | {:>8.2} {} | {:>8.2} {} | {:>14.2} | {:>12.2} | {:>8.3} |",
            p.distance(),
            p.moa(),
            p.elevation().abs(),
            vertical_adjustment,
            p.windage().abs(),
            horizontal_adjustment,
            p.vertical_moa(),
            vertical_adjustment,
            p.horizontal_moa(),
            horizontal_adjustment,
            p.velocity(),
            p.energy(),
            p.time(),
        );
    }
    println!("+--------------+----------+---------------+-------------+------------+------------+----------------+--------------+----------+");
}
