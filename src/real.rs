use crate::{
    config::{LinkerConfig, LinkerConfigImpl},
    link::{LinkCreator, LinkCreatorExt},
    linker::Linker,
    unlink::{LinkRemover, LinkRemoverExt},
    utils,
};
use std::{
    fs,
    os::unix,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct RealLinker {
    dir: PathBuf,
    target: PathBuf,
}

impl RealLinker {
    pub fn new(config: LinkerConfigImpl) -> Self {
        Self {
            dir: config.dir,
            target: config.target,
        }
    }
}

impl LinkerConfig for RealLinker {
    fn dir(&self) -> &Path {
        &self.dir
    }

    fn target(&self) -> &Path {
        &self.target
    }
}

impl LinkCreator for RealLinker {
    fn skip_create_link(&self, link: &Path) -> bool {
        if link.exists() {
            utils::print_file_exists(link);
            true
        } else {
            false
        }
    }
    fn handle_link_creation(&self, original_relative: &Path, link: &Path) {
        if let Err(e) = unix::fs::symlink(original_relative, link) {
            eprintln!("LINK ERROR: {e}");
        }
    }
}

impl LinkCreatorExt for RealLinker {}

impl Linker for RealLinker {
    fn link_package(&self, package: &str) {
        self._link(package);
    }

    fn unlink_package(&self, package: &str) {
        self._unlink(package);
    }

    fn relink_package(&self, package: &str) {
        self._unlink(package);
        self._link(package);
    }
}

impl LinkRemover for RealLinker {
    fn handle_link_removal(&self, link: &Path) {
        if let Err(e) = fs::remove_file(link) {
            eprintln!("UNLINK ERROR: {e}");
        }
    }
}

impl LinkRemoverExt for RealLinker {}
