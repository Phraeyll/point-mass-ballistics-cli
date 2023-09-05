use std::io::Write;

use point_mass_ballistics::{
    output::Measurements,
    units::{
        acceleration::foot_per_second_squared, angle::minute, energy::foot_pound, length::inch,
        length::yard, time::second, velocity::foot_per_second,
    },
};

pub fn write_table<W>(
    writer: &mut W,
    iter: impl IntoIterator<Item = impl Measurements>,
    pretty: bool,
    precision: usize,
) where
    W: Write,
{
    let (rs, fs, eol) = if pretty {
        (
            "+--------------+---------------+------------+-------------+------------+----------------+------------+--------------+----------------------+----------+\n",
            "| ",
            " |\n",
        )
    } else {
        ("", "", "\n")
    };
    let distance = "Distance(yd)";
    let elevation = "Drop(in)";
    let elevation_moa = "Drop(MOA)";
    let windage = "Wind(in)";
    let windage_moa = "Wind(MOA)";
    let velocity = "Velocity(ft/s)";
    let mach = "Mach";
    let energy = "Energy(ftlb)";
    let acceleration = "Acceleration(ft/s^2)";
    let time = "Time(s)";
    writeln!(
        writer,
        "\
        {rs}\
        {fs}{distance:>12} \
        {fs}{elevation:>13} \
        {fs}{elevation_moa:>10} \
        {fs}{windage:>11} \
        {fs}{windage_moa:>10} \
        {fs}{velocity:>14} \
        {fs}{mach:>10} \
        {fs}{energy:>12} \
        {fs}{acceleration:>20} \
        {fs}{time:>8}{eol}\
        {rs}\
        "
    )
    .unwrap();
    for p in iter {
        let distance = p.distance().get::<yard>();
        let elevation = p.elevation().get::<inch>();
        let elevation_moa = p.vertical_angle().get::<minute>();
        let windage = p.windage().get::<inch>();
        let windage_moa = p.horizontal_angle().get::<minute>();
        let velocity = p.velocity().get::<foot_per_second>();
        let mach = p.mach().value;
        let energy = p.energy().get::<foot_pound>();
        let acceleration = p.acceleration().get::<foot_per_second_squared>();
        let time = p.time().get::<second>();
        writeln!(
            writer,
            "\
            {fs}{distance:>12.precision$} \
            {fs}{elevation:>13.precision$} \
            {fs}{elevation_moa:>10.precision$} \
            {fs}{windage:>11.precision$} \
            {fs}{windage_moa:>10.precision$} \
            {fs}{velocity:>14.precision$} \
            {fs}{mach:>10.precision$} \
            {fs}{energy:>12.precision$} \
            {fs}{acceleration:>20.precision$} \
            {fs}{time:>8.3}{eol}\
            {rs}\
            "
        )
        .unwrap();
    }
}
