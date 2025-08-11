use std::{
    os::unix::fs::symlink,
    path::{Path, PathBuf},
};

use crate::utils::{package_list, traverse};

#[derive(Debug)]
pub struct Link {
    dir: PathBuf,
    target: PathBuf,
    simulate: bool,
}

impl Link {
    pub fn new(dir: PathBuf, target: PathBuf, simulate: bool) -> Self {
        Self {
            dir,
            target,
            simulate,
        }
    }

    pub fn handle_packages(&self) {
        for package in package_list(&self.dir) {
            self.handle_package(&package)
        }
    }

    pub fn handle_package(&self, package: &str) {
        let package_path = self.dir.join(package);

        if !package_path.exists() {
            println!("WARNING: Package '{}' not found", package);
            return;
        }

        self.traverse(&package_path, &self.target);
    }

    fn traverse(&self, source_path: &Path, destination_path: &Path) {
        if source_path.is_file() || !destination_path.exists() {
            self.create_symlink(source_path, destination_path);
            return;
        }

        traverse(source_path, destination_path, |s, d| self.traverse(s, d));
    }

    fn create_symlink(&self, src: &Path, dst: &Path) {
        if dst.exists() {
            let warning = "WARNING: File exists and will not be symlinked";
            println!("{}: {}", warning, dst.display());
            return;
        }

        let message = format!("LINK: {} => {}", dst.display(), src.display());

        if self.simulate {
            println!("[SIMULATE] {message}");
            return;
        };

        println!("{message}");

        if let Err(e) = symlink(src, dst) {
            println!("LINK ERROR: {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{FIXTURES_DIR, ensure_exist, touch};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn simulate_is_true() {
        let tempdir = tempdir().unwrap();
        let target = tempdir.path();
        let cmd = init_cmd(target, true);

        cmd.handle_package("fish");

        let config = target.join(".config");
        assert!(!config.exists());
    }

    #[test]
    fn link_config() {
        let tempdir = tempdir().unwrap();
        let target = tempdir.path();
        let cmd = init_cmd(target, false);

        cmd.handle_package("fish");

        assert_associated_symlink("fish", target, ".config");
    }

    #[test]
    fn link_fish() {
        let tempdir = tempdir().unwrap();
        let target = tempdir.path();
        let cmd = init_cmd(target, false);

        ensure_exist(target.join(".config"));

        cmd.handle_package("fish");

        assert_associated_symlink("fish", target, ".config/fish");
    }

    #[test]
    fn link_full() {
        let tempdir = tempdir().unwrap();
        let target = tempdir.path();
        let cmd = init_cmd(target, false);

        let functions = target.join(".config/fish/functions");
        let l = functions.join("l.fish");

        touch(&l);

        cmd.handle_package("fish");

        assert!(!functions.is_symlink());
        assert!(functions.is_dir());

        assert!(l.exists());
        assert!(!l.is_symlink());

        assert_associated_symlink("fish", target, ".config/fish/functions/ls.fish");
        assert_associated_symlink("fish", target, ".config/fish/conf.d");
        assert_associated_symlink("fish", target, ".config/fish/config.fish");
    }

    #[test]
    fn handle_packages() {
        let tempdir = tempdir().unwrap();
        let target = tempdir.path();
        let cmd = init_cmd(target, false);

        ensure_exist(target.join(".config"));

        cmd.handle_packages();

        assert_associated_symlink("fish", target, ".config/fish");
        assert_associated_symlink("git", target, ".gitconfig");
    }

    fn assert_associated_symlink(package: &str, target: &Path, path: &str) {
        dbg!(path);

        let link = target.join(path);

        assert!(link.is_symlink());

        assert_eq!(
            fs::read_link(link).unwrap(),
            PathBuf::from(FIXTURES_DIR).join(package).join(path)
        );
    }

    fn init_cmd(target: &Path, simulate: bool) -> Link {
        Link::new(PathBuf::from(FIXTURES_DIR), target.to_path_buf(), simulate)
    }
}
