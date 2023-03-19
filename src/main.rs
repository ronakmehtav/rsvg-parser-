use anyhow::{Context, Result};
use clap::Parser;

/// Svg Parser
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// FilePath of the svg file.
    #[arg(short, long)]
    filepath: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let data = std::fs::read_to_string(args.filepath).context("File Not found")?;

    data.lines().for_each(|line| println!("{}", line));
    Ok(())
}
