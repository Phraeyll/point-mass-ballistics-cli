use crate::printer::print_table;

use std::{error::Error, file, line, ops::DerefMut, stringify, time::Instant};

use clap::Parser;
use indoc::indoc;
use point_mass_ballistics::{
    output::Measurements,
    projectiles::{self as bc, Projectile, ProjectileImpl},
    simulation::{Simulation, SimulationBuilder},
    units::{
        radian, Acceleration, Angle, Length, Mass, Pressure, ThermodynamicTemperature, Time,
        Velocity,
    },
    Numeric,
};

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[clap(
    name = "Ballistic Solver",
    author = "Phraeyll <Phraeyll@users.no-reply.github.com",
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
    #[clap(subcommand)]
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
    #[clap(long = "time-step", default_value = "0.00005 s")]
    time_step: Time,

    #[clap(flatten)]
    flags: Flags,

    #[clap(flatten)]
    table: Table,

    #[clap(flatten)]
    projectile: ProjectileArg,

    #[clap(flatten)]
    scope: Scope,

    #[clap(flatten)]
    firing: Firing,

    #[clap(flatten)]
    zeroing: Zeroing,
}
#[derive(Debug, Parser)]
struct Flags {
    #[clap(long = "flat")]
    flat: bool,

    #[clap(long = "disable-drag")]
    disable_drag: bool,

    #[clap(long = "disable-coriolis")]
    disable_coriolis: bool,

    #[clap(long = "disable-gravity")]
    disable_gravity: bool,

    #[clap(long = "pretty")]
    pretty: bool,
}
#[derive(Debug, Parser)]
struct Table {
    #[clap(long = "start", default_value = "0.0 yd")]
    table_start: Length,

    #[clap(long = "end", default_value = "1000.0 yd")]
    table_end: Length,

    #[clap(long = "step", default_value = "100.0 yd")]
    table_step: Length,

    #[clap(long = "table-tolerance", default_value = "0.005 in")]
    table_tolerance: Length,
}
#[derive(Debug, Parser)]
struct ProjectileArg {
    #[clap(long = "initial-velocity")]
    projectile_velocity: Option<Velocity>,

    #[clap(long = "mass")]
    projectile_mass: Option<Mass>,

    #[clap(long = "caliber")]
    projectile_caliber: Option<Length>,

    #[clap(long = "bc")]
    projectile_bc: Option<Numeric>,
}
#[derive(Debug, Parser)]
struct Scope {
    #[clap(long = "scope-height")]
    scope_height: Option<Length>,

    #[clap(long = "scope-offset")]
    scope_offset: Option<Length>,

    #[clap(long = "scope-pitch")]
    scope_pitch: Option<Angle>,

    #[clap(long = "scope-yaw")]
    scope_yaw: Option<Angle>,

    #[clap(long = "scope-cant")]
    scope_cant: Option<Angle>,
}
#[derive(Debug, Parser)]
struct Firing {
    #[clap(flatten)]
    firing_atmosphere: FiringAtmosphere,

    #[clap(flatten)]
    firing_wind: FiringWind,

    #[clap(flatten)]
    firing_shooter: FiringShooter,
}
#[derive(Debug, Parser)]
struct FiringWind {
    #[clap(long = "wind-speed")]
    firing_wind_speed: Option<Velocity>,

    #[clap(long = "wind-angle")]
    firing_wind_angle: Option<Angle>,
}
#[derive(Debug, Parser)]
struct FiringAtmosphere {
    #[clap(long = "temperature")]
    firing_atmosphere_temperature: Option<ThermodynamicTemperature>,

    #[clap(long = "pressure")]
    firing_atmosphere_pressure: Option<Pressure>,

    #[clap(long = "humidity")]
    firing_atmosphere_humidity: Option<Numeric>,
}
#[derive(Debug, Parser)]
struct FiringShooter {
    #[clap(long = "lattitude")]
    firing_shooter_lattitude: Option<Angle>,

    #[clap(long = "bearing")]
    firing_shooter_bearing: Option<Angle>,

    #[clap(long = "shot-angle")]
    firing_shooter_angle: Option<Angle>,

    #[clap(long = "gravity")]
    firing_shooter_gravity: Option<Acceleration>,
}
#[derive(Debug, Parser)]
struct Zeroing {
    #[clap(flatten)]
    zeroing_wind: ZeroingWind,

    #[clap(flatten)]
    zeroing_atmosphere: ZeroingAtmosphere,

    #[clap(flatten)]
    zeroing_shooter: ZeroingShooter,

    #[clap(flatten)]
    zeroing_target: ZeroingTarget,
}
#[derive(Debug, Parser)]
struct ZeroingWind {
    #[clap(long = "zeroing-wind-speed")]
    zeroing_wind_speed: Option<Velocity>,

    #[clap(long = "zeroing-wind-angle")]
    zeroing_wind_angle: Option<Angle>,
}
#[derive(Debug, Parser)]
struct ZeroingAtmosphere {
    #[clap(long = "zeroing-temperature")]
    zeroing_atmosphere_temperature: Option<ThermodynamicTemperature>,

    #[clap(long = "zeroing-pressure")]
    zeroing_atmosphere_pressure: Option<Pressure>,

    #[clap(long = "zeroing-humidity")]
    zeroing_atmosphere_humidity: Option<Numeric>,
}
#[derive(Debug, Parser)]
struct ZeroingShooter {
    #[clap(long = "zeroing-lattitude")]
    zeroing_shooter_lattitude: Option<Angle>,

    #[clap(long = "zeroing-bearing")]
    zeroing_shooter_bearing: Option<Angle>,

    #[clap(long = "zeroing-shot-angle")]
    zeroing_shooter_angle: Option<Angle>,

    #[clap(long = "zeroing-gravity")]
    zeroing_shooter_gravity: Option<Acceleration>,
}
#[derive(Debug, Parser)]
struct ZeroingTarget {
    #[clap(long = "zeroing-target-distance", default_value = "200.0 yd")]
    zeroing_target_distance: Length,

    #[clap(long = "zeroing-target-height", default_value = "0.0 in")]
    zeroing_target_height: Length,

    #[clap(long = "zeroing-target-offset", default_value = "0.0 in")]
    zeroing_target_offset: Length,

    #[clap(long = "zeroing-target-tolerance", default_value = "0.001 in")]
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
            Self::G1(ref inner) => inner.run::<bc::G1>(),
            Self::G2(ref inner) => inner.run::<bc::G2>(),
            Self::G5(ref inner) => inner.run::<bc::G5>(),
            Self::G6(ref inner) => inner.run::<bc::G6>(),
            Self::G7(ref inner) => inner.run::<bc::G7>(),
            Self::G8(ref inner) => inner.run::<bc::G8>(),
            Self::GI(ref inner) => inner.run::<bc::GI>(),
            Self::GS(ref inner) => inner.run::<bc::GS>(),
        }
    }
}

