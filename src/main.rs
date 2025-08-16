use clap::{Parser, ValueEnum};
use linker::Linker;
use std::path::PathBuf;

fn main() {
    let cli = Cli::parse();
    let linker = Linker::new(&cli.dir, &cli.target, cli.simulate);

    match (cli.mode, cli.package) {
        (Mode::Link, Some(package)) => linker.link_package(&package),
        (Mode::Link, None) => linker.link_packages(),
        (Mode::Unlink, Some(package)) => linker.unlink_package(&package),
        (Mode::Unlink, None) => linker.unlink_packages(),
        (Mode::Relink, Some(package)) => linker.relink_package(&package),
        (Mode::Relink, None) => linker.relink_packages(),
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

    /// Specific package to link (optional)
    #[arg(value_name = "PACKAGE")]
    pub package: Option<String>,
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
