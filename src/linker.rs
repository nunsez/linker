use crate::utils;
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
        self._link(package);
        self.simulate_warning();
    }

    pub fn link_packages(&self) {
        for package in utils::package_list(&self.dir) {
            self.link_package(&package);
        }
    }

    pub fn unlink_package(&self, package: &str) {
        self._unlink(package);
        self.simulate_warning();
    }

    pub fn unlink_packages(&self) {
        for package in utils::package_list(&self.dir) {
            self.unlink_package(&package);
        }
    }

    pub fn relink_package(&self, package: &str) {
        self._relink(package);
        self.simulate_warning();
    }

    pub fn relink_packages(&self) {
        for package in utils::package_list(&self.dir) {
            self.relink_package(&package);
        }
    }

    fn _link(&self, package: &str) {
        let package_path = self.dir.join(package);

        if !package_path.exists() {
            eprintln!("Package '{}' not found", package);
            return;
        }

        utils::link_tree(&package_path, &self.target, self.simulate);
    }

    fn _unlink(&self, package: &str) {
        let package_path = &self.dir.join(package);
        utils::unlink_tree(package_path, &self.target, self.simulate);
    }

    fn _relink(&self, package: &str) {
        self._unlink(package);
        self._link(package);
    }

    fn simulate_warning(&self) {
        if self.simulate {
            eprintln!("WARNING: in simulation mode so not modifying filesystem.");
        }
    }
}
