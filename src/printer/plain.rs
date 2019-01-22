use super::helper::Adjustment::*;
use point_mass_ballistics::model::*;

pub fn print<'s>(table: impl IntoIterator<Item = Packet<'s>>, output_tolerance: Numeric) {
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
    for p in table.into_iter() {
        println!(
            "{:>12.0} {:>8.2} {:>11.2} {} {:>9.2} {} {:>8.2} {} {:>8.2} {} {:>14.2} {:>12.2} {:>8.3}",
            p.distance(),
            p.moa(),
            // dbg!(p.moa()),
            p.elevation().abs(),
            Elevation(&p.elevation()).adjustment(output_tolerance),
            p.windage().abs(),
            Windage(&p.windage()).adjustment(output_tolerance),
            p.vertical_moa(output_tolerance).abs(),
            Elevation(&p.elevation()).adjustment(output_tolerance),
            p.horizontal_moa(output_tolerance).abs(),
            Windage(&p.windage()).adjustment(output_tolerance),
            p.velocity(),
            p.energy(),
            p.time(),
        );
    }
}
