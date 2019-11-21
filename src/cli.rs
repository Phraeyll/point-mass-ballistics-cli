use crate::printer::*;

use std::{str::FromStr, string::ToString};

use point_mass_ballistics::{
    Acceleration, Angle, BcKind, Length, Mass, Measurements, Numeric, ParseQuantityError, Pressure,
    Result, Simulation, SimulationBuilder, ThermodynamicTemperature, Time, Velocity,
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Ballistic Solver",
    author = "Phraeyll <Phraeyll@users.no-reply.github.com",
    about = r#"
            Produces range table from vector based simulation of Newtons Equations
            using standard, unmodified, point mass model of ballistics.
            Currently, this accounts for drag, gravity, and Coriolis/Eotvos forces.
            This does not currently factor in gyroscopic drift, nor aerodynamic jump.
            Drag tables obtained from JBM Ballistics, and methodologies are mostly from
            Robert L. McCoy's "Modern Exterior Ballistics" ISBN 978-0-7643-3825-0

            The eventual goal of this program is to support modified point mass trajectories
            as well, for factoring in gyroscopic drift and aerodynamic jump (4-DOF models)
            "#
)]
pub struct Options {
    #[structopt(long = "flat")]
    flat: bool,

    #[structopt(long = "disable-drag")]
    disable_drag: bool,

    #[structopt(long = "disable-coriolis")]
    disable_coriolis: bool,

    #[structopt(long = "disable-gravity")]
    disable_gravity: bool,

    #[structopt(long = "pretty")]
    pretty: bool,

    #[structopt(flatten)]
    projectile: Projectile,

    #[structopt(flatten)]
    scope: Scope,

    #[structopt(flatten)]
    atmosphere: Atmosphere,

    #[structopt(flatten)]
    wind: Wind,

    #[structopt(flatten)]
    shooter: Shooter,

    #[structopt(flatten)]
    zero_scope: ZeroScope,

    #[structopt(flatten)]
    zero_atmosphere: ZeroAtmosphere,

    #[structopt(flatten)]
    zero_wind: ZeroWind,

    #[structopt(flatten)]
    zero_shooter: ZeroShooter,

    #[structopt(flatten)]
    table: Table,

    #[structopt(flatten)]
    zero: Zero,

    #[structopt(long = "time-interval", default_value = "0.00005 s")]
    time_interval: MyTime,
}

#[derive(Debug, StructOpt)]
struct Scope {
    #[structopt(long = "scope-height")]
    scope_height: Option<MyLength>,

    #[structopt(long = "scope-offset")]
    scope_offset: Option<MyLength>,

    #[structopt(long = "scope-pitch")]
    scope_pitch: Option<MyAngle>,

    #[structopt(long = "scope-yaw")]
    scope_yaw: Option<MyAngle>,

    #[structopt(long = "scope-cant")]
    scope_cant: Option<MyAngle>,
}

#[derive(Debug, StructOpt)]
struct ZeroScope {
    #[structopt(long = "zero-scope-height")]
    zero_scope_height: Option<MyLength>,

    #[structopt(long = "zero-scope-offset")]
    zero_scope_offset: Option<MyLength>,

    #[structopt(long = "zero-scope-pitch")]
    zero_scope_pitch: Option<MyAngle>,

    #[structopt(long = "zero-scope-yaw")]
    zero_scope_yaw: Option<MyAngle>,

    #[structopt(long = "zero-scope-cant")]
    zero_scope_cant: Option<MyAngle>,
}

#[derive(Debug, StructOpt)]
struct Wind {
    #[structopt(long = "wind-speed")]
    wind_speed: Option<MyVelocity>,

    #[structopt(long = "wind-angle")]
    wind_angle: Option<MyAngle>,
}

#[derive(Debug, StructOpt)]
struct ZeroWind {
    #[structopt(long = "zero-wind-speed")]
    zero_wind_speed: Option<MyVelocity>,

    #[structopt(long = "zero-wind-angle")]
    zero_wind_angle: Option<MyAngle>,
}

#[derive(Debug, StructOpt)]
struct Atmosphere {
    #[structopt(long = "temperature")]
    temperature: Option<MyThermodynamicTemperature>,

    #[structopt(long = "pressure")]
    pressure: Option<MyPressure>,

    #[structopt(long = "humidity")]
    humidity: Option<Numeric>,
}

#[derive(Debug, StructOpt)]
struct ZeroAtmosphere {
    #[structopt(long = "zero-temperature")]
    zero_temperature: Option<MyThermodynamicTemperature>,

    #[structopt(long = "zero-pressure")]
    zero_pressure: Option<MyPressure>,

    #[structopt(long = "zero-humidity")]
    zero_humidity: Option<Numeric>,
}

