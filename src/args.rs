use std::{
    fmt,
    ops::{Deref, DerefMut},
    str::FromStr,
    string::ToString,
};

use point_mass_ballistics::{
    units::{
        Acceleration, Angle, Length, Mass, ParseQuantityError, Pressure, ThermodynamicTemperature,
        Time, Velocity,
    },
    Numeric,
};
use structopt::StructOpt;

#[derive(Debug)]
struct MyParseQuantityError(ParseQuantityError);
impl fmt::Display for MyParseQuantityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match **self {
            ParseQuantityError::NoSeparator => write!(f, "No Separator"),
            ParseQuantityError::UnknownUnit => write!(f, "Unknown Unit"),
            ParseQuantityError::ValueParseError => write!(f, "Value Parse Error"),
        }
    }
}

impl From<ParseQuantityError> for MyParseQuantityError {
    fn from(other: ParseQuantityError) -> Self {
        Self(other)
    }
}

impl From<MyParseQuantityError> for ParseQuantityError {
    fn from(other: MyParseQuantityError) -> Self {
        other.0
    }
}

impl Deref for MyParseQuantityError {
    type Target = ParseQuantityError;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MyParseQuantityError {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
            struct $my($uom);

            impl From<$uom> for $my {
                fn from(other: $uom) -> Self {
                    Self(other)
                }
            }

            impl From<$my> for $uom {
                fn from(other: $my) -> Self {
                    other.0
                }
            }

            impl Deref for $my {
                type Target = $uom;
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl DerefMut for $my {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }

            impl FromStr for $my {
                type Err = MyParseQuantityError;
                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    Ok(<$uom as FromStr>::from_str(s).map(From::from)?)
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
pub enum SimulationKind {
    G1(Args),
    G2(Args),
    G5(Args),
    G6(Args),
    G7(Args),
    G8(Args),
    GI(Args),
    GS(Args),
}

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(long = "time-interval", default_value = "0.00005 s")]
    time_interval: MyTime,

    #[structopt(flatten)]
    flags: Flags,

    #[structopt(flatten)]
    table: Table,

    #[structopt(flatten)]
    projectile: Projectile,

    #[structopt(flatten)]
    scope: Scope,

    #[structopt(flatten)]
    firing: Firing,

    #[structopt(flatten)]
    zeroing: Zeroing,
}
#[derive(Debug, StructOpt)]
pub struct Flags {
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
}
#[derive(Debug, StructOpt)]
pub struct Table {
    #[structopt(long = "start", default_value = "0.0 yd")]
    table_start: MyLength,

    #[structopt(long = "end", default_value = "1000.0 yd")]
    table_end: MyLength,

    #[structopt(long = "step", default_value = "100.0 yd")]
    table_step: MyLength,

    #[structopt(long = "table-tolerance", default_value = "0.005 in")]
    table_tolerance: MyLength,
}
#[derive(Debug, StructOpt)]
pub struct Projectile {
    #[structopt(long = "initial-velocity")]
    projectile_velocity: Option<MyVelocity>,

    #[structopt(long = "mass")]
    projectile_mass: Option<MyMass>,

    #[structopt(long = "caliber")]
    projectile_caliber: Option<MyLength>,

    #[structopt(long = "bc")]
    projectile_bc: Option<Numeric>,
}
#[derive(Debug, StructOpt)]
pub struct Scope {
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
pub struct Firing {
    #[structopt(flatten)]
    firing_atmosphere: FiringAtmosphere,

    #[structopt(flatten)]
    firing_wind: FiringWind,

    #[structopt(flatten)]
    firing_shooter: FiringShooter,
}
#[derive(Debug, StructOpt)]
pub struct FiringWind {
    #[structopt(long = "wind-speed")]
    firing_wind_speed: Option<MyVelocity>,

    #[structopt(long = "wind-angle")]
    firing_wind_angle: Option<MyAngle>,
}
#[derive(Debug, StructOpt)]
pub struct FiringAtmosphere {
    #[structopt(long = "temperature")]
    firing_atmosphere_temperature: Option<MyThermodynamicTemperature>,

    #[structopt(long = "pressure")]
    firing_atmosphere_pressure: Option<MyPressure>,

    #[structopt(long = "humidity")]
    firing_atmosphere_humidity: Option<Numeric>,
}
#[derive(Debug, StructOpt)]
pub struct FiringShooter {
    #[structopt(long = "lattitude")]
    firing_shooter_lattitude: Option<MyAngle>,

    #[structopt(long = "bearing")]
    firing_shooter_bearing: Option<MyAngle>,

    #[structopt(long = "shot-angle")]
    firing_shooter_angle: Option<MyAngle>,

    #[structopt(long = "gravity")]
    firing_shooter_gravity: Option<MyAcceleration>,
}
#[derive(Debug, StructOpt)]
pub struct Zeroing {
    #[structopt(flatten)]
    zeroing_wind: ZeroingWind,

    #[structopt(flatten)]
    zeroing_atmosphere: ZeroingAtmosphere,

    #[structopt(flatten)]
    zeroing_shooter: ZeroingShooter,

    #[structopt(flatten)]
    zeroing_target: ZeroingTarget,
}
#[derive(Debug, StructOpt)]
pub struct ZeroingWind {
    #[structopt(long = "zeroing-wind-speed")]
    zeroing_wind_speed: Option<MyVelocity>,

    #[structopt(long = "zeroing-wind-angle")]
    zeroing_wind_angle: Option<MyAngle>,
}
#[derive(Debug, StructOpt)]
pub struct ZeroingAtmosphere {
    #[structopt(long = "zeroing-temperature")]
    zeroing_atmosphere_temperature: Option<MyThermodynamicTemperature>,

    #[structopt(long = "zeroing-pressure")]
    zeroing_atmosphere_pressure: Option<MyPressure>,

    #[structopt(long = "zeroing-humidity")]
    zeroing_atmosphere_humidity: Option<Numeric>,
}
#[derive(Debug, StructOpt)]
pub struct ZeroingShooter {
    #[structopt(long = "zeroing-lattitude")]
    zeroing_shooter_lattitude: Option<MyAngle>,

    #[structopt(long = "zeroing-bearing")]
    zeroing_shooter_bearing: Option<MyAngle>,

    #[structopt(long = "zeroing-shot-angle")]
    zeroing_shooter_angle: Option<MyAngle>,

    #[structopt(long = "zeroing-gravity")]
    zeroing_shooter_gravity: Option<MyAcceleration>,
}
#[derive(Debug, StructOpt)]
pub struct ZeroingTarget {
    #[structopt(long = "zeroing-target-distance", default_value = "200.0 yd")]
    zeroing_target_distance: MyLength,

    #[structopt(long = "zeroing-target-height", default_value = "0.0 in")]
    zeroing_target_height: MyLength,

    #[structopt(long = "zeroing-target-offset", default_value = "0.0 in")]
    zeroing_target_offset: MyLength,

    #[structopt(long = "zeroing-target-tolerance", default_value = "0.001 in")]
    zeroing_target_tolerance: MyLength,
}

impl Args {
    pub fn time(&self) -> Time {
        *self.time_interval
    }
    pub fn flags(&self) -> &Flags {
        &self.flags
    }
    pub fn table(&self) -> &Table {
        &self.table
    }
    pub fn projectile(&self) -> &Projectile {
        &self.projectile
    }
    pub fn scope(&self) -> &Scope {
        &self.scope
    }
    pub fn firing(&self) -> &Firing {
        &self.firing
    }
    pub fn zeroing(&self) -> &Zeroing {
        &self.zeroing
    }
}
impl Flags {
    pub fn flat(&self) -> bool {
        self.flat
    }
    pub fn pretty(&self) -> bool {
        self.pretty
    }
    pub fn drag(&self) -> bool {
        !self.disable_drag
    }
    pub fn gravity(&self) -> bool {
        !self.disable_gravity
    }
    pub fn coriolis(&self) -> bool {
        !self.disable_coriolis
    }
}
impl Table {
    pub fn start(&self) -> Length {
        *self.table_start
    }
    pub fn end(&self) -> Length {
        *self.table_end
    }
    pub fn step(&self) -> Length {
        *self.table_step
    }
    pub fn tolerance(&self) -> Length {
        *self.table_tolerance
    }
}
impl Projectile {
    pub fn velocity(&self) -> Option<Velocity> {
        self.projectile_velocity.map(From::from)
    }
    pub fn mass(&self) -> Option<Mass> {
        self.projectile_mass.map(From::from)
    }
    pub fn caliber(&self) -> Option<Length> {
        self.projectile_caliber.map(From::from)
    }
    pub fn bc(&self) -> Option<Numeric> {
        self.projectile_bc
    }
}
impl Scope {
    pub fn height(&self) -> Option<Length> {
        self.scope_height.map(From::from)
    }
    pub fn offset(&self) -> Option<Length> {
        self.scope_offset.map(From::from)
    }
    pub fn pitch(&self) -> Option<Angle> {
        self.scope_pitch.map(From::from)
    }
    pub fn yaw(&self) -> Option<Angle> {
        self.scope_yaw.map(From::from)
    }
    pub fn cant(&self) -> Option<Angle> {
        self.scope_cant.map(From::from)
    }
}
impl Firing {
    pub fn atmosphere(&self) -> &FiringAtmosphere {
        &self.firing_atmosphere
    }
    pub fn wind(&self) -> &FiringWind {
        &self.firing_wind
    }
    pub fn shooter(&self) -> &FiringShooter {
        &self.firing_shooter
    }
}
impl FiringWind {
    pub fn speed(&self) -> Option<Velocity> {
        self.firing_wind_speed.map(From::from)
    }
    pub fn angle(&self) -> Option<Angle> {
        self.firing_wind_angle.map(From::from)
    }
}
impl FiringAtmosphere {
    pub fn temperature(&self) -> Option<ThermodynamicTemperature> {
        self.firing_atmosphere_temperature.map(From::from)
    }
    pub fn pressure(&self) -> Option<Pressure> {
        self.firing_atmosphere_pressure.map(From::from)
    }
    pub fn humidity(&self) -> Option<Numeric> {
        self.firing_atmosphere_humidity
    }
}
impl FiringShooter {
    pub fn lattitude(&self) -> Option<Angle> {
        self.firing_shooter_lattitude.map(From::from)
    }
    pub fn bearing(&self) -> Option<Angle> {
        self.firing_shooter_bearing.map(From::from)
    }
    pub fn angle(&self) -> Option<Angle> {
        self.firing_shooter_angle.map(From::from)
    }
    pub fn gravity(&self) -> Option<Acceleration> {
        self.firing_shooter_gravity.map(From::from)
    }
}
impl Zeroing {
    pub fn atmosphere(&self) -> &ZeroingAtmosphere {
        &self.zeroing_atmosphere
    }
    pub fn wind(&self) -> &ZeroingWind {
        &self.zeroing_wind
    }
    pub fn shooter(&self) -> &ZeroingShooter {
        &self.zeroing_shooter
    }
    pub fn target(&self) -> &ZeroingTarget {
        &self.zeroing_target
    }
}
impl ZeroingWind {
    pub fn speed(&self) -> Option<Velocity> {
        self.zeroing_wind_speed.map(From::from)
    }
    pub fn angle(&self) -> Option<Angle> {
        self.zeroing_wind_angle.map(From::from)
    }
}
impl ZeroingAtmosphere {
    pub fn temperature(&self) -> Option<ThermodynamicTemperature> {
        self.zeroing_atmosphere_temperature.map(From::from)
    }
    pub fn pressure(&self) -> Option<Pressure> {
        self.zeroing_atmosphere_pressure.map(From::from)
    }
    pub fn humidity(&self) -> Option<Numeric> {
        self.zeroing_atmosphere_humidity
    }
}
impl ZeroingShooter {
    pub fn lattitude(&self) -> Option<Angle> {
        self.zeroing_shooter_lattitude.map(From::from)
    }
    pub fn bearing(&self) -> Option<Angle> {
        self.zeroing_shooter_bearing.map(From::from)
    }
    pub fn angle(&self) -> Option<Angle> {
        self.zeroing_shooter_angle.map(From::from)
    }
    pub fn gravity(&self) -> Option<Acceleration> {
        self.zeroing_shooter_gravity.map(From::from)
    }
}
impl ZeroingTarget {
    pub fn distance(&self) -> Length {
        *self.zeroing_target_distance
    }
    pub fn height(&self) -> Length {
        *self.zeroing_target_height
    }
    pub fn offset(&self) -> Length {
        *self.zeroing_target_offset
    }
    pub fn tolerance(&self) -> Length {
        *self.zeroing_target_tolerance
    }
}
