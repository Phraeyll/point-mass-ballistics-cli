use crate::formatter::write_table;

use std::{
    error::Error,
    io::{stdout, BufWriter},
};

use clap::{Parser, Subcommand};
use point_mass_ballistics::{
    drag::{g1, g2, g5, g6, g7, g8, gi, gs, DragFunction},
    output::Measurements,
    simulation::{Simulation, SimulationBuilder},
    units::{Angle, Length, Mass, Pressure, ThermodynamicTemperature, Time, Velocity},
    Numeric,
};

pub type Result<D> = std::result::Result<D, Box<dyn Error>>;

macro_rules! time {
    ($expr:expr) => {{
        let time = std::time::Instant::now();
        match $expr {
            tmp => {
                eprintln!(
                    "[{}:{}] {} = {:#?}",
                    std::file!(),
                    std::line!(),
                    std::stringify!($expr),
                    time.elapsed(),
                );
                tmp
            }
        }
    }};
}

#[derive(Debug, Parser)]
#[command(author, version, about, name = "Ballistic Solver")]
pub struct Args {
    #[command(subcommand)]
    model: Model,
}

#[derive(Debug, Subcommand)]
enum Model {
    #[command(about = "drag model")]
    G1(ModelArgs),

    #[command(about = "drag model")]
    G2(ModelArgs),

    #[command(about = "drag model")]
    G5(ModelArgs),

    #[command(about = "drag model")]
    G6(ModelArgs),

    #[command(about = "drag model")]
    G7(ModelArgs),

    #[command(about = "drag model")]
    G8(ModelArgs),

    #[command(about = "drag model")]
    GI(ModelArgs),

    #[command(about = "drag model")]
    GS(ModelArgs),
}

#[derive(Debug, Parser)]
struct ModelArgs {
    #[arg(long = "time-step", default_value = "0.00005 s")]
    time_step: Time,

    #[arg(long = "precision", default_value = "1")]
    precision: usize,

    #[arg(long = "simulations", default_value = "1")]
    simulations: usize,

    #[command(flatten)]
    flags: Flags,

    #[command(flatten)]
    table: Table,

    #[command(flatten)]
    projectile: Projectile,

    #[command(flatten)]
    scope: Scope,

    #[command(flatten)]
    conditions: Conditions,

    #[command(subcommand)]
    scenario: Option<Scenario>,
}

#[derive(Debug, Subcommand)]
enum Scenario {
    Zero {
        #[command(flatten)]
        conditions: Option<Conditions>,

        #[command(flatten)]
        target: Target,
    },
}

#[derive(Debug, Parser)]
struct Flags {
    #[arg(long = "disable-drag")]
    disable_drag: bool,

    #[arg(long = "disable-coriolis")]
    disable_coriolis: bool,

    #[arg(long = "disable-gravity")]
    disable_gravity: bool,

    #[arg(long = "pretty")]
    pretty: bool,
}

#[derive(Debug, Parser)]
struct Table {
    #[arg(long = "table-start", default_value = "0.0 yd")]
    start: Length,

    #[arg(long = "table-end", default_value = "1000.0 yd")]
    end: Length,

    #[arg(long = "table-step", default_value = "100.0 yd")]
    step: Length,
}

#[derive(Debug, Parser)]
struct Projectile {
    #[arg(long = "velocity", default_value = "3000.0 ft/s")]
    velocity: Velocity,

    #[arg(long = "mass", default_value = "220.0 gr")]
    mass: Mass,

    #[arg(long = "caliber", default_value = "0.308 in")]
    caliber: Length,

    #[arg(long = "bc", default_value = "0.5")]
    bc: Numeric,
}

#[derive(Debug, Parser)]
struct Scope {
    #[arg(long = "scope-height", default_value = "1.5 in")]
    height: Length,

    #[arg(long = "scope-offset", default_value = "0.0 in")]
    offset: Length,

    #[arg(long = "scope-pitch", default_value = "0.0 degrees")]
    pitch: Angle,

    #[arg(long = "scope-yaw", default_value = "0.0 degrees")]
    yaw: Angle,

    #[arg(long = "scope-cant", default_value = "0.0 degrees")]
    cant: Angle,
}

#[derive(Debug, Parser)]
struct Conditions {
    #[command(flatten)]
    atmosphere: Atmosphere,

    #[command(flatten)]
    wind: Wind,

    #[command(flatten)]
    shooter: Shooter,
}

#[derive(Debug, Parser)]
struct Wind {
    #[arg(long = "wind-speed", default_value = "0.0 mi/h")]
    speed: Velocity,

    #[arg(long = "wind-direction", default_value = "0.0 degrees")]
    direction: Angle,
}

