use crate::{link, unlink};
use std::{env, fs, path::PathBuf};

#[derive(Debug)]
pub struct Linker {
    dir: PathBuf,
    target: PathBuf,
    simulate: bool,
}

impl Linker {
    pub fn new(dir: &Option<PathBuf>, target: &Option<PathBuf>, simulate: bool) -> Self {
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

        Self {
            dir,
            target,
            simulate,
        }
    }

    pub fn link_package(&self, package: &str) {
        link::link_package(&self.dir, &self.target, package, self.simulate);
    }

    pub fn link_packages(&self) {
        link::link_packages(&self.dir, &self.target, self.simulate);
    }

    pub fn unlink_package(&self, package: &str) {
        unlink::unlink_package(&self.dir, &self.target, package, self.simulate);
    }

    pub fn unlink_packages(&self) {
        unlink::unlink_packages(&self.dir, &self.target, self.simulate);
    }

    pub fn relink_package(&self, package: &str) {
        unlink::unlink_package(&self.dir, &self.target, package, self.simulate);
        link::link_package(&self.dir, &self.target, package, self.simulate);
    }

    pub fn relink_packages(&self) {
        unlink::unlink_packages(&self.dir, &self.target, self.simulate);
        link::link_packages(&self.dir, &self.target, self.simulate);
    }
}
