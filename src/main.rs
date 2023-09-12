use anyhow::Result;
use clap::Parser;
use nix::dir::Dir;
use std::io::Write;
use std::process::exit;

#[derive(Parser)]
#[command(
    author,
    version,
    about="strip last component from file name.",
    long_about = None
)]
struct Cli {
    /// end each output line with NUL, not newline
    #[arg(short('z'), long)]
    zero: bool,
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    println!("Hello, world!");
    Ok(())
}
