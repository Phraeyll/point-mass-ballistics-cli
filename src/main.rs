use crate::args::{Cmd, Result};

use clap::Parser;

mod args;
mod formatter;

fn main() -> Result<()> {
    Cmd::parse().run()
}
