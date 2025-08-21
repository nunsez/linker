use clap::{Parser, ValueEnum};
use linker::Linker;
use std::path::PathBuf;

fn main() {
    let cli = Cli::parse();

    let linker = Linker::build(&cli.dir, &cli.target, cli.simulate).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });

    match cli.mode {
        Mode::Link => linker.link(&cli.packages),
        Mode::Unlink => linker.unlink(&cli.packages),
        Mode::Relink => linker.relink(&cli.packages),
    }
}

#[derive(Debug, Parser)]
#[command(version = "0.1.0")]
#[command(about = "Linker: A lightweight GNU Stow alternative")]
pub struct Cli {
    /// Execution mode
    #[arg(value_enum, required = true)]
    pub mode: Mode,

    /// Set linker dir to DIR (default is current dir)
    #[arg(short, long)]
    pub dir: Option<PathBuf>,

    /// Set target to TARGET (default is parent of linker dir)
    #[arg(short, long)]
    pub target: Option<PathBuf>,

    /// Do not actually make any filesystem changes
    #[arg(short = 'n', long = "simulate")]
    pub simulate: bool,

    /// Specific packages to link
    #[arg(value_name = "PACKAGE")]
    pub packages: Vec<String>,
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Mode {
    /// Link packages
    Link,
    /// Unlink packages
    Unlink,
    /// Relink packages (like `unlink` followed by `link`)
    Relink,
}
