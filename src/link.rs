use crate::utils;
use pathdiff;
use std::{env, os::unix, path::Path};

pub fn link_tree(original: &Path, link: &Path, simulate: bool) {
    if !link.exists() || original.is_file() {
        create_symlink(original, link, simulate);
        return;
    }

    utils::traverse(original, link, |orig, lnk| link_tree(orig, lnk, simulate));
}

fn create_symlink(original: &Path, link: &Path, simulate: bool) {
    if link.exists() {
        eprintln!("File exists and will not be symlinked: {}", link.display());
        return;
    }

    let Some(link_parent) = link.parent() else {
        eprintln!("Failed to get parent directory for {}", link.display());
        return;
    };

    // for relative path exists check
    if let Err(e) = env::set_current_dir(link_parent) {
        eprintln!("{e}");
        return;
    }

    let original_relative = pathdiff::diff_paths(original, link_parent)
        .filter(|p| p.exists())
        .unwrap_or_else(|| original.to_path_buf());

    println!(
        "LINK: {} => {}",
        link.display(),
        original_relative.display()
    );

    if simulate {
        return;
    };

    if let Err(e) = unix::fs::symlink(original_relative, link) {
        eprintln!("LINK ERROR: {e}");
    }
}
