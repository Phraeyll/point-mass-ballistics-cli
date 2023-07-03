use point_mass_ballistics::{
    output::Measurements,
    units::{foot_per_second, foot_pound, inch, moa, second, yard, Length},
};

pub fn print_table<I>(table: I, output_tolerance: Length, pretty: bool, precision: usize)
where
    I: IntoIterator,
    <I as IntoIterator>::Item: Measurements,
{
    let (div, lpad, eol) = if pretty {
        (
            "+--------------+----------+---------------+-------------+------------+------------+----------------+--------------+----------+\n",
            "| ",
            " |\n",
        )
    } else {
        ("", "", "\n")
    };
    print!(
        "\
        {div}\
        {lpad}{:>12} \
        {lpad}{:>13} \
        {lpad}{:>10} \
        {lpad}{:>11} \
        {lpad}{:>10} \
        {lpad}{:>14} \
        {lpad}{:>12} \
        {lpad}{:>8}{eol}\
        {div}\
        ",
        "Distance(yd)",
        "Drop(in)",
        "Drop(MOA)",
        "Wind(in)",
        "Wind(MOA)",
        "Velocity(ft/s)",
        "Energy(ftlb)",
        "Time(s)",
        lpad = lpad,
        eol = eol,
        div = div,
    );
    for p in table.into_iter() {
        print!(
            "\
            {lpad}{:>12.0} \
            {lpad}{:>13.precision$} \
            {lpad}{:>10.precision$} \
            {lpad}{:>11.precision$} \
            {lpad}{:>10.precision$} \
            {lpad}{:>14.precision$} \
            {lpad}{:>12.precision$} \
            {lpad}{:>8.3}{eol}\
            {div}\
            ",
            p.distance().get::<yard>(),
            p.elevation().get::<inch>(),
            p.vertical_angle(output_tolerance).get::<moa>(),
            p.windage().get::<inch>(),
            p.horizontal_angle(output_tolerance).get::<moa>(),
            p.velocity().get::<foot_per_second>(),
            p.energy().get::<foot_pound>(),
            p.time().get::<second>(),
            lpad = lpad,
            eol = eol,
            div = div,
        );
    }
}
