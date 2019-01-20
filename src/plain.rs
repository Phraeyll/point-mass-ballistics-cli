use rballistics_flat::model::point_mass::*;
use super::Adjustment::*;
use ordered_float::OrderedFloat;
pub fn print<'a>(table: impl IntoIterator<Item=(OrderedFloat<Numeric>, Packet<'a>)>, output_tolerance: Numeric) {
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
    for (distance, p) in table.into_iter() {
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
