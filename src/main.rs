use anyhow::Result;
use clap::Parser;
use std::io::Write;
use std::path::Path;
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

    // The file path to read
    name: Vec<std::path::PathBuf>,
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    let dir = args.name;
    if dir.is_empty() {
        writeln!(std::io::stderr(), "dirname: missing operand")?;
        exit(1);
    }
    for path in dir {
        let parent_dir = Path::new(&path).parent();
        match parent_dir {
            Some(directory) => {
                if directory.to_str() == Some("") {
                    if args.zero {
                        write!(std::io::stdout(), ".")?;
                    } else {
                        writeln!(std::io::stdout(), ".")?;
                    }
                } else {
                    if args.zero {
                        write!(std::io::stdout(), "{}", directory.display())?;
                    } else {
                        writeln!(std::io::stdout(), "{}", directory.display())?;
                    }
                }
            }
            None => {}
        }
    }

    Ok(())
}
