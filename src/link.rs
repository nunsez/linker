use crate::utils;
use pathdiff;
use std::{env, os::unix, path::Path};

pub fn link_packages(original: &Path, link: &Path, simulate: bool) {
    for package in utils::package_list(original) {
        link_package(original, link, &package, simulate)
    }
}

pub fn link_package(original: &Path, link: &Path, package: &str, simulate: bool) {
    let package_path = original.join(package);

    if !package_path.exists() {
        eprintln!("Package '{}' not found", package);
        return;
    }

    link_traverse(&package_path, link, simulate);
}

fn link_traverse(original: &Path, link: &Path, simulate: bool) {
    if !link.exists() || original.is_file() {
        create_symlink(original, link, simulate);
        return;
    }

    utils::traverse(original, link, |orig, lnk| {
        link_traverse(orig, lnk, simulate)
    });
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
