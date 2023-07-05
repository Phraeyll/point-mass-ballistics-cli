use point_mass_ballistics::{
    output::Measurements,
    units::{foot_per_second, foot_pound, inch, moa, second, yard, Length},
};

pub fn print_table(
    table: impl IntoIterator<Item = impl Measurements>,
    output_tolerance: Length,
    pretty: bool,
    precision: usize,
) {
    let (rs, fs, eol) = if pretty {
        (
            "+--------------+---------------+------------+-------------+------------+----------------+--------------+----------+\n",
            "| ",
            " |\n",
        )
    } else {
        ("", "", "\n")
    };
    print!(
        "\
        {rs}\
        {fs}{:>12} \
        {fs}{:>13} \
        {fs}{:>10} \
        {fs}{:>11} \
        {fs}{:>10} \
        {fs}{:>14} \
        {fs}{:>12} \
        {fs}{:>8}{eol}\
        {rs}\
        ",
        "Distance(yd)",
        "Drop(in)",
        "Drop(MOA)",
        "Wind(in)",
        "Wind(MOA)",
        "Velocity(ft/s)",
        "Energy(ftlb)",
        "Time(s)",
    );
    for p in table.into_iter() {
        print!(
            "\
            {fs}{:>12.0} \
            {fs}{:>13.precision$} \
            {fs}{:>10.precision$} \
            {fs}{:>11.precision$} \
            {fs}{:>10.precision$} \
            {fs}{:>14.precision$} \
            {fs}{:>12.precision$} \
            {fs}{:>8.3}{eol}\
            {rs}\
            ",
            p.distance().get::<yard>(),
            p.elevation().get::<inch>(),
            p.vertical_angle(output_tolerance).get::<moa>(),
            p.windage().get::<inch>(),
            p.horizontal_angle(output_tolerance).get::<moa>(),
            p.velocity().get::<foot_per_second>(),
            p.energy().get::<foot_pound>(),
            p.time().get::<second>(),
        );
    }
}
