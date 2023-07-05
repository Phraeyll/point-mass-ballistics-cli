use crate::printer::print_table;

use std::{error::Error, file, line, stringify, time::Instant};

use clap::Parser;
use indoc::indoc;
use point_mass_ballistics::{
    output::Measurements,
    projectiles::{g1, g2, g5, g6, g7, g8, gi, gs, DragFunction},
    simulation::{Simulation, SimulationBuilder},
    units::{
        radian, Acceleration, Angle, Length, Mass, Pressure, ThermodynamicTemperature, Time,
        Velocity,
    },
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

#[derive(Debug, Parser)]
pub enum SimulationKind {
    G1(InnerArgs),
    G2(InnerArgs),
    G5(InnerArgs),
    G6(InnerArgs),
    G7(InnerArgs),
    G8(InnerArgs),
    GI(InnerArgs),
    GS(InnerArgs),
}

#[derive(Debug, Parser)]
pub struct InnerArgs {
    #[arg(long = "time-step", default_value = "0.00005 s")]
    time_step: Time,

    #[command(flatten)]
    flags: Flags,

    #[command(flatten)]
    table: Table,

    #[command(flatten)]
    projectile: ProjectileArg,

    #[command(flatten)]
    scope: Scope,

    #[command(flatten)]
    firing: Firing,

    #[command(flatten)]
    zeroing: Zeroing,
}
#[derive(Debug, Parser)]
struct Flags {
    #[arg(long = "flat")]
    flat: bool,

    #[arg(long = "disable-drag")]
    disable_drag: bool,

    #[arg(long = "disable-coriolis")]
    disable_coriolis: bool,

    #[arg(long = "disable-gravity")]
    disable_gravity: bool,

    #[arg(long = "pretty")]
    pretty: bool,

    #[arg(long = "precision", default_value = "1")]
    precision: usize,
}
#[derive(Debug, Parser)]
struct Table {
    #[arg(long = "start", default_value = "0.0 yd")]
    table_start: Length,

    #[arg(long = "end", default_value = "1000.0 yd")]
    table_end: Length,

    #[arg(long = "step", default_value = "100.0 yd")]
    table_step: Length,

    #[arg(long = "table-tolerance", default_value = "0.005 in")]
    table_tolerance: Length,
}
#[derive(Debug, Parser)]
struct ProjectileArg {
    #[arg(long = "initial-velocity")]
    projectile_velocity: Option<Velocity>,

    #[arg(long = "mass")]
    projectile_mass: Option<Mass>,

    #[arg(long = "caliber")]
    projectile_caliber: Option<Length>,

    #[arg(long = "bc")]
    projectile_bc: Option<Numeric>,
}
#[derive(Debug, Parser)]
struct Scope {
    #[arg(long = "scope-height")]
    scope_height: Option<Length>,

    #[arg(long = "scope-offset")]
    scope_offset: Option<Length>,

    #[arg(long = "scope-pitch")]
    scope_pitch: Option<Angle>,

    #[arg(long = "scope-yaw")]
    scope_yaw: Option<Angle>,

    #[arg(long = "scope-cant")]
    scope_cant: Option<Angle>,
}
#[derive(Debug, Parser)]
struct Firing {
    #[command(flatten)]
    firing_atmosphere: FiringAtmosphere,

    #[command(flatten)]
    firing_wind: FiringWind,

    #[command(flatten)]
    firing_shooter: FiringShooter,
}
#[derive(Debug, Parser)]
struct FiringWind {
    #[arg(long = "wind-speed")]
    firing_wind_speed: Option<Velocity>,

    #[arg(long = "wind-angle")]
    firing_wind_angle: Option<Angle>,
}
#[derive(Debug, Parser)]
struct FiringAtmosphere {
    #[arg(long = "temperature")]
    firing_atmosphere_temperature: Option<ThermodynamicTemperature>,

    #[arg(long = "pressure")]
    firing_atmosphere_pressure: Option<Pressure>,

    #[arg(long = "humidity")]
    firing_atmosphere_humidity: Option<Numeric>,
}
#[derive(Debug, Parser)]
struct FiringShooter {
    #[arg(long = "lattitude")]
    firing_shooter_lattitude: Option<Angle>,

    #[arg(long = "bearing")]
    firing_shooter_bearing: Option<Angle>,

    #[arg(long = "shot-angle")]
    firing_shooter_angle: Option<Angle>,

    #[arg(long = "gravity")]
    firing_shooter_gravity: Option<Acceleration>,
}
#[derive(Debug, Parser)]
struct Zeroing {
    #[command(flatten)]
    zeroing_wind: ZeroingWind,

    #[command(flatten)]
    zeroing_atmosphere: ZeroingAtmosphere,

    #[command(flatten)]
    zeroing_shooter: ZeroingShooter,

    #[command(flatten)]
    zeroing_target: ZeroingTarget,
}
#[derive(Debug, Parser)]
struct ZeroingWind {
    #[arg(long = "zeroing-wind-speed")]
    zeroing_wind_speed: Option<Velocity>,

    #[arg(long = "zeroing-wind-angle")]
    zeroing_wind_angle: Option<Angle>,
}
#[derive(Debug, Parser)]
struct ZeroingAtmosphere {
    #[arg(long = "zeroing-temperature")]
    zeroing_atmosphere_temperature: Option<ThermodynamicTemperature>,

    #[arg(long = "zeroing-pressure")]
    zeroing_atmosphere_pressure: Option<Pressure>,

    #[arg(long = "zeroing-humidity")]
    zeroing_atmosphere_humidity: Option<Numeric>,
}
#[derive(Debug, Parser)]
struct ZeroingShooter {
    #[arg(long = "zeroing-lattitude")]
    zeroing_shooter_lattitude: Option<Angle>,

    #[arg(long = "zeroing-bearing")]
    zeroing_shooter_bearing: Option<Angle>,

    #[arg(long = "zeroing-shot-angle")]
    zeroing_shooter_angle: Option<Angle>,

    #[arg(long = "zeroing-gravity")]
    zeroing_shooter_gravity: Option<Acceleration>,
}
#[derive(Debug, Parser)]
struct ZeroingTarget {
    #[arg(long = "zeroing-target-distance", default_value = "200.0 yd")]
    zeroing_target_distance: Length,

    #[arg(long = "zeroing-target-height", default_value = "0.0 in")]
    zeroing_target_height: Length,

    #[arg(long = "zeroing-target-offset", default_value = "0.0 in")]
    zeroing_target_offset: Length,

    #[arg(long = "zeroing-target-tolerance", default_value = "0.001 in")]
    zeroing_target_tolerance: Length,
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
        if !self.flags.flat {
            let zero_builder = time!(self.shared_params::<D>()?);
            let zero_simulation = time!(self.zero_scenario(zero_builder)?);
            angles = time!(self.try_zero(zero_simulation)?);
        };
        let firing_builder = time!(self.shared_params::<D>()?);
        let firing_simulation = time!(self.firing_scenario(firing_builder, angles.0, angles.1)?);
        time!(self.print(&firing_simulation));
        Ok(())
    }
    pub fn print<D>(&self, simulation: &Simulation<D>)
    where
        D: DragFunction,
    {
        let output_tolerance = self.table.table_tolerance;
        print_table(
            self.table_gen(simulation),
            output_tolerance,
            self.flags.pretty,
            self.flags.precision,
        );
    }
    pub fn table_gen<'s, D>(
        &self,
        simulation: &'s Simulation<D>,
    ) -> impl IntoIterator<Item = impl Measurements + 's> + 's
    where
        D: DragFunction,
    {
        let mut start = self.table.table_start;
        let end = self.table.table_end;
        let step = self.table.table_step;
        simulation
            .into_iter()
            .take_while(move |p| p.distance() <= end + step)
            .filter(move |p| {
                if p.distance() >= start {
                    start += step;
                    true
                } else {
                    false
                }
            })
    }
    pub fn try_zero<D>(&self, mut simulation: Simulation<D>) -> Result<(Angle, Angle)>
    where
        D: DragFunction,
    {
        Ok(simulation.find_zero_angles(
            self.zeroing.zeroing_target.zeroing_target_distance,
            self.zeroing.zeroing_target.zeroing_target_height,
            self.zeroing.zeroing_target.zeroing_target_offset,
            self.zeroing.zeroing_target.zeroing_target_tolerance,
        )?)
    }
    pub fn shared_params<D>(&self) -> Result<SimulationBuilder<D>>
    where
        D: DragFunction,
    {
        let mut builder = SimulationBuilder::new();
        builder = builder.set_time_step(self.time_step)?;

        builder = builder.use_coriolis(!self.flags.disable_coriolis);
        builder = builder.use_drag(!self.flags.disable_drag);
        builder = builder.use_gravity(!self.flags.disable_gravity);

        // Projectile
        if let Some(value) = self.projectile.projectile_bc {
            builder = builder.set_bc(value)?
        }
        if let Some(value) = self.projectile.projectile_velocity {
            builder = builder.set_velocity(value)?
        }
        if let Some(value) = self.projectile.projectile_mass {
            builder = builder.set_mass(value)?
        }
        if let Some(value) = self.projectile.projectile_caliber {
            builder = builder.set_caliber(value)?
        }

        // Scope
        if let Some(value) = self.scope.scope_height {
            builder = builder.set_scope_height(value)
        }
        if let Some(value) = self.scope.scope_offset {
            builder = builder.set_scope_offset(value)
        }
        if let Some(value) = self.scope.scope_cant {
            builder = builder.set_scope_roll(value)
        }

        Ok(builder)
    }
    pub fn zero_scenario<D>(&self, mut builder: SimulationBuilder<D>) -> Result<Simulation<D>>
    where
        D: DragFunction,
    {
        // Atmosphere
        if let Some(value) = self
            .zeroing
            .zeroing_atmosphere
            .zeroing_atmosphere_temperature
        {
            builder = builder.set_temperature(value)?
        }
        if let Some(value) = self.zeroing.zeroing_atmosphere.zeroing_atmosphere_pressure {
            builder = builder.set_pressure(value)?
        }
        if let Some(value) = self.zeroing.zeroing_atmosphere.zeroing_atmosphere_humidity {
            builder = builder.set_humidity(value)?
        }

        // Wind
        if let Some(value) = self.zeroing.zeroing_wind.zeroing_wind_speed {
            builder = builder.set_wind_speed(value)?
        }
        if let Some(value) = self.zeroing.zeroing_wind.zeroing_wind_angle {
            builder = builder.set_wind_angle(value)?
        }

        // Shooter
        if let Some(value) = self.zeroing.zeroing_shooter.zeroing_shooter_angle {
            builder = builder.set_shot_angle(value)?
        }
        if let Some(value) = self.zeroing.zeroing_shooter.zeroing_shooter_lattitude {
            builder = builder.set_lattitude(value)?
        }
        if let Some(value) = self.zeroing.zeroing_shooter.zeroing_shooter_bearing {
            builder = builder.set_bearing(value)?
        }
        if let Some(value) = self.zeroing.zeroing_shooter.zeroing_shooter_gravity {
            builder = builder.set_gravity(value)?
        }
        Ok(builder.init())
    }
    pub fn firing_scenario<D>(
        &self,
        mut builder: SimulationBuilder<D>,
        pitch: Angle,
        yaw: Angle,
    ) -> Result<Simulation<D>>
    where
        D: DragFunction,
    {
        // Adjust pitch/yaw with value from args, and provided deltas
        if let Some(value) = self.scope.scope_pitch {
            builder = builder.set_scope_pitch(dbg!(value + pitch))
        } else {
            builder = builder.set_scope_pitch(pitch)
        }
        if let Some(value) = self.scope.scope_yaw {
            builder = builder.set_scope_yaw(dbg!(value + yaw))
        } else {
            builder = builder.set_scope_yaw(yaw)
        }

        // Atmosphere
        if let Some(value) = self.firing.firing_atmosphere.firing_atmosphere_temperature {
            builder = builder.set_temperature(value)?
        }
        if let Some(value) = self.firing.firing_atmosphere.firing_atmosphere_pressure {
            builder = builder.set_pressure(value)?
        }
        if let Some(value) = self.firing.firing_atmosphere.firing_atmosphere_humidity {
            builder = builder.set_humidity(value)?
        }

        // Wind
        if let Some(value) = self.firing.firing_wind.firing_wind_speed {
            builder = builder.set_wind_speed(value)?
        }
        if let Some(value) = self.firing.firing_wind.firing_wind_angle {
            builder = builder.set_wind_angle(value)?
        }

        // Shooter
        if let Some(value) = self.firing.firing_shooter.firing_shooter_angle {
            builder = builder.set_shot_angle(value)?
        }
        if let Some(value) = self.firing.firing_shooter.firing_shooter_lattitude {
            builder = builder.set_lattitude(value)?
        }
        if let Some(value) = self.firing.firing_shooter.firing_shooter_bearing {
            builder = builder.set_bearing(value)?
        }
        if let Some(value) = self.firing.firing_shooter.firing_shooter_gravity {
            builder = builder.set_gravity(value)?
        }
        Ok(builder.init())
    }
}
