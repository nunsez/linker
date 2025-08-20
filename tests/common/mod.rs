use linker::{LinkerConfigImpl, RealLinker, SimulateLinker};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub const FIXTURES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures");

pub fn fixture_path(path: &str) -> PathBuf {
    PathBuf::from(FIXTURES_DIR).join(path)
}

pub fn touch(path: &Path) {
    let parent = path.parent().unwrap();
    ensure_exist(parent);
    fs::write(path, "").unwrap();
}

pub fn ensure_exist<P>(path: P)
where
    P: AsRef<Path>,
{
    fs::create_dir_all(path).unwrap();
}

pub fn build_real_linker(target: &Path) -> RealLinker {
    let config = LinkerConfigImpl::new(
        &Some(PathBuf::from(FIXTURES_DIR)),
        &Some(target.to_path_buf()),
    );
    RealLinker::new(config)
}

pub fn build_simulate_linker(target: &Path) -> SimulateLinker {
    let config = LinkerConfigImpl::new(
        &Some(PathBuf::from(FIXTURES_DIR)),
        &Some(target.to_path_buf()),
    );
    SimulateLinker::new(config)
}

#[allow(dead_code)]
pub fn absolute(relative: &Path, base: &Path) -> PathBuf {
    let abs = base.join(relative);
    path_clean::clean(abs)
}
