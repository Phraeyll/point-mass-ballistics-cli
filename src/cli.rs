use point_mass_ballistics::Numeric;
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
    drag: bool,

    #[structopt(long = "disable-coriolis")]
    coriolis: bool,

    #[structopt(long = "disable-gravity")]
    gravity: bool,

    #[structopt(long = "pretty")]
    pretty: bool,

    #[structopt(flatten)]
    projectile: Projectile,

    #[structopt(flatten)]
    scope: Scope,

    #[structopt(flatten)]
    wind: Wind,

    #[structopt(flatten)]
    shooter: Shooter,

    #[structopt(flatten)]
    zero_scope: ZeroScope,

    #[structopt(flatten)]
    zero_wind: ZeroWind,

    #[structopt(flatten)]
    zero_atmosphere: ZeroAtmosphere,

    #[structopt(flatten)]
    zero_shooter: ZeroShooter,

    #[structopt(flatten)]
    table: Table,

    #[structopt(flatten)]
    zero: Zero,

    #[structopt(long = "time-step")]
    time: Option<Numeric>,
}

#[derive(Debug, StructOpt)]
struct Projectile {
    #[structopt(long = "velocity")]
    velocity: Option<Numeric>,

    #[structopt(long = "mass")]
    mass: Option<Numeric>,

    #[structopt(long = "caliber")]
    caliber: Option<Numeric>,

    #[structopt(long = "bc")]
    bc: Option<Numeric>,

    #[structopt(long = "bc-type")]
    bc_type: Option<Numeric>,
}

#[derive(Debug, StructOpt)]
struct Scope {
    #[structopt(long = "scope-height")]
    height: Option<Numeric>,

    #[structopt(long = "scope-offset")]
    offset: Option<Numeric>,

    #[structopt(long = "scope-pitch")]
    pitch: Option<Numeric>,

    #[structopt(long = "scope-yaw")]
    yaw: Option<Numeric>,

    #[structopt(long = "scope-cant")]
    cant: Option<Numeric>,
}
#[derive(Debug, StructOpt)]
struct ZeroScope {
    #[structopt(long = "zero-scope-height")]
    height: Option<Numeric>,

    #[structopt(long = "zero-scope-offset")]
    offset: Option<Numeric>,

    #[structopt(long = "zero-scope-pitch")]
    pitch: Option<Numeric>,

    #[structopt(long = "zero-scope-yaw")]
    yaw: Option<Numeric>,

    #[structopt(long = "zero-scope-cant")]
    cant: Option<Numeric>,
}

#[derive(Debug, StructOpt)]
struct Wind {
    #[structopt(long = "wind-speed")]
    speed: Option<Numeric>,

    #[structopt(long = "wind-angle")]
    angle: Option<Numeric>,
}
#[derive(Debug, StructOpt)]
struct ZeroWind {
    #[structopt(long = "zero-wind-speed")]
    speed: Option<Numeric>,

    #[structopt(long = "zero-wind-angle")]
    angle: Option<Numeric>,
}

#[derive(Debug, StructOpt)]
struct Atmosphere {
    #[structopt(long = "temperature")]
    temperature: Option<Numeric>,

    #[structopt(long = "pressure")]
    pressure: Option<Numeric>,

    #[structopt(long = "humidity")]
    humidity: Option<Numeric>,
}
#[derive(Debug, StructOpt)]
struct ZeroAtmosphere {
    #[structopt(long = "zero-temperature")]
    temperature: Option<Numeric>,

    #[structopt(long = "zero-pressure")]
    pressure: Option<Numeric>,

    #[structopt(long = "zero-humidity")]
    humidity: Option<Numeric>,
}

#[derive(Debug, StructOpt)]
struct Shooter {
    #[structopt(long = "lattitude")]
    lattitude: Option<Numeric>,

    #[structopt(long = "bearing")]
    bearing: Option<Numeric>,

    #[structopt(long = "shot-angle")]
    angle: Option<Numeric>,

    #[structopt(long = "gravity")]
    gravity: Option<Numeric>,
}

#[derive(Debug, StructOpt)]
struct ZeroShooter {
    #[structopt(long = "zero-lattitude")]
    lattitude: Option<Numeric>,

    #[structopt(long = "zero-bearing")]
    bearing: Option<Numeric>,

    #[structopt(long = "zero-shot-angle")]
    angle: Option<Numeric>,

    #[structopt(long = "zero-gravity")]
    gravity: Option<Numeric>,
}

#[derive(Debug, StructOpt)]
struct Table {
    #[structopt(long = "start")]
    start: Option<Numeric>,

    #[structopt(long = "end")]
    end: Option<Numeric>,

    #[structopt(long = "step")]
    step: Option<Numeric>,

    #[structopt(long = "table-tolerance")]
    tolerance: Option<Numeric>,
}

#[derive(Debug, StructOpt)]
struct Zero {
    #[structopt(long = "zero-distance")]
    distance: Option<Numeric>,

    #[structopt(long = "zero-tolerance")]
    tolerance: Option<Numeric>,
}