impl InnerArgs {
    pub fn run<T>(&self) -> Result<()>
    where
        T: Projectile + From<ProjectileImpl> + DerefMut<Target = ProjectileImpl>,
    {
        let mut angles = (Angle::new::<radian>(0.0), Angle::new::<radian>(0.0));
        if !self.flags.flat {
            let zero_builder = time!(self.shared_params::<T>()?);
            let zero_simulation = time!(self.zero_scenario(zero_builder)?);
            angles = time!(self.try_zero(zero_simulation)?);
        };
        let firing_builder = time!(self.shared_params::<T>()?);
        let firing_simulation = time!(self.firing_scenario(firing_builder, angles.0, angles.1)?);
        time!(self.print(&firing_simulation));
        Ok(())
    }
    pub fn print<T>(&self, simulation: &Simulation<T>)
    where
        T: Projectile,
    {
        let output_tolerance = self.table.table_tolerance;
        print_table(
            self.table_gen(&simulation),
            output_tolerance,
            self.flags.pretty,
        );
    }
    pub fn table_gen<'s, T>(
        &self,
        simulation: &'s Simulation<T>,
    ) -> impl IntoIterator<Item = impl Measurements + 's> + 's
    where
        T: Projectile,
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
    pub fn try_zero<T>(&self, mut simulation: Simulation<T>) -> Result<(Angle, Angle)>
    where
        T: Projectile,
    {
        Ok(simulation.find_zero_angles(
            self.zeroing.zeroing_target.zeroing_target_distance,
            self.zeroing.zeroing_target.zeroing_target_height,
            self.zeroing.zeroing_target.zeroing_target_offset,
            self.zeroing.zeroing_target.zeroing_target_tolerance,
        )?)
    }
    pub fn shared_params<T>(&self) -> Result<SimulationBuilder<T>>
    where
        T: Projectile + From<ProjectileImpl> + DerefMut<Target = ProjectileImpl>,
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
    pub fn zero_scenario<T>(&self, mut builder: SimulationBuilder<T>) -> Result<Simulation<T>>
    where
        T: Projectile,
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
    pub fn firing_scenario<T>(
        &self,
        mut builder: SimulationBuilder<T>,
        pitch: Angle,
        yaw: Angle,
    ) -> Result<Simulation<T>>
    where
        T: Projectile,
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
