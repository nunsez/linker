use crate::utils;
use std::{
    cell::RefCell,
    collections::HashSet,
    env, fs,
    os::unix,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct Linker {
    dir: PathBuf,
    target: PathBuf,
    simulate: bool,
    removed: RefCell<HashSet<PathBuf>>,
}

impl Linker {
    pub fn build(
        dir: &Option<PathBuf>,
        target: &Option<PathBuf>,
        simulate: bool,
    ) -> Result<Self, String> {
        let dir = match dir {
            Some(dir) => fs::canonicalize(dir),
            None => env::current_dir(),
        };
        let dir = dir.map_err(|_| "Failed to get DIR")?;

        let target = match target {
            Some(dir) => fs::canonicalize(dir).ok(),
            None => dir.parent().map(|d| d.to_path_buf()),
        };
        let target = target.ok_or("Failed to get TARGET")?;

        let linker = Self {
            dir,
            target,
            simulate,
            removed: RefCell::new(HashSet::new()),
        };

        Ok(linker)
    }

    pub fn link(&self, packages: &[String]) {
        for package in packages {
            self._link(package);
        }
        self.simulate_warning();
    }

    pub fn unlink(&self, packages: &[String]) {
        for package in packages {
            self._unlink(package);
        }
        self.simulate_warning();
    }

    pub fn relink(&self, packages: &[String]) {
        for package in packages {
            self._unlink(package);
            self._link(package);
            self.simulate_warning();
        }
    }

    fn _link(&self, package: &str) {
        let package_path = self.dir.join(package);

        if !package_path.exists() {
            eprintln!("Package '{}' not found", package);
            return;
        }

        self.link_tree(&package_path, &self.target);
    }

    fn _unlink(&self, package: &str) {
        let package_path = self.dir.join(package);
        self.unlink_tree(&package_path, &self.target);
    }

    fn link_tree(&self, original: &Path, link: &Path) {
        if !link.exists() || original.is_file() {
            self.create_symlink(original, link);
            return;
        }

        utils::walkdir(original, link, |orig, lnk| self.link_tree(orig, lnk));
    }

    fn unlink_tree(&self, original: &Path, link: &Path) {
        if link.is_symlink() {
            self.remove_symlink(original, link);
            return;
        }

        utils::walkdir(original, link, |orig, lnk| self.unlink_tree(orig, lnk));
    }

    fn create_symlink(&self, original: &Path, link: &Path) {
        let removed = self.removed.borrow().contains(link);

        let should_skip = if self.simulate {
            link.exists() && !removed
        } else {
            link.exists()
        };

        if should_skip {
            eprintln!("File exists and will not be symlinked: {}", link.display());
            return;
        }

        let Some(link_parent) = link.parent() else {
            eprintln!("Failed to get parent directory for {}", link.display());
            return;
        };

        let original_relative = match pathdiff::diff_paths(original, link_parent) {
            Some(path_buff) => path_buff,
            None => original.to_path_buf(),
        };

        let extra_message = if self.simulate && removed {
            " (reverts previous action)"
        } else {
            ""
        };

        println!(
            "LINK: {} => {}{}",
            link.display(),
            original_relative.display(),
            extra_message
        );

        if self.simulate {
            return;
        }

        if let Err(e) = unix::fs::symlink(original_relative, link) {
            eprintln!("LINK ERROR: {e}");
        }
    }

    fn remove_symlink(&self, original: &Path, link: &Path) {
        if !link.exists() || !link.is_symlink() {
            return;
        }

        let Ok(link_target) = fs::read_link(link) else {
            return;
        };

        let Some(link_parent) = link.parent() else {
            eprintln!("Failed to get parent directory for {}", link.display());
            return;
        };

        let link_target = utils::absolute(&link_target, link_parent);

        if link_target != original {
            return;
        }

        println!("UNLINK: {}", link.display());

        if self.simulate {
            let mut removed = self.removed.borrow_mut();
            removed.insert(link.to_path_buf());
            return;
        }

        if let Err(e) = fs::remove_file(link) {
            eprintln!("UNLINK ERROR: {e}");
        }
    }

    fn simulate_warning(&self) {
        if self.simulate {
            eprintln!("WARNING: in simulation mode so not modifying filesystem.");
        }
    }
}
