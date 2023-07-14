use point_mass_ballistics::{
    output::Measurements,
    units::{
        acceleration::foot_per_second_squared, angle::minute, energy::foot_pound, length::inch,
        length::yard, time::second, velocity::foot_per_second, Length,
    },
};

pub fn print_table(
    iter: impl IntoIterator<Item = impl Measurements>,
    output_tolerance: Length,
    pretty: bool,
    precision: usize,
) {
    let (rs, fs, eol) = if pretty {
        (
            "+--------------+---------------+------------+-------------+------------+----------------+------------+--------------+----------------------+----------+\n",
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
        {fs}{:>10} \
        {fs}{:>12} \
        {fs}{:>20} \
        {fs}{:>8}{eol}\
        {rs}\
        ",
        "Distance(yd)",
        "Drop(in)",
        "Drop(MOA)",
        "Wind(in)",
        "Wind(MOA)",
        "Velocity(ft/s)",
        "Mach",
        "Energy(ftlb)",
        "Acceleration(ft/s^2)",
        "Time(s)",
    );
    for p in iter {
        print!(
            "\
            {fs}{:>12.precision$} \
            {fs}{:>13.precision$} \
            {fs}{:>10.precision$} \
            {fs}{:>11.precision$} \
            {fs}{:>10.precision$} \
            {fs}{:>14.precision$} \
            {fs}{:>10.precision$} \
            {fs}{:>12.precision$} \
            {fs}{:>20.precision$} \
            {fs}{:>8.3}{eol}\
            {rs}\
            ",
            p.distance().get::<yard>(),
            p.elevation().get::<inch>(),
            p.vertical_angle(output_tolerance).get::<minute>(),
            p.windage().get::<inch>(),
            p.horizontal_angle(output_tolerance).get::<minute>(),
            p.velocity().get::<foot_per_second>(),
            p.mach().value,
            p.energy().get::<foot_pound>(),
            p.acceleration().get::<foot_per_second_squared>(),
            p.time().get::<second>(),
        );
    }
}
