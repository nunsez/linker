use clap::{Parser, ValueEnum};

use std::{env::current_dir, path::PathBuf};

#[derive(Debug, Parser)]
#[command(version = "0.1.0")]
#[command(about = "Linker: A lightweight GNU Stow alternative")]
pub struct Cli {
    /// Execution mode
    #[arg(value_enum, required = true)]
    pub mode: Mode,

    /// Set linker dir to DIR (default is current dir)
    #[arg(short, long)]
    dir: Option<PathBuf>,

    /// Set target to TARGET (default is parent of linker dir)
    #[arg(short, long)]
    target: Option<PathBuf>,

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

impl Cli {
    pub fn dir(&self) -> PathBuf {
        self.dir
            .as_ref()
            .cloned()
            .or_else(|| current_dir().ok())
            .expect("Failed to get DIR")
    }

    pub fn target(&self) -> PathBuf {
        self.target
            .as_ref()
            .cloned()
            .or_else(|| Some(self.dir().parent()?.to_path_buf()))
            .expect("Failed to get TARGET")
    }

    pub fn parsed() -> Self {
        Cli::parse()
    }
}
