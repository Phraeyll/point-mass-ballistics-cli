use super::{
    args::{Args, SimulationKind},
    printer,
};

use std::{error::Error, file, line, ops::DerefMut, stringify, time::Instant};

use point_mass_ballistics::{
    output::Measurements,
    projectiles::{self as bc, Projectile, ProjectileImpl},
    simulation::{Simulation, SimulationBuilder},
    units::{radian, Angle},
};

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

impl SimulationKind {
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        match *self {
            Self::G1(ref params) => params.run::<bc::G1>(),
            Self::G2(ref params) => params.run::<bc::G2>(),
            Self::G5(ref params) => params.run::<bc::G5>(),
            Self::G6(ref params) => params.run::<bc::G6>(),
            Self::G7(ref params) => params.run::<bc::G7>(),
            Self::G8(ref params) => params.run::<bc::G8>(),
            Self::GI(ref params) => params.run::<bc::GI>(),
            Self::GS(ref params) => params.run::<bc::GS>(),
        }
    }
}

impl Args {
    pub fn run<T>(&self) -> Result<(), Box<dyn Error>>
    where
        T: Projectile + From<ProjectileImpl> + DerefMut<Target = ProjectileImpl>,
    {
        let mut angles = (Angle::new::<radian>(0.0), Angle::new::<radian>(0.0));
        if !self.flags().flat() {
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
        let output_tolerance = self.table().tolerance();
        printer::print(
            self.table_gen(&simulation),
            output_tolerance,
            self.flags().pretty(),
        );
    }
    pub fn table_gen<'s, T>(
        &self,
        simulation: &'s Simulation<T>,
    ) -> impl IntoIterator<Item = impl Measurements + 's> + 's
    where
        T: Projectile,
    {
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
    pub fn try_zero<T>(
        &self,
        mut simulation: Simulation<T>,
    ) -> Result<(Angle, Angle), Box<dyn Error>>
    where
        T: Projectile,
    {
        Ok(simulation.find_zero_angles(
            self.zeroing().target().distance(),
            self.zeroing().target().height(),
            self.zeroing().target().offset(),
            self.zeroing().target().tolerance(),
        )?)
    }
    pub fn shared_params<T>(&self) -> Result<SimulationBuilder<T>, Box<dyn Error>>
    where
        T: Projectile + From<ProjectileImpl> + DerefMut<Target = ProjectileImpl>,
    {
        let mut builder = SimulationBuilder::new();
        builder = builder.set_time_step(self.time())?;

        builder = builder.use_coriolis(self.flags().coriolis());
        builder = builder.use_drag(self.flags().drag());
        builder = builder.use_gravity(self.flags().gravity());

        // Projectile
        if let Some(value) = self.projectile().bc() {
            builder = builder.set_bc(value)?
        }
        if let Some(value) = self.projectile().velocity() {
            builder = builder.set_velocity(value)?
        }
        if let Some(value) = self.projectile().mass() {
            builder = builder.set_mass(value)?
        }
        if let Some(value) = self.projectile().caliber() {
            builder = builder.set_caliber(value)?
        }

        // Scope
        if let Some(value) = self.scope().height() {
            builder = builder.set_scope_height(value)
        }
        if let Some(value) = self.scope().offset() {
            builder = builder.set_scope_offset(value)
        }
        if let Some(value) = self.scope().cant() {
            builder = builder.set_scope_roll(value)
        }

        Ok(builder)
    }
    pub fn zero_scenario<T>(
        &self,
        mut builder: SimulationBuilder<T>,
    ) -> Result<Simulation<T>, Box<dyn Error>>
    where
        T: Projectile,
    {
        // Atmosphere
        if let Some(value) = self.zeroing().atmosphere().temperature() {
            builder = builder.set_temperature(value)?
        }
        if let Some(value) = self.zeroing().atmosphere().pressure() {
            builder = builder.set_pressure(value)?
        }
        if let Some(value) = self.zeroing().atmosphere().humidity() {
            builder = builder.set_humidity(value)?
        }

        // Wind
        if let Some(value) = self.zeroing().wind().speed() {
            builder = builder.set_wind_speed(value)?
        }
        if let Some(value) = self.zeroing().wind().angle() {
            builder = builder.set_wind_angle(value)?
        }

        // Shooter
        if let Some(value) = self.zeroing().shooter().angle() {
            builder = builder.set_shot_angle(value)?
        }
        if let Some(value) = self.zeroing().shooter().lattitude() {
            builder = builder.set_lattitude(value)?
        }
        if let Some(value) = self.zeroing().shooter().bearing() {
            builder = builder.set_bearing(value)?
        }
        if let Some(value) = self.zeroing().shooter().gravity() {
            builder = builder.set_gravity(value)?
        }
        Ok(builder.init())
    }
    pub fn firing_scenario<T>(
        &self,
        mut builder: SimulationBuilder<T>,
        pitch: Angle,
        yaw: Angle,
    ) -> Result<Simulation<T>, Box<dyn Error>>
    where
        T: Projectile,
    {
        // Adjust pitch/yaw with value from args, and provided deltas
        if let Some(value) = self.scope().pitch() {
            builder = builder.set_scope_pitch(dbg!(value + pitch))
        } else {
            builder = builder.set_scope_pitch(pitch)
        }
        if let Some(value) = self.scope().yaw() {
            builder = builder.set_scope_yaw(dbg!(value + yaw))
        } else {
            builder = builder.set_scope_yaw(yaw)
        }

        // Atmosphere
        if let Some(value) = self.firing().atmosphere().temperature() {
            builder = builder.set_temperature(value)?
        }
        if let Some(value) = self.firing().atmosphere().pressure() {
            builder = builder.set_pressure(value)?
        }
        if let Some(value) = self.firing().atmosphere().humidity() {
            builder = builder.set_humidity(value)?
        }

        // Wind
        if let Some(value) = self.firing().wind().speed() {
            builder = builder.set_wind_speed(value)?
        }
        if let Some(value) = self.firing().wind().angle() {
            builder = builder.set_wind_angle(value)?
        }

        // Shooter
        if let Some(value) = self.firing().shooter().angle() {
            builder = builder.set_shot_angle(value)?
        }
        if let Some(value) = self.firing().shooter().lattitude() {
            builder = builder.set_lattitude(value)?
        }
        if let Some(value) = self.firing().shooter().bearing() {
            builder = builder.set_bearing(value)?
        }
        if let Some(value) = self.firing().shooter().gravity() {
            builder = builder.set_gravity(value)?
        }
        Ok(builder.init())
    }
}