#[derive(Debug, StructOpt)]
struct Shooter {
    #[structopt(long = "lattitude")]
    lattitude: Option<MyAngle>,

    #[structopt(long = "bearing")]
    bearing: Option<MyAngle>,

    #[structopt(long = "shot-angle")]
    shot_angle: Option<MyAngle>,

    #[structopt(long = "gravity")]
    gravity: Option<MyAcceleration>,
}

#[derive(Debug, StructOpt)]
struct ZeroShooter {
    #[structopt(long = "zero-lattitude")]
    zero_lattitude: Option<MyAngle>,

    #[structopt(long = "zero-bearing")]
    zero_bearing: Option<MyAngle>,

    #[structopt(long = "zero-shot-angle")]
    zero_shot_angle: Option<MyAngle>,

    #[structopt(long = "zero-gravity")]
    zero_gravity: Option<MyAcceleration>,
}

#[derive(Debug, StructOpt)]
struct Table {
    #[structopt(long = "start", default_value = "0.0 yd")]
    start: MyLength,

    #[structopt(long = "end", default_value = "1000.0 yd")]
    end: MyLength,

    #[structopt(long = "step", default_value = "100.0 yd")]
    step: MyLength,

    #[structopt(long = "table-tolerance", default_value = "0.005 in")]
    tolerance: MyLength,
}

#[derive(Debug, StructOpt)]
struct Zero {
    #[structopt(long = "zero-target-distance", default_value = "200.0 yd")]
    zero_target_distance: MyLength,

    #[structopt(long = "zero-target-height", default_value = "0.0 in")]
    zero_target_height: MyLength,

    #[structopt(long = "zero-target-offset", default_value = "0.0 in")]
    zero_target_offset: MyLength,

    #[structopt(long = "zero-target-tolerance", default_value = "0.001 in")]
    zero_target_tolerance: MyLength,
}

#[derive(Debug, StructOpt)]
struct Projectile {
    #[structopt(long = "initial-velocity")]
    projectile_velocity: Option<MyVelocity>,

    #[structopt(long = "mass")]
    mass: Option<MyMass>,

    #[structopt(long = "caliber")]
    caliber: Option<MyLength>,

    #[structopt(long = "bc-value")]
    bc_value: Option<Numeric>,

    #[structopt(long = "bc-kind")]
    bc_kind: Option<BcKind>,
}

#[derive(Debug)]
struct MyParseQuantityError {
    error: ParseQuantityError,
}

impl ToString for MyParseQuantityError {
    fn to_string(&self) -> String {
        match self.error {
            ParseQuantityError::NoSeparator => "No Separator".to_string(),
            ParseQuantityError::UnknownUnit => "Unknown Unit".to_string(),
            ParseQuantityError::ValueParseError => "Value Parse Error".to_string(),
        }
    }
}

macro_rules! my_quantities {
    ( $($my:ident => $uom:ident,)+ ) => {
        my_quantities! {
            $($my => $uom),+
        }
    };
    ( $($my:ident => $uom:ident),* ) => {
        $(
            #[derive(Debug)]
            struct $my {
                val: $uom,
            }
            impl FromStr for $my {
                type Err = MyParseQuantityError;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    <$uom as FromStr>::from_str(s)
                        .map(|val| $my { val })
                        .map_err(|error| MyParseQuantityError { error })
                }
            }
        )*
    };
}

my_quantities! {
    MyAngle => Angle,
    MyMass => Mass,
    MyLength => Length,
    MyTime => Time,
    MyVelocity => Velocity,
    MyAcceleration => Acceleration,
    MyThermodynamicTemperature => ThermodynamicTemperature,
    MyPressure => Pressure,
}

