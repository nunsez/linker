use crate::utils;
use std::{fs, path::Path};

pub fn unlink_tree(original: &Path, link: &Path, simulate: bool) {
    if link.is_symlink() {
        remove_symlink(original, link, simulate);
        return;
    }

    utils::walkdir(original, link, |orig, lnk| unlink_tree(orig, lnk, simulate));
}

fn remove_symlink(original: &Path, link: &Path, simulate: bool) {
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

    if simulate {
        return;
    }

    if let Err(e) = fs::remove_file(link) {
        eprintln!("UNLINK ERROR: {e}");
    }
}
