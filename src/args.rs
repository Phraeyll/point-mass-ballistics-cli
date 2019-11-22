use self::options::Options;
use crate::printer::*;

use std::{str::FromStr, string::ToString};

use point_mass_ballistics::{
    Acceleration, Angle, BcKind, Length, Mass, Measurements, Numeric, ParseQuantityError, Pressure,
    Result, Simulation, SimulationBuilder, ThermodynamicTemperature, Time, Velocity,
};

pub mod options;

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
            #[derive(Clone, Copy, Debug)]
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
        let output_tolerance = self.table().tolerance();
        if self.flags().pretty() {
            pretty::print(self.table_gen(&simulation), output_tolerance);
        } else {
            plain::print(self.table_gen(&simulation), output_tolerance);
        }
    }
    pub fn table_gen<'s>(
        &self,
        simulation: &'s Simulation,
    ) -> impl IntoIterator<Item = impl Measurements + 's> + 's {
        let mut start = self.table().start();
        let end = self.table().end();
        let step = self.table().step();
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
            self.zeroing().target().distance(),
            self.zeroing().target().height(),
            self.zeroing().target().offset(),
            self.zeroing().target().tolerance(),
        )?)
    }
    pub fn shared_params(&self) -> Result<SimulationBuilder> {
        let mut builder = SimulationBuilder::new();
        builder = builder.set_time_step(self.time())?;

        builder = builder.use_coriolis(self.flags().coriolis());
        builder = builder.use_drag(self.flags().drag());
        builder = builder.use_gravity(self.flags().gravity());

        // Projectile
        if let Some(val) = self.projectile().bc().value() {
            if let Some(kind) = self.projectile().bc().kind() {
                builder = builder.set_bc_value(val)?;
                builder = builder.set_bc_kind(kind)?;
            }
        }
        if let Some(val) = self.projectile().velocity() {
            builder = builder.set_velocity(val)?
        }
        if let Some(val) = self.projectile().mass() {
            builder = builder.set_mass(val)?
        }
        if let Some(val) = self.projectile().caliber() {
            builder = builder.set_caliber(val)?
        }

        // Scope
        if let Some(val) = self.scope().height() {
            builder = builder.set_scope_height(val)
        }
        if let Some(val) = self.scope().offset() {
            builder = builder.set_scope_offset(val)
        }
        if let Some(val) = self.scope().cant() {
            builder = builder.set_scope_roll(val)
        }

        Ok(builder)
    }
    pub fn zero_scenario(&self, mut builder: SimulationBuilder) -> Result<Simulation> {
        // Atmosphere
        if let Some(val) = self.zeroing().atmosphere().temperature() {
            builder = builder.set_temperature(val)?
        }
        if let Some(val) = self.zeroing().atmosphere().pressure() {
            builder = builder.set_pressure(val)?
        }
        if let Some(val) = self.zeroing().atmosphere().humidity() {
            builder = builder.set_humidity(val)?
        }

        // Wind
        if let Some(val) = self.zeroing().wind().speed() {
            builder = builder.set_wind_speed(val)?
        }
        if let Some(val) = self.zeroing().wind().angle() {
            builder = builder.set_wind_angle(val)?
        }

        // Shooter
        if let Some(val) = self.zeroing().shooter().angle() {
            builder = builder.set_shot_angle(val)?
        }
        if let Some(val) = self.zeroing().shooter().lattitude() {
            builder = builder.set_lattitude(val)?
        }
        if let Some(val) = self.zeroing().shooter().bearing() {
            builder = builder.set_bearing(val)?
        }
        if let Some(val) = self.zeroing().shooter().gravity() {
            builder = builder.set_gravity(val)?
        }
        Ok(builder.init())
    }
    pub fn firing_scenario(
        &self,
        mut builder: SimulationBuilder,
        pitch: Angle,
        yaw: Angle,
    ) -> Result<Simulation> {
        // Adjust pitch/yaw with val from args, and provided deltas
        if let Some(val) = self.scope().pitch() {
            builder = builder.set_scope_pitch(dbg!(val + pitch))
        } else {
            builder = builder.set_scope_pitch(pitch)
        }
        if let Some(val) = self.scope().yaw() {
            builder = builder.set_scope_yaw(dbg!(val + yaw))
        } else {
            builder = builder.set_scope_yaw(yaw)
        }

        // Atmosphere
        if let Some(val) = self.firing().atmosphere().temperature() {
            builder = builder.set_temperature(val)?
        }
        if let Some(val) = self.firing().atmosphere().pressure() {
            builder = builder.set_pressure(val)?
        }
        if let Some(val) = self.firing().atmosphere().humidity() {
            builder = builder.set_humidity(val)?
        }

        // Wind
        if let Some(val) = self.firing().wind().speed() {
            builder = builder.set_wind_speed(val)?
        }
        if let Some(val) = self.firing().wind().angle() {
            builder = builder.set_wind_angle(val)?
        }

        // Shooter
        if let Some(val) = self.firing().shooter().angle() {
            builder = builder.set_shot_angle(val)?
        }
        if let Some(val) = self.firing().shooter().lattitude() {
            builder = builder.set_lattitude(val)?
        }
        if let Some(val) = self.firing().shooter().bearing() {
            builder = builder.set_bearing(val)?
        }
        if let Some(val) = self.firing().shooter().gravity() {
            builder = builder.set_gravity(val)?
        }
        Ok(builder.init())
    }
}
