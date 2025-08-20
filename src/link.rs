use crate::{config::LinkerConfig, utils};
use std::path::Path;

pub trait LinkCreator: LinkerConfig {
    fn skip_create_link(&self, link: &Path) -> bool;

    fn handle_link_creation(&self, _original_relative: &Path, _link: &Path) {}
}

pub trait LinkCreatorExt: LinkCreator + Sized {
    fn _link(&self, package: &str) {
        let package_path = self.dir().join(package);

        if !package_path.exists() {
            eprintln!("Package '{}' not found", package);
            return;
        }

        self.link_tree(&package_path, self.target());
    }

    fn link_tree(&self, original: &Path, link: &Path) {
        if !link.exists() || original.is_file() {
            common_create_symlink(self, original, link);
            return;
        }

        utils::walkdir(original, link, |orig, lnk| self.link_tree(orig, lnk));
    }
}

pub fn common_create_symlink(creator: &impl LinkCreator, original: &Path, link: &Path) {
    if creator.skip_create_link(link) {
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

    println!(
        "LINK: {} => {}",
        link.display(),
        original_relative.display()
    );

    creator.handle_link_creation(&original_relative, link);
}
