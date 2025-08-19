mod common;

use common::{build_linker, ensure_exist, fixture_path, touch};
use std::os::unix;
use tempfile::tempdir;

#[test]
fn simulate_is_true() {
    let tempdir = tempdir().unwrap();
    let target = tempdir.path();
    let linker = build_linker(target, true);

    ensure_exist(target.join(".config"));
    // create link (simulate false)
    build_linker(target, false).link_package("fish");

    let fish_config = target.join(".config/fish");

    assert!(fish_config.is_symlink());

    linker.unlink_package("fish");

    assert!(fish_config.exists());
}

#[test]
fn unlink_config() {
    let tempdir = tempdir().unwrap();
    let target = tempdir.path();
    let linker = build_linker(target, false);

    linker.link_package("fish");
    linker.unlink_package("fish");

    assert!(!target.join(".config").exists());
}

#[test]
fn unlink_fish() {
    let tempdir = tempdir().unwrap();
    let target = tempdir.path();
    let linker = build_linker(target, false);

    ensure_exist(target.join(".config"));

    linker.link_package("fish");
    linker.unlink_package("fish");

    assert!(!target.join(".config/fish").exists());
}

#[test]
fn unlink_full() {
    let tempdir = tempdir().unwrap();
    let target = tempdir.path();
    let linker = build_linker(target, false);

    let l = target.join(".config/fish/functions/l.fish");
    touch(&l);

    linker.link_package("fish");
    linker.unlink_package("fish");

    assert!(l.is_file());
    assert!(!target.join(".config/fish/conf.d").exists());
    assert!(!target.join(".config/fish/functions/ls.fish").exists());
    assert!(!target.join(".config/fish/config.fish").exists());
}

#[test]
fn ignore_another_symlink() {
    let tempdir = tempdir().unwrap();
    let target = tempdir.path();
    let linker = build_linker(target, false);

    ensure_exist(target.join(".config/fish"));

    let fish_config = target.join(".config/fish/config.fish");
    let git_config = fixture_path("git/.gitconfig");
    unix::fs::symlink(git_config, &fish_config).unwrap();

    linker.unlink_package("fish");

    assert!(fish_config.is_symlink());
}

#[test]
fn unlink_packages() {
    let tempdir = tempdir().unwrap();
    let target = tempdir.path();
    let linker = build_linker(target, false);

    ensure_exist(target.join(".config"));

    linker.link_packages();
    linker.unlink_packages();

    assert!(!target.join(".config/fish").is_symlink());
    assert!(!target.join(".config/git").is_symlink());
}
