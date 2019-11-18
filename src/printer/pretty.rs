use super::helper::Adjustment::*;
use point_mass_ballistics::{
    foot_per_second, foot_pound, inch, moa, second, yard, Length, Measurements,
};

pub fn print<I>(table: I, output_tolerance: Length)
where
    I: IntoIterator,
    <I as IntoIterator>::Item: Measurements,
{
    let divider = "+--------------+----------+---------------+-------------+------------+------------+----------------+--------------+----------+";
    println!("{}", divider);
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
    for p in table.into_iter() {
        println!("{}", divider);
        println!(
            "| {:>12.0} | {:>8.2} | {:>11.2} {} | {:>9.2} {} | {:>8.2} {} | {:>8.2} {} | {:>14.2} | {:>12.2} | {:>8.3} |",
            p.distance().get::<yard>(),
            p.moa().get::<moa>(),
            p.elevation().get::<inch>().abs(),
            Elevation(&p.elevation()).adjustment(output_tolerance),
            p.windage().get::<inch>().abs(),
            Windage(&p.windage()).adjustment(output_tolerance),
            p.vertical_moa(output_tolerance).get::<moa>().abs(),
            Elevation(&p.elevation()).adjustment(output_tolerance),
            p.horizontal_moa(output_tolerance).get::<moa>().abs(),
            Windage(&p.windage()).adjustment(output_tolerance),
            p.velocity().get::<foot_per_second>(),
            p.energy().get::<foot_pound>(),
            p.time().get::<second>(),
        );
    }
    println!("{}", divider);
}
