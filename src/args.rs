use crate::printer::print_table;

use std::{error::Error, file, line, stringify, time::Instant};

use clap::{Parser, Subcommand};
use indoc::indoc;
use point_mass_ballistics::{
    drag::{g1, g2, g5, g6, g7, g8, gi, gs, DragFunction},
    output::Measurements,
    simulation::{Simulation, SimulationBuilder},
    units::{radian, Angle, Length, Mass, Pressure, ThermodynamicTemperature, Time, Velocity},
    Numeric,
};

pub type Result<D> = std::result::Result<D, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    name = "Ballistic Solver",
    about = indoc!{r#"
        Produces range table from vector based simulation of Newtons Equations
        using standard, unmodified, point mass model of ballistics.
        Currently, this accounts for drag, gravity, and Coriolis/Eotvos forces.
        This does not currently factor in gyroscopic drift, nor aerodynamic jump.
        Drag tables obtained from JBM Ballistics, and methodologies are mostly from
        Robert L. McCoy's "Modern Exterior Ballistics" ISBN 978-0-7643-3825-0

        The eventual goal of this program is to support modified point mass trajectories
        as well, for factoring in gyroscopic drift and aerodynamic jump (4-DOF models)
    "#}
)]
pub struct Args {
    #[command(subcommand)]
    simulation: SimulationKind,
}

#[derive(Debug, Subcommand)]
enum SimulationKind {
    G1(InnerArgs),
    G2(InnerArgs),
    G5(InnerArgs),
    G6(InnerArgs),
    G7(InnerArgs),
    G8(InnerArgs),
    GI(InnerArgs),
    GS(InnerArgs),
}

#[derive(Debug, Subcommand)]
enum ScenarioKind {
    Zero(Zeroing),
}

#[derive(Debug, Parser)]
struct Zeroing {
    #[command(flatten)]
    conditions: Conditions,

    #[command(flatten)]
    target: Target,
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
struct InnerArgs {
    #[arg(long = "time-step", default_value = "0.00005 s")]
    time_step: Time,

    #[arg(long = "precision", default_value = "1")]
    precision: usize,

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
    scenario: Option<ScenarioKind>,
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

    #[arg(long = "table-tolerance", default_value = "0.005 in")]
    tolerance: Length,
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

    #[arg(long = "shot-angle", default_value = "0.0 degrees")]
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

macro_rules! time {
    ($expr:expr) => {{
        let time = Instant::now();
        match $expr {
            tmp => {
                eprintln!(
                    "[{}:{}] {} = {:#?}",
                    file!(),
                    line!(),
                    stringify!($expr),
                    time.elapsed()
                );
                tmp
            }
        }
    }};
}

impl Args {
    pub fn run(&self) -> Result<()> {
        self.simulation.run()
    }
}

impl SimulationKind {
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

impl InnerArgs {
    pub fn run<D>(&self) -> Result<()>
    where
        D: DragFunction,
    {
        let mut angles = (Angle::new::<radian>(0.0), Angle::new::<radian>(0.0));
        if let Some(ScenarioKind::Zero(ref zeroing)) = self.scenario {
            let simulation = time!(self.simulation::<D>(&zeroing.conditions, None)?);
            angles = time!(self.try_zero(simulation, &zeroing.target)?);
        }
        let simulation = time!(self.simulation::<D>(&self.conditions, Some(angles))?);
        time!(self.print(&simulation));
        Ok(())
    }

    pub fn print<D>(&self, simulation: &Simulation<D>)
    where
        D: DragFunction,
    {
        let mut start = self.table.start;
        let end = self.table.end;
        let step = self.table.step;
        let iter = simulation
            .into_iter()
            .take_while(move |p| p.distance() <= end + step)
            .filter(move |p| {
                if p.distance() >= start {
                    start += step;
                    true
                } else {
                    false
                }
            });
        print_table(
            iter,
            self.table.tolerance,
            self.flags.pretty,
            self.precision,
        );
    }

    fn try_zero<D>(&self, mut simulation: Simulation<D>, target: &Target) -> Result<(Angle, Angle)>
    where
        D: DragFunction,
    {
        Ok(simulation.find_zero_angles(
            target.distance,
            target.height,
            target.offset,
            target.tolerance,
        )?)
    }

    fn simulation<D>(
        &self,
        conditions: &Conditions,
        angles: Option<(Angle, Angle)>,
    ) -> Result<Simulation<D>>
    where
        D: DragFunction,
    {
        let (pitch, yaw) = angles.map_or(Default::default(), |(pitch, yaw)| {
            (pitch + self.scope.pitch, yaw + self.scope.yaw)
        });
        Ok(SimulationBuilder::new()
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
            // Adjust pitch/yaw with value from args, and provided deltas
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
