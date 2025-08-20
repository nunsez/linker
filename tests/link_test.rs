mod common;

use common::{
    absolute, build_real_linker, build_simulate_linker, ensure_exist, fixture_path, touch,
};
use linker::Linker;
use std::{fs, path::Path};
use tempfile::tempdir;

#[test]
fn simulate_is_true() {
    let tempdir = tempdir().unwrap();
    let target = tempdir.path();
    let linker = build_simulate_linker(target);

    linker.link_package("fish");

    let config = target.join(".config");
    assert!(!config.exists());
}

#[test]
fn link_config() {
    let tempdir = tempdir().unwrap();
    let target = tempdir.path();
    let linker = build_real_linker(target);

    dbg!(&linker);

    linker.link_package("fish");

    assert_associated_symlink("fish", target, ".config");
}

#[test]
fn link_fish() {
    let tempdir = tempdir().unwrap();
    let target = tempdir.path();
    let linker = build_real_linker(target);

    ensure_exist(target.join(".config"));

    linker.link_package("fish");

    assert_associated_symlink("fish", target, ".config/fish");
}

#[test]
fn link_full() {
    let tempdir = tempdir().unwrap();
    let target = tempdir.path();
    let linker = build_real_linker(target);

    let functions = target.join(".config/fish/functions");
    let l = functions.join("l.fish");

    touch(&l);

    linker.link_package("fish");

    assert!(!functions.is_symlink());
    assert!(functions.is_dir());

    assert!(l.exists());
    assert!(!l.is_symlink());

    assert_associated_symlink("fish", target, ".config/fish/functions/ls.fish");
    assert_associated_symlink("fish", target, ".config/fish/conf.d");
    assert_associated_symlink("fish", target, ".config/fish/config.fish");
}

#[test]
fn link_packages() {
    let tempdir = tempdir().unwrap();
    let target = tempdir.path();
    let linker = build_real_linker(target);

    ensure_exist(target.join(".config"));

    linker.link_packages();

    assert_associated_symlink("fish", target, ".config/fish");
    assert_associated_symlink("git", target, ".gitconfig");
}

fn assert_associated_symlink(package: &str, target: &Path, path: &str) {
    let link = target.join(path);

    assert!(link.is_symlink());

    let link_base = link.parent().unwrap();
    let link_relative = fs::read_link(&link).unwrap();
    let link_absolute = absolute(&link_relative, link_base);
    let src = fixture_path(package).join(path);

    assert_eq!(link_absolute, src);
}
