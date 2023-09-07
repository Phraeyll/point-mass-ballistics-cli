use crate::args::{Args, Result};

use clap::Parser;

mod args;
mod formatter;

fn main() -> Result<()> {
    let cmd = Args::parse();
    cmd.run()
}
