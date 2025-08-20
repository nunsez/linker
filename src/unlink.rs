use crate::{config::LinkerConfig, utils};
use std::{fs, path::Path};

pub trait LinkRemover: LinkerConfig {
    fn handle_link_removal(&self, _link: &Path) {}
}

pub trait LinkRemoverExt: LinkRemover + Sized {
    fn _unlink(&self, package: &str) {
        let package_path = self.dir().join(package);
        self.unlink_tree(&package_path, self.target());
    }

    fn unlink_tree(&self, original: &Path, link: &Path) {
        if link.is_symlink() {
            common_remove_symlink(self, original, link);
            return;
        }

        utils::walkdir(original, link, |orig, lnk| self.unlink_tree(orig, lnk));
    }
}

fn common_remove_symlink(remover: &impl LinkRemover, original: &Path, link: &Path) {
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

    remover.handle_link_removal(link);
}
