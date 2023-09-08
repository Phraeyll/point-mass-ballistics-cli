use crate::cmd::{Cmd, Result};

use clap::Parser;

mod cmd;
mod formatter;

fn main() -> Result<()> {
    Cmd::parse().run()
}
