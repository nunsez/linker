use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub trait LinkerConfig {
    fn dir(&self) -> &Path;
    fn target(&self) -> &Path;
}

#[derive(Debug)]
pub struct LinkerConfigImpl {
    pub dir: PathBuf,
    pub target: PathBuf,
}

impl LinkerConfigImpl {
    pub fn new(dir: &Option<PathBuf>, target: &Option<PathBuf>) -> Self {
        let dir = match dir {
            Some(dir) => fs::canonicalize(dir),
            None => env::current_dir(),
        };
        let dir = dir.expect("Failed to get DIR");

        let target = match target {
            Some(dir) => fs::canonicalize(dir).ok(),
            None => dir.parent().map(|d| d.to_path_buf()),
        };
        let target = target.expect("Failed to get TARGET");

        Self { dir, target }
    }
}
