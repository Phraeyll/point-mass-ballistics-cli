use self::Adjustment::*;

use point_mass_ballistics::{
    foot_per_second, foot_pound, inch, moa, second, yard, Length, Measurements,
};

// pub mod plain;
// pub mod pretty;

#[derive(Clone, Copy)]
pub enum Adjustment {
    Elevation(Length),
    Windage(Length),
}

// Show needed adjustments to correct shot
impl Adjustment {
    pub fn adjustment(&self, tolerance: Length) -> char {
        let (n, positive, negative) = match *self {
            Self::Elevation(n) => (n, 'D', 'U'),
            Self::Windage(n) => (n, 'L', 'R'),
        };
        if n > tolerance {
            positive
        } else if n < -tolerance {
            negative
        } else {
            '*'
        }
    }
}

pub fn print<I>(table: I, output_tolerance: Length, pretty: bool)
where
    I: IntoIterator,
    <I as IntoIterator>::Item: Measurements,
{
    let (div, start, end) = if pretty {
        (
            "+--------------+----------+---------------+-------------+------------+------------+----------------+--------------+----------+\n",
            "| ",
            " |",
        )
    } else {
        ("", "", "")
    };
    println!(
        "{div}{start}{:>12} {start}{:>8} {start}{:>13} {start}{:>11} {start}{:>10} {start}{:>10} {start}{:>14} {start}{:>12} {start}{:>8}{end}",
        "Distance(yd)",
        "MOA",
        "Elevation(in)",
        "Windage(in)",
        "Vertical",
        "Horizontal",
        "Velocity(ft/s)",
        "Energy(ftlb)",
        "Time(s)",
        start=start,
        end=end,
        div=div,
    );
    for p in table.into_iter() {
        println!(
            "{div}{start}{:>12.0} {start}{:>8.2} {start}{:>11.2} {} {start}{:>9.2} {} {start}{:>8.2} {} {start}{:>8.2} {} {start}{:>14.2} {start}{:>12.2} {start}{:>8.3}{end}",
            p.distance().get::<yard>(),
            p.angle().get::<moa>(),
            p.elevation().get::<inch>().abs(),
            Elevation(p.elevation()).adjustment(output_tolerance),
            p.windage().get::<inch>().abs(),
            Windage(p.windage()).adjustment(output_tolerance),
            p.vertical_angle(output_tolerance).get::<moa>().abs(),
            Elevation(p.elevation()).adjustment(output_tolerance),
            p.horizontal_angle(output_tolerance).get::<moa>().abs(),
            Windage(p.windage()).adjustment(output_tolerance),
            p.velocity().get::<foot_per_second>(),
            p.energy().get::<foot_pound>(),
            p.time().get::<second>(),
            start=start,
            end=end,
            div=div,
        );
    }
    print!("{}", div);
}
