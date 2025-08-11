use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::utils::{package_list, traverse};

pub struct Unlink {
    dir: PathBuf,
    target: PathBuf,
    simulate: bool,
}

impl Unlink {
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
        self.traverse(&package_path, &self.target);
    }

    fn traverse(&self, source_path: &Path, destination_path: &Path) {
        if destination_path.is_symlink() {
            self.remove_symlink(source_path, destination_path);
            return;
        }

        traverse(source_path, destination_path, |s, d| self.traverse(s, d));
    }

    fn remove_symlink(&self, original: &Path, link: &Path) {
        if !link.exists() || !link.is_symlink() {
            return;
        }

        let Ok(link_target) = fs::read_link(link) else {
            return;
        };

        let Some(link_parent) = link.parent() else {
            println!("Failed to get parent directory for {}", link.display());
            return;
        };

        if let Err(e) = env::set_current_dir(link_parent) {
            println!("{e}");
            return;
        }

        match fs::canonicalize(link_target) {
            Ok(link_target) => {
                if link_target != original {
                    return;
                }
            }
            Err(e) => {
                println!("{e}");
                return;
            }
        }

        println!("UNLINK: {}", link.display());

        if self.simulate {
            return;
        }

        if let Err(e) = fs::remove_file(link) {
            println!("UNLINK ERROR: {e}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{FIXTURES_DIR, ensure_exist, touch};
    use std::os::unix::fs::symlink as fs_symlink;
    use tempfile::tempdir;

    #[test]
    fn simulate_is_true() {
        let tempdir = tempdir().unwrap();
        let target = tempdir.path();
        let cmd = init_cmd(target, true);

        ensure_exist(target.join(".config"));

        symlink("fish", target, ".config/fish");

        assert!(target.join(".config/fish").is_symlink());

        cmd.handle_package("fish");

        assert!(target.join(".config/fish").exists());
    }

    #[test]
    fn unlink_config() {
        let tempdir = tempdir().unwrap();
        let target = tempdir.path();
        let cmd = init_cmd(target, false);

        let config = target.join(".config");

        symlink("fish", target, ".config");

        assert!(target.join(".config").is_symlink());

        cmd.handle_package("fish");

        assert!(!config.exists());
    }

    #[test]
    fn unlink_fish() {
        let tempdir = tempdir().unwrap();
        let target = tempdir.path();
        let cmd = init_cmd(target, false);

        ensure_exist(target.join(".config"));

        symlink("fish", target, ".config/fish");

        assert!(target.join(".config/fish").is_symlink());

        cmd.handle_package("fish");

        assert!(!target.join(".config/fish").exists());
    }

    #[test]
    fn unlink_full() {
        let tempdir = tempdir().unwrap();
        let target = tempdir.path();
        let cmd = init_cmd(target, false);

        let l = target.join(".config/fish/functions/l.fish");
        touch(&l);

        symlink("fish", target, ".config/fish/conf.d");
        symlink("fish", target, ".config/fish/functions/ls.fish");
        symlink("fish", target, ".config/fish/config.fish");

        assert!(target.join(".config/fish/conf.d").is_symlink());
        assert!(target.join(".config/fish/functions/ls.fish").is_symlink());
        assert!(target.join(".config/fish/config.fish").is_symlink());

        cmd.handle_package("fish");

        assert!(l.is_file());
        assert!(!target.join(".config/fish/conf.d").exists());
        assert!(!target.join(".config/fish/functions/ls.fish").exists());
        assert!(!target.join(".config/fish/config.fish").exists());
    }

    #[test]
    fn ignore_another_symlink() {
        let tempdir = tempdir().unwrap();
        let target = tempdir.path();
        let cmd = init_cmd(target, false);

        ensure_exist(target.join(".config/fish"));

        let config = target.join(".config/fish/config.fish");
        fs_symlink(PathBuf::from(FIXTURES_DIR).join("git/.gitconfig"), &config).unwrap();

        cmd.handle_package("fish");

        assert!(config.is_symlink());
    }

    #[test]
    fn handle_packages() {
        let tempdir = tempdir().unwrap();
        let target = tempdir.path();
        let cmd = init_cmd(target, false);

        ensure_exist(target.join(".config"));

        symlink("fish", target, ".config/fish");
        symlink("git", target, ".gitconfig");

        assert!(target.join(".config/fish").is_symlink());
        assert!(target.join(".gitconfig").is_symlink());

        cmd.handle_packages();

        assert!(!target.join(".config/fish").exists());
        assert!(!target.join(".config/git").exists());
    }

    fn init_cmd(target: &Path, simulate: bool) -> Unlink {
        Unlink::new(PathBuf::from(FIXTURES_DIR), target.to_path_buf(), simulate)
    }

    fn symlink(package: &str, target: &Path, path: &str) {
        fs_symlink(
            PathBuf::from(FIXTURES_DIR).join(package).join(path),
            target.join(path),
        )
        .unwrap();
    }
}
