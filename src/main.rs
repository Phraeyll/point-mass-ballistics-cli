use self::args::Args;

use args::Result;

use clap::Parser;

mod args;
mod printer;

fn main() -> Result<()> {
    let cmd = Args::parse();
    cmd.run()?;
    Ok(())
}