impl Options {
    pub fn print(&self, simulation: &Simulation) {
        let output_tolerance = self.table.tolerance.val;
        if self.pretty {
            pretty::print(self.table(&simulation), output_tolerance);
        } else {
            plain::print(self.table(&simulation), output_tolerance);
        }
    }
    pub fn table<'s>(
        &self,
        simulation: &'s Simulation,
    ) -> impl IntoIterator<Item = impl Measurements + 's> + 's {
        let mut start = self.table.start.val;
        let end = self.table.end.val;
        let step = self.table.step.val;
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
    pub fn try_zero(&self, mut simulation: Simulation) -> Result<(Angle, Angle)> {
        Ok(simulation.find_zero_angles(
            self.zero.zero_target_distance.val,
            self.zero.zero_target_height.val,
            self.zero.zero_target_offset.val,
            self.zero.zero_target_tolerance.val,
        )?)
    }
    pub fn shared_params(&self) -> Result<SimulationBuilder> {
        let mut builder = SimulationBuilder::new();
        builder = builder.set_time_step(self.time_interval.val)?;

        builder = builder.use_coriolis(!self.disable_coriolis);
        builder = builder.use_drag(!self.disable_drag);
        builder = builder.use_gravity(!self.disable_gravity);

        // Projectile
        if let Some(val) = self.projectile.bc_value {
            builder = builder.set_bc_value(val)?
        }
        if let Some(val) = self.projectile.bc_kind {
            builder = builder.set_bc_kind(val)?
        }
        if let Some(ref val) = self.projectile.projectile_velocity {
            builder = builder.set_velocity(val.val)?
        }
        if let Some(ref val) = self.projectile.mass {
            builder = builder.set_mass(val.val)?
        }
        if let Some(ref val) = self.projectile.caliber {
            builder = builder.set_caliber(val.val)?
        }
        Ok(builder)
    }
    pub fn zero_scenario(&self, mut builder: SimulationBuilder) -> Result<Simulation> {
        // Scope
        if let Some(ref val) = self.zero_scope.zero_scope_height {
            builder = builder.set_scope_height(val.val)
        }
        if let Some(ref val) = self.zero_scope.zero_scope_offset {
            builder = builder.set_scope_offset(val.val)
        }
        if let Some(ref val) = self.zero_scope.zero_scope_cant {
            builder = builder.set_scope_roll(val.val)
        }

        // Atmosphere
        if let Some(ref val) = self.zero_atmosphere.zero_temperature {
            builder = builder.set_temperature(val.val)?
        }
        if let Some(ref val) = self.zero_atmosphere.zero_pressure {
            builder = builder.set_pressure(val.val)?
        }
        if let Some(val) = self.zero_atmosphere.zero_humidity {
            builder = builder.set_humidity(val)?
        }

        // Wind
        if let Some(ref val) = self.zero_wind.zero_wind_speed {
            builder = builder.set_wind_speed(val.val)?
        }
        if let Some(ref val) = self.zero_wind.zero_wind_angle {
            builder = builder.set_wind_angle(val.val)?
        }

        // Shooter
        if let Some(ref val) = self.zero_shooter.zero_shot_angle {
            builder = builder.set_shot_angle(val.val)?
        }
        if let Some(ref val) = self.zero_shooter.zero_lattitude {
            builder = builder.set_lattitude(val.val)?
        }
        if let Some(ref val) = self.zero_shooter.zero_bearing {
            builder = builder.set_bearing(val.val)?
        }
        if let Some(ref val) = self.zero_shooter.zero_gravity {
            builder = builder.set_gravity(val.val)?
        }
        Ok(builder.init())
    }
    pub fn firing_scenario(
        &self,
        mut builder: SimulationBuilder,
        pitch: Angle,
        yaw: Angle,
    ) -> Result<Simulation> {
        // Scope
        if let Some(ref val) = self.scope.scope_height {
            builder = builder.set_scope_height(val.val)
        }
        if let Some(ref val) = self.scope.scope_offset {
            builder = builder.set_scope_offset(val.val)
        }
        if let Some(ref val) = self.scope.scope_pitch {
            builder = builder.set_scope_pitch(dbg!(val.val + pitch))
        } else {
            builder = builder.set_scope_pitch(pitch)
        }
        if let Some(ref val) = self.scope.scope_yaw {
            builder = builder.set_scope_yaw(dbg!(val.val + yaw))
        } else {
            builder = builder.set_scope_yaw(yaw)
        }
        if let Some(ref val) = self.scope.scope_cant {
            builder = builder.set_scope_roll(val.val)
        }

        // Atmosphere
        if let Some(ref val) = self.atmosphere.temperature {
            builder = builder.set_temperature(val.val)?
        }
        if let Some(ref val) = self.atmosphere.pressure {
            builder = builder.set_pressure(val.val)?
        }
        if let Some(val) = self.atmosphere.humidity {
            builder = builder.set_humidity(val)?
        }

        // Wind
        if let Some(ref val) = self.wind.wind_speed {
            builder = builder.set_wind_speed(val.val)?
        }
        if let Some(ref val) = self.wind.wind_angle {
            builder = builder.set_wind_angle(val.val)?
        }

        // Shooter
        if let Some(ref val) = self.shooter.shot_angle {
            builder = builder.set_shot_angle(val.val)?
        }
        if let Some(ref val) = self.shooter.lattitude {
            builder = builder.set_lattitude(val.val)?
        }
        if let Some(ref val) = self.shooter.bearing {
            builder = builder.set_bearing(val.val)?
        }
        if let Some(ref val) = self.shooter.gravity {
            builder = builder.set_gravity(val.val)?
        }
        Ok(builder.init())
    }
}
