use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, disable_version_flag = true)]
struct Cli {
    /// Print version and use lowercase V for short version as it is more common
    #[arg(short = 'v', long, action = clap::builder::ArgAction::Version)]
    version: (),
}

pub fn start() -> Result<()> {
    // NOTE: TBD
    let _ = Cli::parse();

    Ok(())
}