#[derive(Debug, Parser)]
struct Atmosphere {
    #[arg(long = "temperature", default_value = "59 degree Fahrenheit")]
    temperature: ThermodynamicTemperature,

    #[arg(long = "pressure", default_value = "29.92 in Hg")]
    pressure: Pressure,

    #[arg(long = "humidity", default_value = "0.0")]
    humidity: Numeric,
}

#[derive(Debug, Parser)]
struct Shooter {
    #[arg(long = "lattitude", default_value = "0.0 degrees")]
    lattitude: Angle,

    #[arg(long = "bearing", default_value = "0.0 degrees")]
    bearing: Angle,

    #[arg(long = "incline", default_value = "0.0 degrees")]
    incline: Angle,
}

#[derive(Debug, Parser)]
struct Target {
    #[arg(long = "target-distance", default_value = "100.0 yd")]
    distance: Length,

    #[arg(long = "target-height", default_value = "0.0 in")]
    height: Length,

    #[arg(long = "target-offset", default_value = "0.0 in")]
    offset: Length,

    #[arg(long = "target-tolerance", default_value = "0.001 in")]
    tolerance: Length,
}

impl Args {
    pub fn run(&self) -> Result<()> {
        self.model.run()
    }
}

impl Model {
    pub fn run(&self) -> Result<()> {
        match *self {
            Self::G1(ref inner) => inner.run::<g1::Drag>(),
            Self::G2(ref inner) => inner.run::<g2::Drag>(),
            Self::G5(ref inner) => inner.run::<g5::Drag>(),
            Self::G6(ref inner) => inner.run::<g6::Drag>(),
            Self::G7(ref inner) => inner.run::<g7::Drag>(),
            Self::G8(ref inner) => inner.run::<g8::Drag>(),
            Self::GI(ref inner) => inner.run::<gi::Drag>(),
            Self::GS(ref inner) => inner.run::<gs::Drag>(),
        }
    }
}

impl ModelArgs {
    pub fn run<D>(&self) -> Result<()>
    where
        D: DragFunction,
    {
        let angles = Some(match self.scenario {
            Some(Scenario::Zero {
                ref conditions,
                ref target,
            }) => {
                let conditions = conditions.as_ref().unwrap_or(&self.conditions);
                let mut simulation = time!(self.simulation::<D>(conditions, None)?);
                simulation.find_zero_angles(
                    target.distance,
                    target.height,
                    target.offset,
                    target.tolerance,
                )?
            }
            None => Default::default(),
        });

        let simulation = time!(self.simulation::<D>(&self.conditions, angles)?);
        let mut writer = BufWriter::new(stdout().lock());
        for _ in 0..self.simulations {
            let mut next = self.table.start;
            let end = self.table.end;
            let step = self.table.step;
            let iter = simulation
                .into_iter()
                .take_while(|p| p.distance() <= end + step)
                .filter(|p| {
                    if p.distance() >= next {
                        next += step;
                        true
                    } else {
                        false
                    }
                });
            time!(write_table(
                &mut writer,
                iter,
                self.flags.pretty,
                self.precision
            ));
        }
        Ok(())
    }

    fn simulation<D>(
        &self,
        conditions: &Conditions,
        angles: Option<(Angle, Angle)>,
    ) -> Result<Simulation<D>>
    where
        D: DragFunction,
    {
        // pitch/yaw with value from args, and provided deltas if post zeroing
        let (pitch, yaw) = angles.map_or(Default::default(), |(pitch, yaw)| {
            (pitch + self.scope.pitch, yaw + self.scope.yaw)
        });
        Ok(SimulationBuilder::new()
            // Flags
            .set_time_step(self.time_step)?
            .use_coriolis(!self.flags.disable_coriolis)
            .use_drag(!self.flags.disable_drag)
            .use_gravity(!self.flags.disable_gravity)
            // Projectile
            .set_bc(self.projectile.bc)?
            .set_velocity(self.projectile.velocity)?
            .set_mass(self.projectile.mass)?
            .set_caliber(self.projectile.caliber)?
            // Scope
            .set_scope_height(self.scope.height)
            .set_scope_offset(self.scope.offset)
            .set_scope_roll(self.scope.cant)
            .set_scope_pitch(pitch)
            .set_scope_yaw(yaw)
            // Atmosphere
            .set_temperature(conditions.atmosphere.temperature)?
            .set_pressure(conditions.atmosphere.pressure)?
            .set_humidity(conditions.atmosphere.humidity)?
            // Wind
            .set_wind_speed(conditions.wind.speed)?
            .set_wind_direction(conditions.wind.direction)?
            // Shooter
            .set_incline(conditions.shooter.incline)?
            .set_lattitude(conditions.shooter.lattitude)?
            .set_bearing(conditions.shooter.bearing)?
            .init())
    }
}
