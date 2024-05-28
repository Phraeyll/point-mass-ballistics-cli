use crate::{cmd::Cmd, error::Result};

use clap::Parser;

mod cmd;
mod error;
mod formatter;

fn main() -> Result<()> {
    Cmd::parse().run()
}
