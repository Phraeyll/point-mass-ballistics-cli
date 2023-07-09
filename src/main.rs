use crate::args::{Args, Result};

use clap::Parser;

mod args;
mod printer;

fn main() -> Result<()> {
    let cmd = Args::parse();
    cmd.run()?;
    Ok(())
}
