use crate::{config::LinkerConfig, utils};

pub trait Linker: LinkerConfig {
    fn link_package(&self, package: &str);

    fn unlink_package(&self, package: &str);

    fn relink_package(&self, package: &str);

    fn link_packages(&self) {
        for package in utils::package_list(self.dir()) {
            self.link_package(&package);
        }
    }

    fn unlink_packages(&self) {
        for package in utils::package_list(self.dir()) {
            self.unlink_package(&package);
        }
    }

    fn relink_packages(&self) {
        for package in utils::package_list(self.dir()) {
            self.relink_package(&package);
        }
    }
}
