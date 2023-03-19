use clap::Parser;

/// Svg Parser
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// FilePath of the svg file.
    #[arg(short, long)]
    filepath: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();
    println!("Hello, {:?}", args.filepath);
}
