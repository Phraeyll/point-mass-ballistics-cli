use ordered_float::OrderedFloat;

use super::Adjustment::*;
use rballistics_flat::model::point_mass::*;

pub fn print<'a>(
    table: impl IntoIterator<Item = (OrderedFloat<Numeric>, Packet<'a>)>,
    output_tolerance: Numeric,
) {
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
    for (_, p) in table.into_iter() {
        println!(
            "{:>12.0} {:>8.2} {:>11.2} {} {:>9.2} {} {:>8.2} {} {:>8.2} {} {:>14.2} {:>12.2} {:>8.3}",
            p.distance(),
            p.moa(),
            p.elevation().abs(),
            Elevation(&p.elevation()).adjustment(output_tolerance),
            p.windage().abs(),
            Windage(&p.windage()).adjustment(output_tolerance),
            p.vertical_moa(output_tolerance),
            Elevation(&p.vertical_moa(output_tolerance)).adjustment(output_tolerance),
            p.horizontal_moa(output_tolerance),
            Windage(&p.horizontal_moa(output_tolerance)).adjustment(output_tolerance),
            p.velocity(),
            p.energy(),
            p.time(),
        );
    }
}
