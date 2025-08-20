use crate::{
    config::{LinkerConfig, LinkerConfigImpl},
    link::{LinkCreator, LinkCreatorExt},
    linker::Linker,
    unlink::{LinkRemover, LinkRemoverExt},
    utils,
};
use std::{
    cell::RefCell,
    collections::HashSet,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct SimulateLinker {
    dir: PathBuf,
    target: PathBuf,
    removed: RefCell<HashSet<PathBuf>>,
}

impl SimulateLinker {
    pub fn new(config: LinkerConfigImpl) -> Self {
        Self {
            dir: config.dir,
            target: config.target,
            removed: RefCell::new(HashSet::new()),
        }
    }
}

impl LinkerConfig for SimulateLinker {
    fn dir(&self) -> &Path {
        &self.dir
    }

    fn target(&self) -> &Path {
        &self.target
    }
}

impl LinkCreator for SimulateLinker {
    fn skip_create_link(&self, link: &Path) -> bool {
        let removed = self.removed.borrow();

        if !removed.contains(link) && link.exists() {
            utils::print_file_exists(link);
            true
        } else {
            false
        }
    }
}

impl LinkCreatorExt for SimulateLinker {}

impl LinkRemover for SimulateLinker {
    fn handle_link_removal(&self, link: &Path) {
        let mut removed = self.removed.borrow_mut();
        removed.insert(link.to_path_buf());
    }
}

impl LinkRemoverExt for SimulateLinker {}

impl Linker for SimulateLinker {
    fn link_package(&self, package: &str) {
        self._link(package);
        simulate_warning();
    }

    fn unlink_package(&self, package: &str) {
        self._unlink(package);
        simulate_warning();
    }

    fn relink_package(&self, package: &str) {
        self._unlink(package);
        self._link(package);
        simulate_warning();
    }
}

fn simulate_warning() {
    eprintln!("WARNING: in simulation mode so not modifying filesystem.");
}
